use std::fmt::Debug;

use thiserror::Error;

use crate::{App, commands, widgets::State};

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
    MultipleMatches,
    #[error("Invalid arguments")]
    ArgumentError,
    #[error("Could not find target")]
    NotFound,
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
            Err(CommandError::MultipleMatches)
        }
    }

    pub fn execute(&mut self, command: &str, state: &mut App) -> Result<()> {
        let idx = self.find(command)?;
        let cmd = &mut self.commands[idx];
        let f = &mut cmd.function;
        f(command, state)?;

        Ok(())
    }
}
pub struct Command {
    key: String,
    function: Box<dyn FnMut(&str, &mut App) -> Result<()>>,
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("key", &self.key)
            // .field("function", &self.function)
            .finish()
    }
}

/// Populate the command handler.
pub fn make_command_handler() -> CommandHandler {
    let mut handler = CommandHandler::new();
    handler.push(command!("spells", commands::spell));
    handler.push(command!("inventory", commands::inventory));
    handler.push(command!("main", commands::g_main));
    handler.push(command!("add", commands::add));

    handler
}
