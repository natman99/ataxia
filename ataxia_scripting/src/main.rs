use std::fs;

fn main() {
    // rhai::CustomType;
    let mut e = ataxia_scripting::Interface::new();

    let s = fs::read_to_string("test.rhai").unwrap();

    let c = e.execute(&s, None).unwrap();

    dbg!(c);
}
