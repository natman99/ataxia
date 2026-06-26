use std::fmt::Debug;

use thiserror::Error;

use crate::{App, commands};

macro_rules! command {
    ($key:literal, $command:expr) => {
        Box::new(Command {
            key: $key.to_string(),
            function: Box::new($command),
        })
    };
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Unknown command")]
    Unknown,
    #[error("Substring matches for multiple commands")]
    MultipleMatches(Vec<String>),
    #[error("Invalid arguments")]
    ArgumentError,
    #[error("Could not find target")]
    NotFound,
    #[error("Command Error: {0}")]
    CommandError(Box<dyn std::error::Error>),
}

pub fn to_command_error<T: std::error::Error + 'static>(f: T) -> CommandError {
    CommandError::CommandError(Box::new(f))
}

pub type Result<T> = std::result::Result<T, CommandError>;
#[derive(Debug)]
pub struct CommandHandler {
    commands: Vec<Box<Command>>,
}

impl CommandHandler {
    pub fn new() -> Self {
        CommandHandler { commands: vec![] }
    }

    pub fn push(&mut self, command: Box<Command>) {
        self.commands.push(command);
    }
    /// Returns the command index if there is only one option.
    pub fn find(&self, substring: &str) -> Result<usize> {
        let split = substring.split(" ").collect::<Vec<&str>>();
        let Some(substring) = split.get(0) else {
            return Err(CommandError::Unknown);
        };

        let res = self
            .commands
            .iter()
            .enumerate()
            .filter(|(_, f)| f.key.starts_with(substring))
            .collect::<Vec<(usize, &Box<Command>)>>();
        if res.len() == 1 {
            Ok(res[0].0)
        } else if res.is_empty() {
            Err(CommandError::Unknown)
        } else {
            let opts = res
                .iter()
                .map(|f| &f.1.key)
                .cloned()
                .collect::<Vec<String>>();
            Err(CommandError::MultipleMatches(opts))
        }
    }

    /// Get command name from index. For autocomplete.
    pub fn get_name(&self, idx: usize) -> Option<&str> {
        self.commands.get(idx).map(|f| f.key.as_str())
    }

    pub fn execute(&mut self, command: &str, state: &mut App) -> Result<()> {
        let idx = self.find(command)?;
        let cmd = &mut self.commands[idx];
        let f = &mut cmd.function;
        f(command, state)?;

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<Command>> {
        self.commands.iter()
    }

    pub fn command(mut self, command: Box<Command>) -> CommandHandler {
        self.commands.push(command);
        self
    }
}
pub struct Command {
    key: String,
    function: Box<dyn FnMut(&str, &mut App) -> Result<()>>,
}

impl Command {
    pub fn key(&self) -> &str {
        self.key.as_str()
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command").field("key", &self.key).finish()
    }
}

/// Populate the command handler.
pub fn make_command_handler() -> CommandHandler {
    let handler = CommandHandler::new()
        .command(command!("add", commands::add))
        .command(command!("buff", commands::buff))
        .command(command!("damage", commands::damage))
        .command(command!("heal", commands::heal))
        .command(command!("help", commands::help))
        .command(command!("inventory", commands::inventory))
        .command(command!("m", commands::g_main))
        .command(command!("main", commands::g_main))
        .command(command!("meter", commands::meter))
        .command(command!("remove", commands::remove))
        .command(command!("restore", commands::restore))
        .command(command!("search", commands::search))
        .command(command!("spells", commands::spell))
        .command(command!("spend", commands::spend))
        .command(command!("tick", commands::tick))
        .command(command!("create", commands::create));

    handler
}
