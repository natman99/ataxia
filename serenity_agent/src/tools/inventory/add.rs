use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serenity_types::{Item, roll::Roll};

use crate::tools::{InitError, SheetState};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct AddSuccess {
    /// The result of the operation.
    result: String,
    /// The item that was added.
    added: Item,
    /// The old version of the item, if there is one.
    old_item: Option<Item>,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct AddItemArgs {
    /// Id of the item. This is an all lowercase version of the item's name with underscores instead of spaces.
    pub id: String,
    /// The name of the item
    pub name: String,
    /// The description of the item.
    pub description: String,
    /// How many of the item? Should never be zero.
    pub quantity: i32,
    /// The roll if the item deals damage or has an effect.
    pub roll: Option<String>,
}

pub struct AddItem {
    pub inner: SheetState,
}

impl Tool for AddItem {
    const NAME: &'static str = "Add item";

    type Error = rig::tool::ToolError;

    type Args = AddItemArgs;

    type Output = AddSuccess;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(AddItemArgs);
        ToolDefinition {
            name: "Add item".to_string(),
            description: "Add or update an item to the character sheet. Use this tool when the user asks to add or update an item. This function replaces items with the same name, so you can also use it to update items.".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut s = self.inner.lock();
        let roll = if let Some(r) = args.roll {
            Roll::try_from(r.as_str()).ok()
        } else {
            None
        };
        let item = Item {
            id: args.id,
            name: args.name,
            description: args.description,
            quantity: args.quantity,
            roll,
        };
        let old = s.inventory.0.insert(item.id.clone(), item.clone());

        Ok(AddSuccess {
            result: "Success".to_string(),
            added: item,
            old_item: old,
        })
    }
}

impl ToolEmbedding for AddItem {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Add item".into(),
            "Add equipment".into(),
            "Modify item".into(),
            "Modify inventory".into(),
            "Set item".into(),
            "Update item".into(),
            "Create item".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(AddItem {
            inner: state.clone(),
        })
    }
}
