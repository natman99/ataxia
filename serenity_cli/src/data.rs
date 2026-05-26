use std::{fs, sync::Arc};

use serenity_types::database::{condition::Condition, spell::Spell};

const SPELLS_PATH: &'static str = r"../5e-database/src/2014/5e-SRD-Spells.json";
const CONDITIONS_PATH: &'static str = r"../5e-database/src/2014/5e-SRD-Spells.json";

#[derive(Debug, Clone)]
pub struct Data {
    pub spells: Arc<[Spell]>,
    pub conditions: Arc<[Condition]>,
}

impl Data {
    pub fn new() -> anyhow::Result<Self> {
        let s = fs::read_to_string(SPELLS_PATH)?;
        let spells: Vec<Spell> = serde_json::from_str(&s)?;
        let spells = Arc::from(spells.into_boxed_slice());

        let s = fs::read_to_string(CONDITIONS_PATH)?;

        let conditions: Vec<Condition> = serde_json::from_str(&s)?;
        let conditions = Arc::from(conditions.into_boxed_slice());

        Ok(Data { spells, conditions })
    }
}
