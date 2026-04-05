use std::path::Path;

use std::sync::Arc;
use std::time::Instant;
use std::{fs, future};

use futures::SinkExt;
use log::{debug, info, warn};
use parking_lot::Mutex;
use qdrant_client::qdrant::{CreateCollectionBuilder, VectorParamsBuilder};
use rig::agent::{Agent, MultiTurnStreamItem};
use rig::client::{Capabilities, Client, CompletionClient, EmbeddingsClient, Provider};
use rig::completion::CompletionModel;
use rig::embeddings::{EmbeddingModel, EmbeddingsBuilder};
use rig::message::Message;
use rig::prelude::TypedPrompt;
use rig::providers::ollama::{self, OllamaExt};

use rig::streaming::{StreamedAssistantContent, StreamedUserContent, StreamingChat};
use rig::tool::ToolSet;
use schemars::JsonSchema;
use serde::Deserialize;
use serenity_types::Character;
use tokio::sync::mpsc::Sender;

use crate::tools::basic::{Adder, Multiply, Subtract};
use crate::tools::damage::Damage;
use crate::tools::get_sheet::GetSheet;
use crate::tools::heal::Heal;
use crate::tools::save_sheet::SaveSheet;
use crate::tools::search::Search;

mod database;
mod tools;

const QDRANT_URL: &str = "http://localhost:6334";
const INFO_COLLECTION: &str = "info";
const VECTOR_SIZE: u64 = 768;

const DOC_PATH: &str = r"C:\Users\Nathaniel\Nextcloud\Documents\Dnd";

const BLACKLIST: [&str; 4] = ["Wiki", "base", "Templates", "Categories"];
pub struct MyClient<T: AsRef<Path> + Clone> {
    judge_agent: Agent<ollama::CompletionModel>,
    agent: Agent<ollama::CompletionModel>,
    messages: Vec<Message>,
    tokens: u64,
    prev_messages: Vec<String>,
    _path: T,
    sender: tokio::sync::mpsc::Sender<String>,
}

