use std::error::Error;

use rig::client::{CompletionClient, EmbeddingsClient, Nothing, ProviderClient};
use rig::completion::{Prompt, ToolDefinition};
use rig::embeddings::EmbeddingsBuilder;
use rig::providers::ollama;

use rig::tool::{Tool, ToolEmbedding, ToolSet};
use rig_derive::rig_tool;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = ollama::Client::new(Nothing).unwrap();

    let toolset = ToolSet::builder().dynamic_tool(Adder).build();

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

    let agent = client
        .agent("qwen3:8b")
        .preamble("You are a hyper serious assassin assistant. Call the tools, report results, and make edgy quips.")
        .dynamic_tools(1, index, toolset)
        .build();

    println!("Ready.");
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        let response = agent.prompt(&buf).await.unwrap();

        println!("{:?}", response);
        buf.clear();
    }
}
#[derive(Deserialize, Serialize, Debug)]
struct AddArgs {
    x: i32,

    y: i32,
}
#[derive(Serialize, Deserialize)]
struct Adder;

impl Tool for Adder {
    const NAME: &'static str = "Add";

    type Error = rig::tool::ToolError;

    type Args = AddArgs;

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "Add".to_string(),
            description: "Adds two numbers".to_string(),
            parameters: serde_json::from_value(json!({
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "First number"
                    },
                    "y": {
                        "type": "number",
                        "description": "First number"
                   }

                },
                // "required": ["q"]

            }))
            .expect("Should always work"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("{:?}", args);
        Ok(format!("{}", args.x + args.y))
    }
}

impl ToolEmbedding for Adder {
    type InitError = InitError;

    type Context = ();

    type State = ();

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Get the instructions".into()]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Adder)
    }
}
#[derive(Debug, thiserror::Error)]
#[error("Init Error")]
struct InitError;
