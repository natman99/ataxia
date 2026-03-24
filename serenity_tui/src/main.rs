mod terminal;

use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};

use serde::Serialize;
use serenity_types::{Item, sheet};

use sheet::HitPoints;
use sheet::{AbilityScores, Character, class::Class, skills::Skill};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    path: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<CliCommands>,
}

#[derive(Subcommand, Clone, Debug)]
enum CliCommands {
    New { name: String },
    Load { path: PathBuf },
    // Template {
    //     template: serenity_types::builder::ElementType,
    // },
}

fn main() {
    let cli = Cli::parse();

    // if let Some(path) = cli.path {
    //     let file = fs::read_to_string(path).expect("Failed to read file.");
    //     let sheet = serde_json::from_str(&file).expect("Failed to parse file.");
    //     let mut app = App::new(sheet);
    //     ratatui::run(|terminal| app.run(terminal)).expect("App crashed");
    // }

    if let Some(command) = cli.command {
        match command {
            CliCommands::New { name } => {
                let level = 1;
                let health = HitPoints::new(8, sheet::roll::Die::D8);
                let class = Class::default();
                let ability_scores = AbilityScores::default();
                let character = Character::new(name.clone(), level, health, class, ability_scores);

                let string =
                    serde_json::to_string_pretty(&character).expect("Failed to write character.");

                let path = format!("./{}.json", name);
                fs::write(path, &string).expect("Failed to save file.");
            }
            CliCommands::Load { path } => {
                let text = fs::read_to_string(&path).expect("Failed to read file");

                let c: Character = serde_json::from_str(&text).expect("Failed to parse file");
            }
        }
    } else {
        println!("No command found. Run --help for more.");
    }
}

fn default_save<T: Default + Serialize>(p: &str) -> anyhow::Result<()> {
    let t = T::default();

    let json = serde_json::to_string_pretty(&t)?;

    fs::write(p, json)?;

    Ok(())
}

// #[derive(Default)]
// struct Terminal<'a> {
//     text: String,
//     output: Vec<Line<'a>>,
// }

// impl<'a> Terminal<'a> {
//     pub fn complete(&mut self) {
//         let opt = Skill::ALL_STR;

//         let split = self.text.split(" ").collect::<Vec<&str>>();
//         let target = split.last();

//         if let Some(w) = target {
//             let complete = opt.iter().find(|f| f.starts_with(*w));

//             if let Some(complete) = complete {
//                 self.text = self.text.replace(w, *complete);
//             }
//         }
//     }
// }
