use std::fs;

use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding, ToolError},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tools::InitError;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct LoadSheetArgs {}

#[derive(Serialize, Deserialize)]
pub struct LoadSheet;

impl Tool for LoadSheet {
    const NAME: &'static str = "Load sheet";

    type Error = rig::tool::ToolError;

    type Args = LoadSheetArgs;

    type Output = serenity_types::Character;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(LoadSheetArgs);
        ToolDefinition {
            name: "Load sheet".to_string(),
            description: "Load the character sheet".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("Load sheet");
        let s = match fs::read_to_string("test.json") {
            Ok(s) => s,
            Err(e) => return Err(ToolError::ToolCallError(Box::new(e))),
        };
        let s = match serde_json::from_str(&s) {
            Ok(s) => s,
            Err(e) => return Err(ToolError::JsonError(e)),
        };
        Ok(s)
    }
}

impl ToolEmbedding for LoadSheet {
    type InitError = InitError;

    type Context = ();

    type State = ();

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Load the character sheet".into()]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(LoadSheet)
    }
}
