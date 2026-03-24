use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serenity_types::Item;

use crate::tools::{InitError, SheetState};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct RemoveSuccess {
    /// The result of the operation.
    result: String,
    /// The item that was removed, if found.
    removed: Option<Item>,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct RemoveItemArgs {
    /// This corrosponds to the item's id. This is an all lowercase version of the item's name, with underscores replacing spaces.
    item_id: String,
}

pub struct RemoveItem {
    pub inner: SheetState,
}

impl Tool for RemoveItem {
    const NAME: &'static str = "Remove item";

    type Error = rig::tool::ToolError;

    type Args = RemoveItemArgs;

    type Output = RemoveSuccess;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(RemoveItemArgs);
        ToolDefinition {
            name: "Remove item".to_string(),
            description: "Remove an item to the character sheet. Use this tool when the user asks to remove or delete an item. If the returned item is null, inform the user there was no matching item.".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut s = self.inner.lock();

        let old = s.inventory.0.remove(&args.item_id);

        Ok(RemoveSuccess {
            result: "Success".to_string(),
            removed: old,
        })
    }
}

impl ToolEmbedding for RemoveItem {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Remove item".into(),
            "Delete item".into(),
            "Modify inventory".into(),
            "Set item".into(),
            "Update item".into(),
            "Delete equipment".into(),
            "Remove equipment".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(RemoveItem {
            inner: state.clone(),
        })
    }
}
