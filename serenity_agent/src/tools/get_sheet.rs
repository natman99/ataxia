use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
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

    type Output = serenity_types::Character;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(GetSheetArgs);
        ToolDefinition {
            name: "Get sheet".to_string(),
            description: "Get the character sheet. This tool can be used to get the current state. This includes health, class, name, features, traits, backstore, ability scores, and initiative.
                Use whenever the user requests ANY information or when you need more information to fufill a request.
                For the purposes of reducing context size, this does not contain the inventory. Use the seperate tool for that.".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let s = self.inner.lock();

        let mut i = s.clone();
        i.inventory.0.clear();
        Ok(i)
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
            "Get traits".into(),
            "Get features".into(),
            "Get hp".into(),
            "Show me x".into(),
            "Tell me x".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(GetSheet {
            inner: state.clone(),
        })
    }
}
