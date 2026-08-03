use std::{
    str::FromStr,
    sync::{Arc, Mutex, RwLock},
};

use ataxia_types::{
    AbilityScores, ArmorClass, Character, HitPoints, Initiative, Inventory, Item, Score,
    WalkingSpeed,
    class::{Class, Classes, Level},
    condition::Conditions,
    damage::DamageType,
    database::spell,
    feat::Feat,
    feature::{Effect, Feature, Features, HasFeature},
    language::Languages,
    lore::Lore,
    meter::{Meter, Meters},
    roll::{Die, Roll},
    senses::Senses,
    skills::{Skill, Skills},
    source::{HasSource, Source},
    spells::{self, Area, Component, Damage, Heal, School, Spell, SpellType, Spells, Success},
};
use rhai::{Array, Dynamic, Engine, ImmutableString, Scope};

#[cfg(test)]
mod test;

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
        let name = String::new();
        let race = String::new();
        let classes: Classes = Classes::default();
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

        let _: () = self.engine.eval_with_scope(&mut scope, script)?;

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
        let class = scope.get_mut("classes")?.take().try_cast()?;
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

fn register_engine_functions(engine: &mut Engine) -> () {
    engine.register_fn("class", Class::new);

    register_spell_functions(engine);
    register_item_functions(engine);
    register_class_functions(engine);

    engine.register_fn("source", |f: &mut Feature, source: ImmutableString| {
        f.add_source(Source::Single(source.to_string()));
    });

    engine.register_fn("source", |f: &mut Feat, source: ImmutableString| {
        f.add_source(Source::Single(source.to_string()));
    });

    engine.register_fn("source", |f: &mut Meter, source: ImmutableString| {
        f.add_source(Source::Single(source.to_string()));
    });

    engine.register_fn("feature", |f: &mut Feat, feature: Effect| {
        f.add(feature);
    });

    engine.register_fn("standard_array", || -> Array {
        [8, 10, 12, 13, 14, 15]
            .iter()
            .map(|f| Dynamic::from_int(*f))
            .collect::<Vec<Dynamic>>()
            .into()
    });
}

