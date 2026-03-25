use crate::tools::{InitError, SheetState};
use log::{debug, info};
use rig::{
    completion::ToolDefinition,
    providers::ollama::EmbeddingModel,
    tool::{Tool, ToolEmbedding, ToolError},
    vector_store::{VectorStoreIndexDyn, request::VectorSearchRequestBuilder},
};
use rig_qdrant::QdrantVectorStore;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct SearchArgs {
    search_term: String,
}

pub struct Search {
    pub inner: QdrantVectorStore<EmbeddingModel>,
}

impl Tool for Search {
    const NAME: &'static str = "Search";

    type Error = rig::tool::ToolError;

    type Args = SearchArgs;

    type Output = Vec<Value>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(SearchArgs);
        ToolDefinition {
            name: "Search".to_string(),
            description: "Search the provided database. Use this whenever the user asks for details about items, rules, story elements, or enemies. This uses a semantic search.
                Report to the user if there are no results.".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        info!("Search: {}", &args.search_term);
        let search = match VectorSearchRequestBuilder::default()
            .query(args.search_term)
            .samples(10)
            .threshold(0.8)
            .build()
        {
            Ok(e) => e,
            Err(e) => return Err(ToolError::ToolCallError(Box::new(e))),
        };
        let results = match self.inner.top_n(search).await {
            Ok(a) => a,
            Err(e) => return Err(ToolError::ToolCallError(Box::new(e))),
        };
        debug!("{:?}", results);
        let out = results.into_iter().map(|f| f.2).collect::<Vec<Value>>();
        Ok(out)
    }
}