impl<T: AsRef<Path> + Clone> MyClient<T> {
    pub async fn new(
        path: T,
        client: Client<OllamaExt>,
        sheet: Arc<Mutex<Character>>,
        sender: Sender<String>,
    ) -> anyhow::Result<Self> {
        let toolset = ToolSet::builder()
            .dynamic_tool(Adder)
            .dynamic_tool(Subtract)
            .dynamic_tool(Multiply)
            .static_tool(GetSheet {
                inner: sheet.clone(),
            })
            .dynamic_tool(GetSheet {
                inner: sheet.clone(),
            })
            .dynamic_tool(Damage {
                inner: sheet.clone(),
            })
            .dynamic_tool(SaveSheet {
                inner: sheet.clone(),
            })
            .dynamic_tool(Heal {
                inner: sheet.clone(),
            })
            .dynamic_tool(tools::ability_scores::SetScore {
                inner: sheet.clone(),
            })
            .dynamic_tool(tools::health::set::SetHealth {
                inner: sheet.clone(),
            })
            .dynamic_tool(tools::skills::SetProficiency {
                inner: sheet.clone(),
            })
            .dynamic_tool(tools::skills::SetProficiencyBonus {
                inner: sheet.clone(),
            })
            .dynamic_tool(tools::inventory::add::AddItem {
                inner: sheet.clone(),
            })
            .dynamic_tool(tools::inventory::remove::RemoveItem {
                inner: sheet.clone(),
            })
            .dynamic_tool(tools::inventory::get::GetInventory {
                inner: sheet.clone(),
            })
            .dynamic_tool(tools::armor::SetArmor {
                inner: sheet.clone(),
            })
            .dynamic_tool(tools::recalculate::Recalculate {
                inner: sheet.clone(),
            })
            .dynamic_tool(tools::reload::Reload {
                inner: sheet.clone(),
                path: path.as_ref().to_path_buf(),
            })
            .dynamic_tool(tools::feature::add::AddFeature {
                inner: sheet.clone(),
            })
            .build();

        let embedding_model = client.embedding_model(ollama::NOMIC_EMBED_TEXT);

        let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
            .documents(toolset.schemas()?)?
            .build()
            .await?;

        let vector_store =
            rig::vector_store::in_memory_store::InMemoryVectorStore::from_documents_with_id_f(
                embeddings,
                |tool| tool.name.clone(),
            );

        let index = vector_store.index(embedding_model.clone());

        let qdrant_client = qdrant_client::Qdrant::from_url(QDRANT_URL).build().unwrap();

        if !qdrant_client.collection_exists(INFO_COLLECTION).await? {
            qdrant_client
                .create_collection(
                    CreateCollectionBuilder::new(INFO_COLLECTION).vectors_config(
                        VectorParamsBuilder::new(
                            VECTOR_SIZE,
                            qdrant_client::qdrant::Distance::Cosine,
                        ),
                    ),
                )
                .await?;
        }

        info!("Running indexing");

        let t = Instant::now();

        let v_client = Arc::new(qdrant_client.clone());
        let v_client2 = v_client.clone();
        let v_embedding_model = Arc::new(embedding_model.clone());
        let v_embedding_model2 = v_embedding_model.clone();
        tokio::spawn(async move {
            let v_client = v_client.clone();
            let v_embedding_model = v_embedding_model.clone();
            match database::incremental_index(v_client, DOC_PATH, &v_embedding_model, &BLACKLIST)
                .await
            {
                Ok(_) => info!("Indexing finished"),
                Err(e) => log::error!("Indexing failed: {:?}", e),
            };
        });

        info!("Finished indexing in {} seconds", t.elapsed().as_secs_f32());

        info!("Starting indexing service");
        tokio::spawn(async move {
            let client = v_client2.clone();
            let v_model = v_embedding_model2.clone();

            database::run_update_service(client, v_model, DOC_PATH, &BLACKLIST).await;
        });

        let judge_agent = client.agent("qwen3:8b")
                .preamble("You are an assistant for deciding if a prompt or question is the same topic as the previous conversation OR a new topic.
                    Follow the providing schema. 100 is the highest confidence, 0 is the lowest.").build();

        let agent = client
                .agent("qwen3:8b")
                .preamble("You are a very careful D&D character sheet manager. You can take multiple turns. Call the get sheet tool liberally to assure your state is up to date.
                    ALWAYS use the provided tools to view, modify or save the sheet.
                    When in doubt, call the get sheet tool if you do not have access to information requested in the users prompt OR needed to carry out an action.
                    Do NOT guess values or make up sheet data. Use tools for EVERY change or read operation. Don't output markdown.
                    Always tell the user what you did. If you are not sure about something, ask the user.")
                .dynamic_tools(3, index, toolset)
                .tool(Search {
                    client: qdrant_client.clone(),
                    embedding_model: embedding_model.clone(),
                    reranker: Reranker::new()?,
                })
                // .dynamic_context(5, docs_v)
                .default_max_turns(8)

                .max_tokens(10 * 1000)
                // .tool(ThinkTool)


                .build();

        let se = Self {
            judge_agent,
            agent,
            tokens: 0,
            messages: vec![],
            prev_messages: vec![],
            _path: path,
            sender,
        };

        Ok(se)
    }

    pub async fn prompt<S: AsRef<str>>(&mut self, s: S) -> anyhow::Result<String> {
        let result: CheckOutput = self
            .judge_agent
            .prompt_typed(format!(
                "new prompt: {}. \n\nPrevious context: {:#?}",
                s.as_ref(),
                &self.prev_messages
            ))
            .await?;
        if result.confidence > 60 {
            debug!("{:?}: {:?}", result.classification, result.reason);
            match result.classification {
                PromptClassification::SameTopic => {}
                PromptClassification::NewTopic => {
                    debug!("Clearing messages");

                    self.messages.clear();
                    self.tokens = 0;
                    self.prev_messages.clear();
                }
            }
        }

        let mut response = self
            .agent
            .stream_chat(s.as_ref(), self.messages.clone())
            .await;
        use futures::StreamExt;

        let mut out = String::new();

        while let Some(e) = response.next().await {
            match e {
                Ok(item) => match item {
                    MultiTurnStreamItem::StreamAssistantItem(e) => {
                        // StreamedAssistantContent
                        match e {
                            StreamedAssistantContent::ToolCall {
                                tool_call,
                                internal_call_id: _,
                            } => {
                                debug!("Called tool: {}", tool_call.function.name);
                                if tool_call.function.name == "think" {
                                    debug!("Thoughts: {}", tool_call.function.arguments)
                                }

                                let _ = self.sender.send(tool_call.function.name).await;
                            }
                            _ => (),
                        }
                        // println!("{:?}", e)
                    }
                    MultiTurnStreamItem::FinalResponse(e) => {
                        if let Some(history) = e.history() {
                            self.messages.clear();
                            self.messages.append(&mut history.to_vec());
                        }
                        out.push_str(e.response());
                        self.prev_messages.push(format!("user: {}", s.as_ref()));
                        self.prev_messages.push(format!("agent: {}", e.response()));
                        self.tokens += e.usage().total_tokens;
                        debug!("tokens: {}", self.tokens);
                    }
                    MultiTurnStreamItem::StreamUserItem(item) => match item {
                        StreamedUserContent::ToolResult {
                            tool_result: _,
                            internal_call_id: _,
                        } => {
                            // debug!("{:?}", tool_result)
                        }
                    },
                    _ => (),
                },
                Err(e) => {
                    warn!("{:?}", e);
                }
            }
        }
        Ok(out)
    }
}

#[derive(Debug, Deserialize, JsonSchema, PartialEq)]
enum PromptClassification {
    NewTopic,
    SameTopic,
}

#[derive(Debug, JsonSchema, Deserialize)]
struct CheckOutput {
    /// 0-100 how confident you are. Integer.
    confidence: i32,
    /// new or same topic.
    classification: PromptClassification,
    /// Short one-sentence explanation of why you choise this classification.
    reason: String,
    /// What exactly is unclear without history? (or 'none' if standalone)
    #[serde(rename = "missing_referent")]
    _missing_referent: String,
}

use anyhow::Result;
use fastembed::{ExecutionProviderDispatch, RerankInitOptions, RerankerModel, TextRerank};
#[derive(Debug, Clone)]
pub struct Reranker {
    model: Arc<Mutex<TextRerank>>,
}

impl Reranker {
    pub fn new() -> Result<Self> {
        // Downloads ~400-500MB on first run, then caches
        //
        use ort::ep::{CPU, CUDA, DirectML};
        let execution_providers: Vec<ExecutionProviderDispatch> = vec![
            CUDA::default().build().error_on_failure(),
            DirectML::default().build(),
            CPU::default().build(),
        ];

        let model = TextRerank::try_new(
            RerankInitOptions::new(RerankerModel::BGERerankerV2M3)
                .with_execution_providers(execution_providers)
                .with_show_download_progress(true),
        )?;

        let model = Arc::new(Mutex::new(model));

        Ok(Self { model })
    }

    /// Reranks a list of documents for a given query.
    /// Returns top_k documents sorted by relevance (score 0.0 - 1.0)
    pub fn rerank(
        &self,
        query: &str,
        documents: Vec<&str>,
        top_k: usize,
    ) -> Result<Vec<(f32, String)>> {
        if documents.is_empty() {
            return Ok(vec![]);
        }

        // fastembed rerank returns scores (higher = more relevant)
        let results = self.model.lock().rerank(query, &documents, true, None)?;

        // results are already sorted by score descending

        let mut scored: Vec<(f32, String)> = results
            .into_iter()
            .map(|item| (item.score, item.document.unwrap_or("Error".to_string())))
            .collect();

        // Take only top_k
        if scored.len() > top_k {
            scored.truncate(top_k);
        }

        Ok(scored)
    }
}
