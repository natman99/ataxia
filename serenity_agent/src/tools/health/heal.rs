use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tools::{InitError, SheetState};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct HealArgs {
    amount: i32,
}

pub struct Heal {
    pub inner: SheetState,
}

impl Tool for Heal {
    const NAME: &'static str = "Heal";

    type Error = rig::tool::ToolError;

    type Args = HealArgs;

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(HealArgs);
        ToolDefinition {
            name: "Heal".to_string(),
            description:
                "Apply healing to the character. Healing is added from the current health and capped at max health."
                    .to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut i = self.inner.lock();
        i.health.heal(args.amount);

        Ok(format!("Healed by {}", args.amount))
    }
}

impl ToolEmbedding for Heal {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Apply healing to the character.".into(),
            "Heal the character".into(),
            "Heal the sheet".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Heal {
            inner: state.clone(),
        })
    }
}
