use std::fs;

use ataxia_types::{
    Character, Item,
    condition::Condition,
    database::{condition::Condition as DBCondition, spell::DatabaseSpell},
    spells::Spell,
};
use log::{error, info};
use reqwest::Url;
use rig::{
    client::{AgentClientExt, AgentModelExt, CompletionClient},
    prelude::Prompt,
    providers::openai,
};
use scraper::{Html, Selector};

const CONDITIONS: &str = include_str!("../../5e-database/src/2014/en/5e-SRD-Conditions.json");
const SPELLS: &str = include_str!("../../5e-database/src/2014/en/5e-SRD-Spells.json");

const PREAMBLE: &str = "You are a D&D 5e expert. Take the provided information and return **only** valid json according to the schema provided";
const MAX_RETRIES: u32 = 6;

#[derive(Debug)]
pub struct Registry {
    items: Vec<Item>,
    spells: Vec<Spell>,
    conditions: Vec<Condition>,
}

impl Registry {
    pub fn new(sheet: &Character) -> anyhow::Result<Self> {
        let conditions: Vec<DBCondition> = serde_json::from_str(CONDITIONS)?;
        let conditions = conditions.into_iter().map(Condition::from).collect();
        let spells: Vec<DatabaseSpell> = serde_json::from_str(SPELLS)?;
        let spells = spells
            .into_iter()
            .map(|f| Spell::try_from_database(f, sheet))
            .filter_map(std::result::Result::ok)
            .collect();

        Ok(Self {
            items: vec![],
            spells,
            conditions,
        })
    }
}
#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("Failed to fetch website content: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Html parsing error")]
    ParseError,
    #[error("LLM api error")]
    LlmError(#[from] rig::completion::PromptError),
    #[error("Url parse error: {0}")]
    UrlError(#[from] url::ParseError),
    #[error("{0}")]
    LlmHttpError(#[from] rig::http_client::Error),
    #[error("Llm hit max retries")]
    LlmMaxRetires,
}

pub(crate) fn parsednd5e(s: &str) -> Result<String, FetchError> {
    let document = Html::parse_document(&s);

    let selector = Selector::parse("#page-content").unwrap();

    let Some(inner) = document.select(&selector).next() else {
        return Err(FetchError::ParseError);
    };

    let html = inner.text().collect::<String>();
    Ok(html)
}

pub async fn scrape_websites(spell_name: &str) -> anyhow::Result<Vec<String>> {
    let mut results = vec![];

    let url = dnd5e_url(spell_name)?;
    let a = fetch_dnd_5e(url).await?;
    let a = format!("Spell name: {spell_name}\n\n {a}");
    results.push(a);

    Ok(results)
}

async fn fetch_dnd_5e(url: Url) -> Result<String, FetchError> {
    info!("Fetching: {}", url.as_str());
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    let data = parsednd5e(&text)?;

    Ok(data)
}

fn dnd5e_url(spell_name: &str) -> Result<Url, url::ParseError> {
    let s = spell_name.replace("'", "").replace(" ", "-").to_lowercase();
    info!("{s}");
    let i = format!("https://dnd5e.wikidot.com/spell:{s}");
    let url = Url::parse(&i)?;
    Ok(url)
}

pub async fn ask_llm(
    spell_info: Vec<String>,
    base_url: Url,
    api_key: Option<&str>,
    model: &str,
) -> Result<Spell, FetchError> {
    use rig::{
        agent::Agent,
        client::{CompletionClient, ProviderClient},
        completion::{AssistantContent, CompletionModel},
    };
    let client = rig::providers::openai::Client::builder()
        .base_url(base_url)
        .api_key(api_key.unwrap_or("dummy"))
        .build()?;
    let model = client.completion_model(model);

    let agent = model
        .into_agent_builder()
        .preamble(PREAMBLE)
        .max_tokens(2048)
        .build();

    let schema = schemars::schema_for!(Spell);
    let s = serde_json::to_string(&schema).expect("Static string");

    let prompt = format!("{}\n\n{:#?}", s, spell_info);

    let mut counter = 0;
    let mut last = None;
    let spell = loop {
        if counter > MAX_RETRIES {
            break None;
        }

        let prompt = if last.is_none() {
            prompt.clone()
        } else {
            format!(
                "Your last try failed to parse with error: {}. Fix issues and try again. \n\n{}",
                last.unwrap(),
                &prompt
            )
        };
        info!("Prompting");
        let resp = agent.prompt(prompt).await?;
        info!("Got response");

        match serde_json::from_str::<Spell>(&resp) {
            Ok(a) => break Some(a),
            Err(e) => last = Some(e),
        }
        counter += 1;
        info!("Parse failed, {counter}");
        info!("{resp}");
    };
    let Some(spell) = spell else {
        return Err(FetchError::LlmMaxRetires);
    };

    Ok(spell)
}
