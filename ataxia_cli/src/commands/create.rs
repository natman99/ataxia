use ataxia_types::{Item, database::spell::Spell, feature::Effect};
use itertools::Itertools;

use crate::{
    App,
    command::{self, CommandError},
    search::{self, filter},
};

pub fn create(cmd: &str, app: &mut App) -> command::Result<()> {
    let split = cmd.split(" ").collect::<Vec<&str>>();

    if let Some(kind) = split.get(1) {
        if ["spell", "feature", "item"].contains(kind) {
            let res = std::process::Command::new("ataxia_ui").arg(kind).output();
            match res {
                Ok(res) => {
                    let s = match String::from_utf8(res.stdout) {
                        Ok(s) => s,
                        Err(a) => return Err(CommandError::CommandError(Box::new(a))),
                    };
                    let s = s.trim();
                    match *kind {
                        "spell" => {
                            let j = serde_json::from_str::<Spell>(&s);
                            match j {
                                Ok(spell) => {
                                    app.sheet.spells.spells.insert(spell.name.clone(), spell);
                                }
                                Err(a) => return Err(CommandError::CommandError(Box::new(a))),
                            }
                        }

                        "item" => {
                            let j = serde_json::from_str::<Item>(&s);
                            match j {
                                Ok(item) => {
                                    app.sheet.inventory.0.insert(item.name.to_string(), item);
                                }
                                Err(a) => return Err(CommandError::CommandError(Box::new(a))),
                            }
                        }

                        "feature" => {
                            let j = serde_json::from_str::<Effect>(&s);
                            match j {
                                Ok(feature) => {
                                    app.sheet
                                        .features
                                        .inner
                                        .insert(feature.name.to_string(), feature);
                                }
                                Err(a) => return Err(CommandError::CommandError(Box::new(a))),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                Err(e) => return Err(CommandError::CommandError(Box::new(e))),
            }
        } else {
            return Err(CommandError::ArgumentError);
        }
    }

    Ok(())
}
