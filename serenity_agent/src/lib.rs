use std::path::Path;

use std::fs;
use std::sync::Arc;
use std::time::Instant;

use log::{debug, info, warn};
use parking_lot::Mutex;
use qdrant_client::qdrant::{CreateCollectionBuilder, VectorParamsBuilder};
use rig::agent::{Agent, MultiTurnStreamItem};
use rig::client::{Client, CompletionClient, EmbeddingsClient};
use rig::embeddings::EmbeddingsBuilder;
use rig::message::Message;
use rig::prelude::TypedPrompt;
use rig::providers::ollama::{self, CompletionModel, OllamaExt};

use rig::streaming::{StreamedAssistantContent, StreamedUserContent, StreamingChat};
use rig::tool::ToolSet;
use schemars::JsonSchema;
use serde::Deserialize;
use serenity_types::Character;

use crate::tools::basic::{Adder, Multiply, Subtract};
use crate::tools::damage::Damage;
use crate::tools::get_sheet::GetSheet;
use crate::tools::heal::Heal;
use crate::tools::save_sheet::SaveSheet;
use crate::tools::search::Search;

mod database;
mod tools;

const QDRANT_URL: &'static str = "http://localhost:6334";
const INFO_COLLECTION: &'static str = "info";
const VECTOR_SIZE: u64 = 768;

const DOC_PATH: &'static str = r"C:\Users\Nathaniel\Nextcloud\Documents\Dnd";

pub struct MyClient {
    client: Client<OllamaExt>,
    judge_agent: Agent<CompletionModel>,
    agent: Agent<CompletionModel>,
    messages: Vec<Message>,
    prev_messages: Vec<String>,
}

impl MyClient {
    pub async fn new<T: AsRef<Path>>(path: T, client: Client<OllamaExt>) -> anyhow::Result<Self> {
        let s = fs::read_to_string(path.as_ref())?;

        let sheet: Character = serde_json::from_str(&s)?;
        let sheet = Arc::new(Mutex::new(sheet));
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
        database::incremental_index(&qdrant_client, DOC_PATH, &embedding_model).await?;
        info!("Finished indexing in {} seconds", t.elapsed().as_secs_f32());
        let judge_agent: Agent<CompletionModel> = client.agent("qwen3:8b")
                .preamble("You are an assistant for deciding if a prompt or question is the same topic as the previous conversation.
                    Follow the providing schema. 100 is the highest confidence, 0 is the lowest.").build();

        let agent = client
                .agent("qwen3:8b")
                .preamble("You are a very careful D&D character sheet manager. You can take multiple turns. Call the get sheet tool liberally to assure your state is up to date. Use the provided information to answer the users questions.
                    ALWAYS use the provided tools to view, modify or save the sheet. When in doubt, call the get sheet tool if you do not have access to information requested in the users prompt OR needed to carry out an action.
                    Do NOT guess values or make up sheet data. Use tools for EVERY change or read operation. Don't output markdown. Always tell the user what you did. If you are not sure about something, ask the user.")
                .dynamic_tools(3, index, toolset)
                // .tool(Search {
                //     inner: docs_v
                // }) TODO
                // .dynamic_context(5, docs_v)
                .default_max_turns(8)

                .max_tokens(10 * 1000)
                // .tool(ThinkTool)


                .build();

        let se = Self {
            client,
            judge_agent,
            agent,
            messages: vec![],
            prev_messages: vec![],
        };

        Ok(se)
    }

    pub async fn prompt(&mut self, s: &str) -> anyhow::Result<String> {
        let result: CheckOutput = self
            .judge_agent
            .prompt_typed(&format!(
                "new prompt: {}. Previous prompt: {:#?}",
                s, &self.prev_messages
            ))
            .await
            .unwrap();
        println!("{:?}: {:?}", result.classification, result.reason);
        // High chance of context required.
        if result.confidence > 60 {
            match result.classification {
                PromptClassification::SameTopic => {}
                PromptClassification::StandAlone => {
                    debug!("Clearing messages");

                    self.messages.clear();
                    self.prev_messages.clear();
                }
            }
        }

        let mut response = self.agent.stream_chat(s, self.messages.clone()).await;
        use futures::StreamExt;

        let mut output = String::new();

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
                            }
                            _ => (),
                        } // Add an item called wooden sword. Make up a description. It deals 2d6 damage
                        // println!("{:?}", e)
                    }
                    MultiTurnStreamItem::FinalResponse(e) => {
                        if let Some(history) = e.history() {
                            self.messages.clear();
                            self.messages.append(&mut history.to_vec());
                        }
                        output.push_str(e.response());
                        self.prev_messages.push(format!("user: {}", s.to_string()));
                        self.prev_messages
                            .push(format!("agent: {}", e.response().to_string()));
                        debug!("tokens: {}", e.usage().total_tokens);
                    }
                    MultiTurnStreamItem::StreamUserItem(item) => match item {
                        StreamedUserContent::ToolResult {
                            tool_result: _,
                            internal_call_id: _,
                        } => {
                            // println!("{:?}", tool_result)
                        }
                    },
                    _ => (),
                },
                Err(e) => {
                    warn!("{:?}", e);
                }
            }
        }

        Ok(output)
    }
}

#[derive(Debug, Deserialize, JsonSchema, PartialEq)]
enum PromptClassification {
    StandAlone,
    SameTopic,
}

#[derive(Debug, JsonSchema, Deserialize)]
struct CheckOutput {
    /// 0-100 how confident you are. Integer.
    confidence: i32,
    /// Standalone or same topic.
    classification: PromptClassification,
    /// Short one-sentence explanation of why you choise this classification.
    reason: String,
    /// What exactly is unclear without history? (or 'none' if standalone)
    missing_referent: String,
}
