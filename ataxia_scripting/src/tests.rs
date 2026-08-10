use std::str::FromStr;

use ataxia_types::{
    Character, Item, Score,
    class::{ClassType, Level},
    damage::DamageType,
    language::Language,
    lore::{Morality, Order},
    meter::{Meter, RestoreTime},
    roll::Roll,
    skills::Skill,
    source::Source,
    spells::{self, Area, Component, Damage, School, Spell, SpellType},
};

use super::Interface;
#[test]
fn name() {
    let mut i = Interface::new();
    let script = r#"
        name = "hello"
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.name, "hello");
}

#[test]
fn class() {
    let mut i = Interface::new();
    let script = r#"
        classes[0].set_level(2);
        classes[0].set_class("wizard");
        classes[0].set_subclass("School of Necromancy");
        classes[0].subclass = "pie subclass";
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.classes[0].level, Level(2));

    assert_eq!(c.classes[0].class, ClassType::Wizard);
    assert_eq!(c.classes[0].hit_dice, ClassType::Wizard.get_hit_dice());
    assert_eq!(c.classes[0].max_healing_die, 2);
    assert_eq!(c.classes[0].subclass, "pie subclass");

    let script = r#"
        classes[0].level = 19;

        classes[0].max_healing_die = 4;
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.classes[0].max_healing_die, 4);
    assert_eq!(c.classes[0].level.0, 19);
}

#[test]
fn spell() {
    let mut i = Interface::new();

    let script = r#"
        let s = spell("Magic Missile");
        s.source("Tasha's");
        s.component("V");
        s.area("Cube", 12);
        s.desc("Goes boom");
        s.cast_time = "1 action";
        s.cast_time("1 action");
        s.duration("Instant");
        s.level(3);
        s.level = 12;

        s.concentration(true);
        s.concentration = false;
        s.ritual = true;
        s.melee();
        s.damage("2d4", "fire");
        s.element("Acid");
        s.school("Necromancy");
        s.range("12 toes");
        spells.add(s);
        "#;

    let c = i.execute(script, None).unwrap();
    let spell = c.spells.spells.get("Magic Missile").unwrap();
    let s = Spell {
        name: "Magic Missile".to_string(),
        desc: "Goes boom".to_string(),
        range: "12 toes".to_string(),
        cast_time: "1 action".to_string(),
        concentration: false,
        duration: "Instant".to_string(),
        ritual: true,
        level: 12,
        spell_type: SpellType::Melee,
        effect: spells::Effect::Damage(Damage {
            damage_type: DamageType::Acid,
            damage: Roll::from_str("2d4").unwrap(),
        }),
        school: School::Necromancy,
        area: Some(Area {
            shape: "Cube".to_string(),
            size: 12,
        }),
        components: vec![Component::from_str("V").unwrap()],
        source: Some(Source::Single("Tasha's".to_string())),
    };
    assert_eq!(&s, spell);
}
#[test]
fn scores() {
    let mut i = Interface::new();
    let script = r#"
        ability_scores.str = 14;
        ability_scores.dex = 12;
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.ability_scores.str.get(), 14i64);
    assert_eq!(c.ability_scores.dex.get(), 12i64);
}

#[test]
fn health() {
    let mut i = Interface::new();
    let script = r#"
        health.max = 14;
        health.current = 12;
        health.level_bonus = 2;
        health.set_max(13);
        health.set_current(11);
        health.set_level_bonus(1);
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.health.max, 13);
    assert_eq!(c.health.current, 11);
    assert_eq!(c.health.level_bonus, 1);
}

#[test]
fn race() {
    let mut i = Interface::new();
    let script = r#"
        race = "elf";
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.race, "elf");
}

#[test]
fn initiative() {
    let mut i = Interface::new();
    let script = r#"
        initiative.bonus = 2;
        initiative.set_bonus(1);
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.initiative.bonus, 1);
}

#[test]
fn armor_class() {
    let mut i = Interface::new();
    let script = r#"
        armor_class = 12;
        armor_class += 2;
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.armor_class.get(), 14);
}
#[test]
fn inventory() {
    let mut i = Interface::new();
    let script = r#"
        let i = item("cool item", roll("2d4"));
        i.desc("a really cool item");
        i.count(2);
        i.source("book of cool items");
        inventory.add(i);

        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(
        c.inventory.0.get("cool item").unwrap(),
        &Item {
            name: "cool item".to_string(),
            description: "a really cool item".to_string(),
            quantity: 2,
            roll: Some(Roll::from_str("2d4").unwrap()),
            source: Some(Source::Single("book of cool items".to_string())),
            feature: None
        }
    );
}

