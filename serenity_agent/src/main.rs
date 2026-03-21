use std::error::Error;
use std::fs;
use std::sync::Arc;

use parking_lot::Mutex;
use rig::OneOrMany;
use rig::agent::{MultiTurnStreamItem, stream_to_stdout};
use rig::client::{CompletionClient, EmbeddingsClient, Nothing};
use rig::completion::{Chat, Prompt};
use rig::embeddings::{EmbeddingModel, EmbeddingsBuilder};
use rig::message::{AssistantContent, Message, UserContent};
use rig::prelude::TypedPrompt;
use rig::providers::ollama;

use rig::streaming::{StreamedAssistantContent, StreamedUserContent, StreamingChat};
use rig::tool::ToolSet;
use rig::tools::ThinkTool;
use rig::vector_store::{VectorSearchRequest, VectorStoreIndexDyn};
use schemars::JsonSchema;
use serde::Deserialize;
use serenity_types::Character;
use tokio::io;

use crate::tools::basic::{Adder, Multiply, Subtract};
use crate::tools::damage::Damage;
use crate::tools::get_sheet::GetSheet;
use crate::tools::heal::Heal;
use crate::tools::save_sheet::SaveSheet;

mod tools;

#[tokio::main]
async fn main() {
    let client = ollama::Client::new(Nothing).unwrap();

    let s = fs::read_to_string("test.json").unwrap();
    let sheet: Character = serde_json::from_str(&s).unwrap();
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
        .dynamic_tool(crate::tools::ability_scores::SetScore {
            inner: sheet.clone(),
        })
        .dynamic_tool(crate::tools::health::set::SetHealth {
            inner: sheet.clone(),
        })
        .build();

    let embedding_model = client.embedding_model(ollama::NOMIC_EMBED_TEXT);
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .documents(toolset.schemas().unwrap())
        .unwrap()
        .build()
        .await
        .unwrap();

    let vector_store =
        rig::vector_store::in_memory_store::InMemoryVectorStore::from_documents_with_id_f(
            embeddings,
            |tool| tool.name.clone(),
        );

    let index = vector_store.index(embedding_model);

    #[derive(Debug, Deserialize, JsonSchema)]
    enum PromptClassification {
        StandAlone,
        ContextDepndant,
    }

    #[derive(Debug, JsonSchema, Deserialize)]
    struct CheckOutput {
        /// 0-100 how confident you are. Integer.
        confidence: i32,
        /// Standalone or context dependant.
        classification: PromptClassification,
        /// Short one-sentence explanation of why you choise this classification.
        reason: String,
        /// What exactly is unclear without history? (or 'none' if standalone)
        missing_referent: String,
    }

    let check_agent = client.agent("qwen3:8b")
        .preamble("You are an assistant for deciding if a prompt requires previous context in order to be understood. References to 'the character' or 'the sheet' are not considered as requiring context.
            Follow the providing schema. The number is the probability of the instruction being self contained or not. 100 means the question requires context, 0 means it does not.").build();
    // let result: CheckOutput = check_agent
    //     .prompt_typed("What was his name again?")
    //     .await
    //     .unwrap();

    // println!("Chances: {:?}", result);

    let agent = client
        .agent("qwen3:8b")
        .preamble("You are a very careful D&D character sheet manager. Use the think tool for multiple step instructions to plan accordingly. Call the get sheet tool liberally to assure your state is up to date.
            ALWAYS use the provided tools to view, modify or save the sheet. When in doubt, call the get sheet tool if you do not have access to any information requested in the users prompt.
            Do NOT guess values or make up sheet data. Use tools for EVERY change or read operation. Don't output markdown. Always output some final result, even if it's just \"success\"")
        .dynamic_tools(3, index, toolset)
        .default_max_turns(8)
        .tool(ThinkTool)
        // .tool_choice(rig::message::ToolChoice::Required)


        .build();

    let mut messages = vec![];
    println!("Ready.");
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        let result: CheckOutput = check_agent.prompt_typed(&buf).await.unwrap();
        println!("{:?}", result);
        // High chance of context required.
        if result.confidence > 60 {
        } else {
            println!("Clearing messages");
            messages.clear();
        }

        let mut response = agent.stream_chat(&buf, messages.clone()).await;
        // MultiTurnStreamItem;
        use futures::StreamExt;
        while let Some(e) = response.next().await {
            match e {
                Ok(item) => match item {
                    MultiTurnStreamItem::StreamAssistantItem(e) => {
                        // StreamedAssistantContent
                        match e {
                            StreamedAssistantContent::ToolCall {
                                tool_call,
                                internal_call_id,
                            } => {
                                println!("Called tool: {}", tool_call.function.name);
                            }
                            _ => (),
                        }
                        // println!("{:?}", e)
                    }
                    MultiTurnStreamItem::FinalResponse(e) => {
                        if let Some(history) = e.history() {
                            messages.clear();
                            messages.append(&mut history.to_vec());
                        }
                        println!(">  {}", e.response());

                        println!("tokens: {}", e.usage().total_tokens);
                    }
                    MultiTurnStreamItem::StreamUserItem(item) => match item {
                        StreamedUserContent::ToolResult {
                            tool_result,
                            internal_call_id,
                        } => {
                            // println!("{:?}", tool_result)
                        }
                    },
                    _ => (),
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }

        buf.clear();
    }
}
