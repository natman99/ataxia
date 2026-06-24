use clap::clap_derive::{Args, Parser, Subcommand, ValueEnum};
fn main() {}

struct App {}

#[derive(Parser)]
struct Cli {
    #[arg(value_enum)]
    kind: Kind,
}

#[derive(ValueEnum, Debug, Clone)]
enum Kind {
    Spell,
    Item,
    Feature,
}
