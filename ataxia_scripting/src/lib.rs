use std::sync::{Arc, Mutex, RwLock};

use ataxia_types::{
    AbilityScores, ArmorClass, Character, HitPoints, Initiative, Inventory, Score, WalkingSpeed,
    class::{Class, Classes},
    condition::Conditions,
    feature::Features,
    language::Languages,
    lore::Lore,
    meter::Meters,
    senses::Senses,
    skills::{Skill, Skills},
    spells::{Spell, Spells},
};
use rhai::{Engine, ImmutableString, Scope};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Command {
    FixedHp,
}

#[derive(Debug)]
pub struct Interface {
    engine: Engine,
    pub spells: Spells,
    pub conditions: Spells,
    pub result: Arc<Mutex<String>>,
}

impl Interface {
    pub fn new() -> Self {
        let mut engine = Engine::new();
        let result = Arc::new(Mutex::new(String::new()));

        let r = result.clone();

        engine.on_print(move |f| r.lock().unwrap().push_str(f));

        register_engine_types(&mut engine);

        Interface {
            engine: engine,
            spells: Default::default(),
            conditions: Default::default(),
            result,
        }
    }

    pub fn execute(&mut self, character: Option<Character>) -> anyhow::Result<Character> {
        let name = String::new();
        let race = String::new();
        let class = Classes::default();
        let initiative = Initiative::default();
        let armor_class = ArmorClass::default();
        let health = HitPoints::default();
        let inventory = Inventory::default();
        let ability_scores = AbilityScores::default();
        let skills = Skills::default();
        let spells = Spells::default();
        let senses = Senses::default();
        let features = Features::default();
        let ability_modifier = Score::default();
        let meters = Meters::default();
        let lore = Lore::default();
        let walking_speed = WalkingSpeed::default();
        let languages = Languages::default();
        let conditions = Conditions::default();

        let mut scope = Scope::new();
        scope.push("name", name);
        scope.push("race", race);
        scope.push("class", class);
        scope.push("initiative", initiative);
        scope.push("armor_class", armor_class);
        scope.push("health", health);
        scope.push("inventory", inventory);
        scope.push("ability_scores", ability_scores);
        scope.push("skills", skills);
        scope.push("spells", spells);
        scope.push("senses", senses);
        scope.push("features", features);
        scope.push("ability_modifier", ability_modifier);
        scope.push("meters", meters);
        scope.push("lore", lore);
        scope.push("walking_speed", walking_speed);
        scope.push("languages", languages);
        scope.push("conditions", conditions);

        let mut c = self
            .extract_scope(&mut scope)
            .ok_or(anyhow::anyhow!("Failed to extract scope."))?;

        if let Some(char) = character {
            c.health.current = char.health.current;
            for (k, i) in char.meters.meters.iter() {
                if let Some(meter) = c.meters.meters.get_mut(k) {
                    meter.spent = i.spent;
                }
            }
        }

        Ok(c)
    }

    fn extract_scope(&self, scope: &mut Scope) -> Option<Character> {
        let name = scope.get_mut("name")?.take().try_cast()?;
        let race = scope.get_mut("race")?.take().try_cast()?;
        let class = scope.get_mut("class")?.take().try_cast()?;
        let initiative = scope.get_mut("initiative")?.take().try_cast()?;
        let armor_class = scope.get_mut("armor_class")?.take().try_cast()?;

        let health = scope.get_mut("health")?.take().try_cast()?;
        let inventory = scope.get_mut("inventory")?.take().try_cast()?;
        let ability_scores = scope.get_mut("ability_scores")?.take().try_cast()?;
        let skills = scope.get_mut("skills")?.take().try_cast()?;
        let spells = scope.get_mut("spells")?.take().try_cast()?;
        let senses = scope.get_mut("senses")?.take().try_cast()?;
        let features = scope.get_mut("features")?.take().try_cast()?;
        let ability_modifier = scope.get_mut("ability_modifier")?.take().try_cast()?;
        let meters = scope.get_mut("meters")?.take().try_cast()?;
        let lore = scope.get_mut("lore")?.take().try_cast()?;
        let walking_speed = scope.get_mut("walking_speed")?.take().try_cast()?;
        let languages = scope.get_mut("languages")?.take().try_cast()?;
        let conditions = scope.get_mut("conditions")?.take().try_cast()?;

        Some(Character {
            name,
            race,
            class,
            initiative,
            armor_class,
            health,
            inventory,
            ability_scores,
            skills,
            spells,
            senses,
            features,
            ability_modifier,
            meters,
            lore,
            walking_speed,
            languages,
            conditions,
        })
    }
}

fn register_engine_functions(engine: &mut Engine) -> _ {
    todo!()
}

fn register_engine_types(engine: &mut Engine) {
    engine.build_type::<Character>();
    engine.build_type::<AbilityScores>();
    engine.build_type::<ArmorClass>();
    engine.build_type::<HitPoints>();
    engine.build_type::<Initiative>();
    engine.build_type::<Inventory>();
    engine.build_type::<Score>();
    engine.build_type::<WalkingSpeed>();
    engine.build_type::<Classes>();
    engine.build_type::<Conditions>();
    engine.build_type::<Features>();
    engine.build_type::<Languages>();
    engine.build_type::<Lore>();
    engine.build_type::<Meters>();
    engine.build_type::<Senses>();
    engine.build_type::<Skills>();
    engine.build_type::<Skill>();
    engine.build_type::<Spells>();
    engine.build_type::<Spell>();
}
