use std::{fs, sync::Arc};

use parking_lot::Mutex;
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding, ToolError},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serenity_types::Character;

use crate::tools::InitError;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct DamageArgs {
    num: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Damage {
    inner: Arc<Mutex<Character>>,
}

impl Tool for Damage {
    const NAME: &'static str = "Damage";

    type Error = rig::tool::ToolError;

    type Args = DamageArgs;

    type Output = ();

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
        println!("Damage");
        let i = self.inner.lock();
        i.health.current -= args.num;

        Ok(())
    }
}

impl ToolEmbedding for Damage {
    type InitError = InitError;

    type Context = ();

    type State = ();

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Apply damage to the character.".into(),
            "Damage the character".into(),
            "Damage the sheet".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(LoadSheet)
    }
}
