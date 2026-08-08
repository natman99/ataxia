use std::sync::{Arc, Mutex};

use ataxia_types::{
    AbilityScores, ArmorClass, Character, HitPoints, Initiative, Inventory, WalkingSpeed,
    class::{Class, Classes},
    condition::Conditions,
    feat::Feat,
    feature::Features,
    language::Languages,
    lore::Lore,
    meter::Meters,
    senses::Senses,
    skills::Skills,
    spells::{Spell, Spells},
};
use rhai::{Engine, Module, Scope, combine_with_exported_module};

#[cfg(test)]
mod test;

mod library;

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

        register_engine_functions(&mut engine);

        Interface {
            engine: engine,
            spells: Default::default(),
            conditions: Default::default(),
            result,
        }
    }

    pub fn execute(
        &mut self,
        script: &str,
        character: Option<Character>,
    ) -> anyhow::Result<Character> {
        let Character {
            name,
            race,
            classes,
            initiative,
            armor_class,
            health,
            inventory,
            ability_scores,
            skills,
            saving_throws,
            spells,
            senses,
            features,
            ability_modifier,
            meters,
            lore,
            walking_speed,
            languages,
            conditions,
        } = Character::default();

        let armor_class = armor_class.0;
        let walking_speed = walking_speed.0;

        let mut scope = Scope::new();
        scope.push("name", name);
        scope.push("race", race);
        scope.push("classes", classes);
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
        scope.push("saving_throws", saving_throws);

        // We should run automated additions including adding meters, abilities, calculating health, and applying item stats.
        scope.push("calculate", true);

        let _: () = self.engine.eval_with_scope(&mut scope, script)?;

        let mut c = self.extract_scope(&mut scope)?;

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

    fn extract_scope(&self, scope: &mut Scope) -> anyhow::Result<Character> {
        macro_rules! extract {
            ($key:expr, $label:expr) => {{
                let var = scope
                    .get_mut($key)
                    .ok_or_else(|| anyhow::anyhow!("missing variable: {}", $label))?;
                var.take()
                    .try_cast()
                    .ok_or_else(|| anyhow::anyhow!("failed to cast variable: {}", $label))
            }};
        }

        let name = extract!("name", "name")?;
        let race = extract!("race", "race")?;
        let classes = extract!("classes", "classes")?;
        let initiative = extract!("initiative", "initiative")?;
        let armor_class_raw: i64 = extract!("armor_class", "armor_class")?;
        let mut armor_class = ArmorClass(armor_class_raw);
        armor_class.set(armor_class_raw);

        let health = extract!("health", "health")?;
        let inventory = extract!("inventory", "inventory")?;
        let ability_scores = extract!("ability_scores", "ability_scores")?;
        let skills = extract!("skills", "skills")?;
        let spells = extract!("spells", "spells")?;
        let senses = extract!("senses", "senses")?;
        let features = extract!("features", "features")?;
        let ability_modifier = extract!("ability_modifier", "ability_modifier")?;
        let meters = extract!("meters", "meters")?;
        let lore = extract!("lore", "lore")?;
        let walking_speed_raw: i64 = extract!("walking_speed", "walking_speed")?;
        let walking_speed = WalkingSpeed(walking_speed_raw);
        let languages = extract!("languages", "languages")?;
        let conditions = extract!("conditions", "conditions")?;
        let saving_throws = extract!("saving_throws", "saving_throws")?;

        Ok(Character {
            name,
            race,
            classes,
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
            saving_throws,
        })
    }
}

fn register_engine_types(engine: &mut Engine) {
    engine.build_type::<Character>();
    engine.build_type::<AbilityScores>();
    engine.build_type::<ArmorClass>();
    engine.build_type::<HitPoints>();
    engine.build_type::<Initiative>();
    engine.build_type::<Inventory>();
    engine.build_type::<WalkingSpeed>();
    engine.build_type::<Classes>();
    engine.build_type::<Class>();
    engine.build_type::<Conditions>();
    engine.build_type::<Features>();
    engine.build_type::<Languages>();
    engine.build_type::<Lore>();
    engine.build_type::<Meters>();
    engine.build_type::<Senses>();
    engine.build_type::<Skills>();
    engine.build_type::<Spells>();
    engine.build_type::<Spell>();
    engine.build_type::<Feat>();

    engine.register_indexer_get_set(Classes::get, Classes::set);
}

fn register_engine_functions(engine: &mut Engine) {
    let mut module = Module::new();
    combine_with_exported_module!(&mut module, "ataxia", library::library);
    engine.register_global_module(module.into());
}