fn register_spell_functions(engine: &mut Engine) {
    engine.register_fn("add", |spells: &mut Spells, s: Spell| {
        spells.spells.insert(s.name.clone(), s);
    });

    engine.register_fn("spell", |name: ImmutableString| -> Spell {
        Spell {
            name: name.to_string(),
            ..Default::default()
        }
    });

    engine.register_fn(
        "spell",
        |name: ImmutableString, description: ImmutableString| -> Spell {
            Spell {
                name: name.to_string(),
                desc: description.to_string(),
                ..Default::default()
            }
        },
    );

    engine.register_fn("source", |f: &mut Spell, source: ImmutableString| {
        f.add_source(Source::Single(source.to_string()));
    });

    engine.register_fn("component", |spell: &mut Spell, f: ImmutableString| {
        let Ok(c) = Component::from_str(f.as_str()) else {
            return;
        };
        spell.components.push(c);
    });

    engine.register_fn(
        "area",
        |spell: &mut Spell, shape: ImmutableString, size: i64| {
            spell.area = Some(Area {
                shape: shape.to_string(),
                size: size as i32,
            });
        },
    );

    engine.register_fn("desc", |spell: &mut Spell, desc: ImmutableString| {
        spell.desc = desc.to_string();
    });

    engine.register_fn("description", |spell: &mut Spell, desc: ImmutableString| {
        spell.desc = desc.to_string();
    });

    engine.register_fn(
        "cast_time",
        |spell: &mut Spell, cast_time: ImmutableString| {
            spell.cast_time = cast_time.to_string();
        },
    );

    engine.register_fn("range", |spell: &mut Spell, range: ImmutableString| {
        spell.range = range.to_string();
    });
    engine.register_fn(
        "duration",
        |spell: &mut Spell, duration: ImmutableString| {
            spell.duration = duration.to_string();
        },
    );

    engine.register_fn("concentration", |spell: &mut Spell, concentration: bool| {
        spell.concentration = concentration;
    });

    engine.register_fn("ritual", |spell: &mut Spell, ritual: bool| {
        spell.ritual = ritual;
    });

    engine.register_fn("level", |spell: &mut Spell, level: i64| {
        spell.level = level;
    });

    engine.register_fn("automatic", |spell: &mut Spell| {
        spell.spell_type = SpellType::Automatic;
    });

    engine.register_fn("melee", |spell: &mut Spell| {
        spell.spell_type = SpellType::Melee;
    });

    engine.register_fn("ranged", |spell: &mut Spell| {
        spell.spell_type = SpellType::Ranged;
    });

    engine.register_fn("saving", |spell: &mut Spell, dc_type: ImmutableString| {
        let Ok(s) = Score::from_str(dc_type.as_str()) else {
            return;
        };
        spell.spell_type = SpellType::Saving {
            dc_type: s,
            success: ataxia_types::spells::Success::Half,
        };
    });

    engine.register_fn(
        "saving",
        |spell: &mut Spell, dc_type: ImmutableString, success: ImmutableString| {
            let Ok(s) = Score::from_str(dc_type.as_str()) else {
                return;
            };
            let Ok(success) = Success::from_str(success.as_str()) else {
                return;
            };
            spell.spell_type = SpellType::Saving {
                dc_type: s,
                success,
            };
        },
    );

    engine.register_fn("damage", |spell: &mut Spell, damage: ImmutableString| {
        let Ok(r) = Roll::from_str(damage.as_str()) else {
            return;
        };
        spell.effect = ataxia_types::spells::Effect::Damage(Damage {
            damage: r,
            damage_type: DamageType::Force,
        });
    });

    engine.register_fn(
        "damage",
        |spell: &mut Spell, damage: ImmutableString, element: ImmutableString| {
            let Ok(r) = Roll::from_str(damage.as_str()) else {
                return;
            };

            let Ok(element) = DamageType::from_str(element.as_str()) else {
                return;
            };
            spell.effect = ataxia_types::spells::Effect::Damage(Damage {
                damage: r,
                damage_type: element,
            });
        },
    );

    engine.register_fn("element", |spell: &mut Spell, element: ImmutableString| {
        if let spells::Effect::Damage(d) = &mut spell.effect {
            let Ok(element) = DamageType::from_str(element.as_str()) else {
                return;
            };
            d.damage_type = element
        }
    });

    engine.register_fn("heal", |spell: &mut Spell, healing: ImmutableString| {
        let heal = match Roll::from_str(healing.as_str()) {
            Ok(r) => Heal::Roll(r),
            Err(_) => Heal::Static(healing.to_string()),
        };
        spell.effect = spells::Effect::Heal(heal);
    });

    engine.register_fn("school", |spell: &mut Spell, school: ImmutableString| {
        let Ok(s) = School::from_str(school.as_str()) else {
            return;
        };
        spell.school = s;
    });
}

fn register_item_functions(engine: &mut Engine) {
    engine.register_fn(
        "item",
        |name: ImmutableString, description: ImmutableString| -> Item {
            Item {
                name: name.to_string(),
                description: description.to_string(),
                ..Default::default()
            }
        },
    );

    engine.register_fn("item", |name: ImmutableString, roll: &Roll| Item {
        name: name.to_string(),
        roll: Some(roll.clone()),
        ..Default::default()
    });

    engine.register_fn(
        "item",
        |name: ImmutableString, description: ImmutableString, roll: &Roll| -> Item {
            Item {
                name: name.to_string(),
                description: description.to_string(),
                roll: Some(roll.clone()),
                ..Default::default()
            }
        },
    );

    engine.register_fn("desc", |f: &mut Item, desc: ImmutableString| {
        f.description = desc.to_string()
    });

    engine.register_fn("description", |f: &mut Item, desc: ImmutableString| {
        f.description = desc.to_string()
    });

    engine.register_fn("count", |f: &mut Item, count: i64| {
        f.quantity = count as i32;
    });

    engine.register_fn("quantity", |f: &mut Item, count: i64| {
        f.quantity = count as i32;
    });

    engine.register_fn("feature", |f: &mut Item, feature: Effect| {
        f.add(feature);
    });

    engine.register_fn("source", |f: &mut Item, source: ImmutableString| {
        f.add_source(Source::Single(source.to_string()));
    });
}

fn register_class_functions(engine: &mut Engine) {
    engine.register_fn("set_class", |class: &mut Class, s: ImmutableString| {
        class.set_class(s.as_str());
    });

    engine.register_fn("set_level", |class: &mut Class, level: i64| {
        class.set_level(level);
    });

    engine.register_fn("set_hit_die", |class: &mut Class, die: ImmutableString| {
        let Ok(d) = Die::from_str(die.as_str()) else {
            return;
        };
        class.hit_dice = d;
    });
}
