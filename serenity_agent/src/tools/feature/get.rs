use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tools::{InitError, SheetState};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct GetInventoryArgs {}

pub struct GetInventory {
    pub inner: SheetState,
}

impl Tool for GetInventory {
    const NAME: &'static str = "Get inventory";

    type Error = rig::tool::ToolError;

    type Args = GetInventoryArgs;

    // type Output = String;

    type Output = serenity_types::Inventory;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(GetInventoryArgs);
        ToolDefinition {
            name: "Get inventory".to_string(),
            description: "Get the inventory. This contains all the items the character has. Use this to get information on items and equipment.".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let s = self.inner.lock();

        let i = s.clone();
        Ok(i.inventory)
    }
}

impl ToolEmbedding for GetInventory {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Get inventory".into(),
            "Get items".into(),
            "Does the sheet have x?".into(),
            "Does the character have x?".into(),
            "What items does the character sheet have".into(),
            "Equipment".into(),
            "View inventory".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(GetInventory {
            inner: state.clone(),
        })
    }
}
