use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tools::{InitError, SheetState};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct RecalculateArgs {}

pub struct Recalculate {
    pub inner: SheetState,
}

impl Tool for Recalculate {
    const NAME: &'static str = "Recalculate";

    type Error = rig::tool::ToolError;

    type Args = RecalculateArgs;

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(RecalculateArgs);
        ToolDefinition {
            name: "Recalculate".to_string(),
            description:
                "Recalculate the sheet. This will update and recalculate all values. Only use this when you are **absolutely** sure the user is requesting it. When in doubt, ask and double check."
                    .to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut i = self.inner.lock();
        *i = i.calculate();

        Ok("Recalculated sheet".to_string())
    }
}

impl ToolEmbedding for Recalculate {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Recalculate".into(),
            "Recalculate sheet".into(),
            "RUBBER DUCKS. I LOVE RUBBER DUCKS".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Recalculate {
            inner: state.clone(),
        })
    }
}
