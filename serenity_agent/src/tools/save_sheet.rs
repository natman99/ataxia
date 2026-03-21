use std::fs;

use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding, ToolError},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tools::{InitError, SheetState};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct LoadSheetArgs {}

pub struct SaveSheet {
    pub inner: super::SheetState,
}

impl Tool for SaveSheet {
    const NAME: &'static str = "Save sheet";

    type Error = rig::tool::ToolError;

    type Args = LoadSheetArgs;

    type Output = ();

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(LoadSheetArgs);
        ToolDefinition {
            name: "Save sheet".to_string(),
            description: "Save the character sheet".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // println!("Save sheet");
        let s = self.inner.lock();
        match serde_json::to_string_pretty(&*s) {
            Ok(string) => match fs::write("test.json", string) {
                Ok(_) => (),
                Err(e) => return Err(ToolError::ToolCallError(Box::new(e))),
            },
            Err(e) => return Err(ToolError::ToolCallError(Box::new(e))),
        }
        Ok(())
    }
}

impl ToolEmbedding for SaveSheet {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Save the character sheet".into(), "Save state".into()]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(SaveSheet {
            inner: state.clone(),
        })
    }
}
