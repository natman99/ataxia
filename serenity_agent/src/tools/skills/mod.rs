use std::{fs, sync::Arc};

use parking_lot::Mutex;
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding, ToolError},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serenity_types::{Character, skills::Skill};

use crate::tools::{InitError, SheetState};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct SetProficiencyArgs {
    skill: serenity_types::skills::Skill,
    yes: bool,
}

pub struct SetProficiency {
    pub inner: SheetState,
}

impl Tool for SetProficiency {
    const NAME: &'static str = "Set proficiency";

    type Error = rig::tool::ToolError;

    type Args = SetProficiencyArgs;

    type Output = ();

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(SetProficiencyArgs);
        ToolDefinition {
            name: "Set proficiency".to_string(),
            description: "Set a skill proficiency on the sheet".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // println!("Damage");
        let mut i = self.inner.lock();

        i.skills.proficiencies.set(args.skill, args.yes);

        Ok(())
    }
}

impl ToolEmbedding for SetProficiency {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Set a skill proficiency.".into(),
            "Set skill".into(),
            "Set skill proficiencs.".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(SetProficiency {
            inner: state.clone(),
        })
    }
}
