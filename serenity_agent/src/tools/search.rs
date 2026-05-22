use crate::{INFO_COLLECTION, Reranker};
use log::info;
use qdrant_client::{Qdrant, qdrant::QueryPointsBuilder};
use rig::{
    completion::ToolDefinition,
    embeddings::EmbeddingModel,
    tool::{Tool, ToolError},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct SearchArgs {
    search_term: String,
}

pub struct Search<T: EmbeddingModel> {
    pub client: Qdrant,
    pub embedding_model: T,
    pub reranker: Reranker,
}

impl<T: EmbeddingModel> Tool for Search<T> {
    const NAME: &'static str = "Search";

    type Error = rig::tool::ToolError;

    type Args = SearchArgs;

    type Output = Vec<String>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(SearchArgs);
        ToolDefinition {
            name: "Search".to_string(),
            description: "Search the provided database. Use this whenever the user asks for details about characters, items, rules, story elements, or enemies. This uses a vector search, feel free to put questions or phrases as well as search terms.
                Report to the user if there are no results.".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        info!("Search: {}", &args.search_term);

        let embedding = self
            .embedding_model
            .embed_text(&args.search_term)
            .await
            .map_err(|e| ToolError::ToolCallError(Box::new(e)))?;
        let results = match self
            .client
            .query(
                QueryPointsBuilder::new(INFO_COLLECTION)
                    // .score_threshold(0.4)
                    .query(
                        embedding
                            .vec
                            .into_iter()
                            .map(|f| f as f32)
                            .collect::<Vec<f32>>(),
                    )
                    .with_payload(true)
                    .limit(30)
                    .build(),
            )
            .await
        {
            Ok(e) => e,
            Err(e) => return Err(ToolError::ToolCallError(Box::new(e))),
        };

        // results.result.iter().for_each(|f| info!("{}", f.score));

        let documents = results
            .result
            .iter()
            .map(|f| &f.payload["content"])
            .filter_map(|f| f.as_str())
            .map(|f| f.trim())
            .collect::<Vec<&str>>();
        let out = self
            .reranker
            .rerank(&args.search_term, documents, 8)
            .unwrap();

        info!("{:#?}", out);
        let out = out
            .into_iter()
            .filter(|f| f.0 > -6.0)
            .map(|f| f.1)
            .collect();

        // TODO headers
        Ok(out)
    }
}