#[test]
fn meter() {
    let mut i = Interface::new();
    let script = r#"
        let m = meter("first", 2);
        m.short_rest();
        m.source("cake power");
        meters.add(m);
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(
        c.meters.meters["first"],
        Meter {
            name: "first".to_string(),
            slot_number: 2,
            spent: 0,
            restore: RestoreTime {
                short_rest: true,
                long_rest: true
            },
            source: Some(Source::Single("cake power".to_string()))
        }
    );
}
#[test]
fn saving_throws() {
    let mut i = Interface::new();
    let script = r#"
        saving_throws.str = true;
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.saving_throws.str, true);
}

#[test]
fn walking_speed() {
    let mut i = Interface::new();
    let script = r#"
        walking_speed = 40;
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.walking_speed.0, 40);
}

#[test]
fn skills() {
    let mut i = Interface::new();
    let script = r#"
        skills.add("deception");
        skills.add("persuasion");

        skills.expertise("acrobatics");
        skills.proficiency_bonus = 4;

        skills.overrides.set("acrobatics", "str");

        "#;
    let c = i.execute(script, None).unwrap();
    assert!(c.skills.proficiencies.contains(Skill::Deception));

    assert!(c.skills.proficiencies.contains(Skill::Persuasion));

    assert!(c.skills.proficiencies.contains(Skill::Acrobatics));
    assert!(c.skills.expertise.contains(Skill::Acrobatics));

    assert_eq!(c.skills.proficiency_bonus, 4);
    assert_eq!(c.skills.overrides.acrobatics, Some(Score::Str));
}

#[test]
fn senses() {
    let mut i = Interface::new();
    let script = r#"
        senses.darkvision = true;
        senses.tremor_sense = true;
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.senses.extra.dark_vision, true);
    assert_eq!(c.senses.extra.tremor_sense, true);
}

#[test]
fn interactive() {
    let i = Interface::new();
    let mut character = Character::default();
    character.ability_scores.con.base = 14;
    let s = i.interactive("2 + con.mod()", character).unwrap();

    assert_eq!(s.to_string(), "4");
}

#[test]
fn interactive_roll() {
    let i = Interface::new();
    let mut character = Character::default();
    character.ability_scores.con.base = 14;
    let s = i.interactive("2 + 2d4 + str.mod()", character).unwrap();
    assert!(s.is_int())
}

#[test]
fn ability_modifier() {
    let mut i = Interface::new();
    let script = r#"
        ability_modifier = dex;
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.ability_modifier, Score::Dex);
}

#[test]
fn lore() {
    let mut i = Interface::new();
    let script = r#"
        lore.backstory = "likes to do cool things";
        lore.personality_traits = "haha loves cake!";
        lore.allies = "bakers, cool people";
        lore.enemies = "people without rad sunglasses";
        lore.physical_traits = "rad sunglasses";
        lore.alignment.order = "lawful";
        lore.alignment.morality = "evil";
        lore.alignment.set("neutral", "neutral");
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.lore.backstory, "likes to do cool things");

    assert_eq!(c.lore.backstory, "likes to do cool things");
    assert_eq!(c.lore.personality_traits, "haha loves cake!");
    assert_eq!(c.lore.allies, "bakers, cool people");
    assert_eq!(c.lore.enemies, "people without rad sunglasses");
    assert_eq!(c.lore.physical_traits, "rad sunglasses");
    assert_eq!(c.lore.alignment.order, Order::Neutral);
    assert_eq!(c.lore.alignment.morality, Morality::Neutral);
}

#[test]
fn languages() {
    let mut i = Interface::new();
    let script = r#"
        languages.add("elvish");
        languages.add("spanish");
        languages.add("orcish");
        "#;
    let c = i.execute(script, None).unwrap();
    assert!(c.languages.languages.contains(&Language::Elvish));

    assert!(c.languages.languages.contains(&Language::Orcish));

    assert!(
        c.languages
            .languages
            .contains(&Language::Other("spanish".to_string()))
    )
}
