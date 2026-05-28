use std::fs;

use serenity_types::Character;

fn main() {
    // let s = std::fs::read_to_string(r"../5e-database\src\2014\5e-SRD-Spells.json").unwrap();

    // let s: Vec<database::spell::Spell> = serde_json::from_str(&s).unwrap();

    // let s = std::fs::read_to_string(r"../5e-database\src\2014\5e-SRD-Races.json").unwrap();

    // let s: Vec<database::race::Race> = serde_json::from_str(&s).unwrap();

    // let f = Character::default();

    // let s = serde_json::to_string_pretty(&f).unwrap();

    // std::fs::write("test.json", s).unwrap();

    // let mut f = HitPoints::default();
    // f.level_bonus = 1;
    // let mut scores = AbilityScores::default();
    // scores.con = AbilityScore::new(15, 0);
    // let f = f.fixed(Level(4), serenity_types::roll::Die::D6, &scores);

    // dbg!("{:?}", f);
    //

    let f = Character::default();
    let s = serde_json::to_string_pretty(&f).unwrap();

    fs::write("test.json", s).unwrap();
}
