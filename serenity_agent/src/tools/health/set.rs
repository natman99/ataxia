use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serenity_types::HitPoints;

use crate::tools::{InitError, SheetState, Success};

pub struct SetHealth {
    pub inner: SheetState,
}

impl Tool for SetHealth {
    const NAME: &'static str = "Set health";

    type Error = rig::tool::ToolError;

    type Args = HitPoints;

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(HitPoints);
        ToolDefinition {
            name: "Set health".to_string(),
            description: "Manually set health object. Only use if heal and damage cannot be used, e.g. the user does not wish to simply add or remove from the current health."
                .to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut i = self.inner.lock();
        i.health = args;
        Ok(Success::success())
    }
}

impl ToolEmbedding for SetHealth {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Set bonus health".into(),
            "Set temp health".into(),
            "Set health manually".into(),
            "Set health attributes".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(SetHealth {
            inner: state.clone(),
        })
    }
}
