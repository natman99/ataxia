use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tools::{InitError, SheetState};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct SetArmorArgs {
    amount: i32,
}

pub struct SetArmor {
    pub inner: SheetState,
}

impl Tool for SetArmor {
    const NAME: &'static str = "Set armor";

    type Error = rig::tool::ToolError;

    type Args = SetArmorArgs;

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(SetArmorArgs);
        ToolDefinition {
            name: "Set armor".to_string(),
            description:
                "Set the character's armor. This should normally not be called manually. Only call if the user explicitly requests it."
                    .to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut i = self.inner.lock();

        i.armor_class.set(args.amount as u32);

        Ok(format!("Set armor class to {}", args.amount))
    }
}

impl ToolEmbedding for SetArmor {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Set armor".into(),
            "Apply armor".into(),
            "Set armor class".into(),
            "Modify armor class".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(SetArmor {
            inner: state.clone(),
        })
    }
}
