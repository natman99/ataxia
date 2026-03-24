use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serenity_types::{Score, ability_score::AbilityScore};

use crate::tools::{InitError, SheetState};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct SetScoreArgs {
    /// The amount. Does not accept negative numbers.
    amount: u32,
    /// The score to set
    score: Score,
    /// Optional bonus.
    bonus: Option<u32>,
}

pub struct SetScore {
    pub inner: SheetState,
}

impl Tool for SetScore {
    const NAME: &'static str = "Set score";

    type Error = rig::tool::ToolError;

    type Args = SetScoreArgs;

    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let s = schemars::schema_for!(SetScoreArgs);
        ToolDefinition {
            name: "Set score".to_string(),
            description:
                "Set an ability score to a new value. Scores are str, dex, con, int, wis, and char"
                    .to_string(),
            parameters: serde_json::to_value(s).expect("Schema error"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut i = self.inner.lock();
        let score = AbilityScore::new(args.amount, args.bonus.unwrap_or(0));
        match args.score {
            Score::Str => i.ability_scores.str = score,
            Score::Dex => i.ability_scores.dex = score,
            Score::Con => i.ability_scores.con = score,
            Score::Int => i.ability_scores.int = score,
            Score::Wis => i.ability_scores.wis = score,
            Score::Char => i.ability_scores.char = score,
        }

        Ok(format!("Set {} to {}", args.score, args.amount))
    }
}

impl ToolEmbedding for SetScore {
    type InitError = InitError;

    type Context = ();

    type State = SheetState;

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Override score".into(),
            "Set ability score".into(),
            "Set ability bonus".into(),
            "Update ability score".into(),
            "Set score".into(),
        ]
    }

    fn context(&self) -> Self::Context {}

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(SetScore {
            inner: state.clone(),
        })
    }
}
