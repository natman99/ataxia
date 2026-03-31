use std::{fs, path::Path};

use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding, ToolError},
    wasm_compat::{WasmCompatSend, WasmCompatSync},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serenity_types::Character;

use crate::tools::{InitError, SheetState, Success};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct ReloadArgs {}

pub struct Reload<T: AsRef<Path> + WasmCompatSend + Send + WasmCompatSync> {
    pub inner: SheetState,
    pub path: T,
}

impl<T: std::marker::Send + std::marker::Sync + AsRef<Path>> Tool for Reload<T> {
    const NAME: &'static str = "Reload";

    type Error = rig::tool::ToolError;

    type Args = ReloadArgs;

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(ReloadArgs);
        ToolDefinition {
            name: "Reload".to_string(),
            description: "Reload the character from disk. Used to sync with other changes made."
                .to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let p = self.path.as_ref().to_path_buf();
        let data = tokio::task::spawn_blocking(|| fs::read_to_string(p))
            .await
            .map_err(|e| ToolError::ToolCallError(Box::new(e)))?
            .map_err(|e| ToolError::ToolCallError(Box::new(e)))?;

        let j: Character =
            serde_json::from_str(&data).map_err(|e| ToolError::ToolCallError(Box::new(e)))?;

        let mut s = self.inner.lock();
        *s = j;

        Ok(Success::success())
    }
}

impl<T: Send + Sync + AsRef<Path> + Clone> ToolEmbedding for Reload<T> {
    type InitError = InitError;

    type Context = ();

    type State = (SheetState, T);

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Reload".into(), "Reload the sheet".into()]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Reload {
            inner: state.0.clone(),
            path: state.1.clone(),
        })
    }
}
