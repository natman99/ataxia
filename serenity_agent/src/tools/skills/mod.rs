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

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(SetProficiencyArgs);
        ToolDefinition {
            name: "Set proficiency".to_string(),
            description: "Set a skill proficiency on the sheet. Use this whenever the user asks to set a skill proficiency.".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // println!("Damage");
        let mut i = self.inner.lock();

        i.skills.proficiencies.set(args.skill, args.yes);

        Ok(format!("Set {} to {}", args.skill, args.yes))
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
            "Set skill proficiencies".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(SetProficiency {
            inner: state.clone(),
        })
    }
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct SetProficiencyBonusArgs {
    /// The bonus to set. Should not be negative.
    bonus: u32,
}

pub struct SetProficiencyBonus {
    pub inner: SheetState,
}

impl Tool for SetProficiencyBonus {
    const NAME: &'static str = "Set proficiency bonus";

    type Error = rig::tool::ToolError;

    type Args = SetProficiencyBonusArgs;

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(SetProficiencyArgs);
        ToolDefinition {
            name: "Set proficiency bonus".to_string(),
            description: "Set the skill proficiency bonus on the sheet. Use this whenever the user asks to update the bonus.".to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // println!("Damage");
        let mut i = self.inner.lock();

        i.skills.proficiency_bonus = args.bonus;

        Ok(format!("Set bonus to {}", args.bonus))
    }
}

impl ToolEmbedding for SetProficiencyBonus {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Set the skill proficiency bonus.".into(),
            "Set skill bonus".into(),
            "Set skill proficiency bonus.".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(SetProficiencyBonus {
            inner: state.clone(),
        })
    }
}
