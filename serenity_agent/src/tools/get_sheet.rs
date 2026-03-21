use std::fs;

use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding, ToolError},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tools::{InitError, SheetState};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct GetSheetArgs {}

pub struct GetSheet {
    pub inner: SheetState,
}

impl Tool for GetSheet {
    const NAME: &'static str = "Get sheet";

    type Error = rig::tool::ToolError;

    type Args = GetSheetArgs;

    // type Output = String;

    type Output = serenity_types::Character;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(GetSheetArgs);
        ToolDefinition {
            name: "Get sheet".to_string(),
            description: "Get the character sheet. This tool can be used to get the current state. Use whenever you report details after updating a value.".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let s = self.inner.lock();
        // let res = match serde_json::to_string_pretty(&*s) {
        //     Ok(s) => s,
        //     Err(e) => return Err(ToolError::ToolCallError(Box::new(e))),
        // };
        Ok(s.clone())
    }
}

impl ToolEmbedding for GetSheet {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Get the character sheet".into(),
            "Get the sheet".into(),
            "Get character".into(),
            "Get state".into(),
            "Call this tool to get the character sheet".into(),
            "Update state".into(),
            "Get information".into(),
            "Show me x".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(GetSheet {
            inner: state.clone(),
        })
    }
}
