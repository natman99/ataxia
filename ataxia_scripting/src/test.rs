use ataxia_types::class::{ClassType, Level};

use super::*;
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
fn race() {
    let mut i = Interface::new();
    let script = r#"
        race = "myrace"
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.race, "myrace");
}
#[test]
fn class() {
    let mut i = Interface::new();
    let script = r#"
        classes[0].set_level(2);
        classes[0].set_class("wizard");
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.class[0].level, Level(2));

    assert_eq!(c.class[0].class, ClassType::Wizard);
    assert_eq!(c.class[0].hit_dice, ClassType::Wizard.get_hit_dice());
    assert_eq!(c.class[0].max_healing_die, 2);

    let script = r#"
        classes[0].level = 19;

        classes[0].max_healing_die = 4;
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.class[0].max_healing_die, 4);
    assert_eq!(c.class[0].level.0, 19);
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
