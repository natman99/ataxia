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
        "#;
    let c = i.execute(script, None).unwrap();
    assert_eq!(c.class[0].level, Level(2));
}
