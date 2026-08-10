use ataxia_types::{
    Character, Item,
    condition::Condition,
    database::{condition::Condition as DBCondition, spell::DatabaseSpell},
    spells::Spell,
};

const CONDITIONS: &str = include_str!("../../5e-database/src/2014/en/5e-SRD-Conditions.json");
const SPELLS: &str = include_str!("../../5e-database/src/2014/en/5e-SRD-Spells.json");

#[derive(Debug)]
pub struct Global {
    items: Vec<Item>,
    spells: Vec<Spell>,
    conditions: Vec<Condition>,
}

impl Global {
    pub fn new(sheet: &Character) -> anyhow::Result<Self> {
        let conditions: Vec<DBCondition> = serde_json::from_str(CONDITIONS)?;
        let conditions = conditions.into_iter().map(Condition::from).collect();
        let spells: Vec<DatabaseSpell> = serde_json::from_str(SPELLS)?;
        let spells = spells
            .into_iter()
            .map(|f| Spell::try_from_database(f, sheet))
            .filter_map(std::result::Result::ok)
            .collect();

        Ok(Self {
            items: vec![],
            spells,
            conditions,
        })
    }
}
