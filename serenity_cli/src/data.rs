use std::{fs, sync::Arc};

use serenity_types::database::{
    condition::Condition, generic::Generic, spell::Spell, traits::Trait,
};

const SPELLS_PATH: &'static str = r"../5e-database/src/2014/5e-SRD-Spells.json";
const CONDITIONS_PATH: &'static str = r"../5e-database/src/2014/5e-SRD-Conditions.json";
const TRAITS_PATH: &'static str = r"../5e-database/src/2014/5e-SRD-Traits.json";
const SKILLS_PATH: &'static str = r"../5e-database/src/2014/5e-SRD-Skills.json";
const RULES_PATH: &'static str = r"../5e-database/src/2014/5e-SRD-Rule-Sections.json";
const MAGIC_ITEMS_PATH: &'static str = r"../5e-database/src/2014/5e-SRD-Magic-Items.json";

#[derive(Debug, Clone)]
pub struct Data {
    pub spells: Arc<[Spell]>,
    pub conditions: Arc<[Condition]>,
    pub traits: Arc<[Trait]>,
    pub skills: Arc<[Generic]>,
    pub rules: Arc<[Generic]>,
    pub magic_items: Arc<[Generic]>,
}

impl Data {
    pub fn new() -> anyhow::Result<Self> {
        let spells = fs::read_to_string(SPELLS_PATH)?;

        let spells: Vec<Spell> = serde_json::from_str(&spells)?;
        let spells = Arc::from(spells.into_boxed_slice());

        let conditions = fs::read_to_string(CONDITIONS_PATH)?;

        let conditions: Vec<Condition> = serde_json::from_str(&conditions)?;
        let conditions = Arc::from(conditions.into_boxed_slice());

        let traits = fs::read_to_string(TRAITS_PATH)?;

        let traits: Vec<Trait> = serde_json::from_str(&traits)?;
        let traits = Arc::from(traits.into_boxed_slice());

        let skills = fs::read_to_string(SKILLS_PATH)?;

        let skills: Vec<Generic> = serde_json::from_str(&skills)?;
        let skills = Arc::from(skills.into_boxed_slice());

        let rules = fs::read_to_string(RULES_PATH)?;

        let rules: Vec<Generic> = serde_json::from_str(&rules)?;
        let rules = Arc::from(rules.into_boxed_slice());

        let magic_items = fs::read_to_string(MAGIC_ITEMS_PATH)?;

        let magic_items: Vec<Generic> = serde_json::from_str(&magic_items)?;
        let magic_items = Arc::from(magic_items.into_boxed_slice());

        Ok(Data {
            spells,
            conditions,
            traits,
            skills,
            rules,
            magic_items,
        })
    }
}
