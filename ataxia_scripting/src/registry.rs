use std::fs;

use ataxia_types::{
    Character, Item,
    condition::Condition,
    database::{condition::Condition as DBCondition, spell::DatabaseSpell},
    spells::Spell,
};
use log::{error, info};
use reqwest::Url;
use scraper::{Html, Selector};

const CONDITIONS: &str = include_str!("../../5e-database/src/2014/en/5e-SRD-Conditions.json");
const SPELLS: &str = include_str!("../../5e-database/src/2014/en/5e-SRD-Spells.json");

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
    LlmError,
    #[error("Url parse error: {0}")]
    UrlError(#[from] url::ParseError),
}

pub(crate) fn parsednd5e(s: &str) -> Result<String, FetchError> {
    let document = Html::parse_document(&s);

    let selector = Selector::parse("#page-content").unwrap();

    let Some(inner) = document.select(&selector).next() else {
        fs::write("dummy.html", s).unwrap();
        return Err(FetchError::ParseError);
    };

    let html = inner.text().collect::<String>();
    Ok(html)
}

pub fn scrape_websites(spell_name: &str) -> anyhow::Result<Vec<String>> {
    let mut results = vec![];

    let url = dnd5e_url(spell_name)?;
    let a = fetch_dnd_5e(url)?;
    results.push(a);

    Ok(results)
}

fn fetch_dnd_5e(url: Url) -> Result<String, FetchError> {
    info!("Fetching: {}", url.as_str());
    let response = reqwest::blocking::get(url)?;
    let text = response.text()?;
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
