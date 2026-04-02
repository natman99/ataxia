use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serenity_types::{Item, feature::Feature, roll::Roll};

use crate::tools::{InitError, SheetState};

pub struct AddFeature {
    pub inner: SheetState,
}

impl Tool for AddFeature {
    const NAME: &'static str = "Add feature";

    type Error = rig::tool::ToolError;

    type Args = Feature;

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(Feature);
        ToolDefinition {
            name: "Add feature".to_string(),
            description: "Add or update a feature on the character sheet. A call with the same name replaces the previous feature, so multiple similar features MUST have unique names. Use this behavior to update the feature if the user asks.
                Features can include feats, class features, or traits".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut s = self.inner.lock();
        log::debug!("{:?}", &args);

        s.features.inner.insert(args.name.clone(), args);

        Ok("Feature added".to_string())
    }
}

impl ToolEmbedding for AddFeature {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Add feature".into(),
            "Add trait".into(),
            "Add class feature".into(),
            "Update feature".into(),
            "Add feat".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(AddFeature {
            inner: state.clone(),
        })
    }
}
