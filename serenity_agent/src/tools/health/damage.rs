use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tools::{InitError, SheetState};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct DamageArgs {
    amount: i32,
}

pub struct Damage {
    pub inner: SheetState,
}

impl Tool for Damage {
    const NAME: &'static str = "Damage";

    type Error = rig::tool::ToolError;

    type Args = DamageArgs;

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(DamageArgs);
        ToolDefinition {
            name: "Damage".to_string(),
            description:
                "Apply damage to the character. Damage is subtracted from the total health."
                    .to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // println!("Damage");
        let mut i = self.inner.lock();
        i.health.hit(args.amount);

        Ok(format!("Took {} points of damage", args.amount))
    }
}

impl ToolEmbedding for Damage {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Apply damage to the character.".into(),
            "Damage the character".into(),
            "Damage the sheet".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Damage {
            inner: state.clone(),
        })
    }
}
