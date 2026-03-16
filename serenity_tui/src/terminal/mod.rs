use crate::sheet::{roll::Roll, skills::Skill};

#[derive(Debug)]

pub struct TerminalCommands {
    // command: Command,
    action: Option<Action>,
    skill: Option<Skill>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Get,
    Set,
}

impl TryFrom<&str> for Action {
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "get" => Ok(Self::Get),
            "set" => Ok(Self::Set),
            _ => Err(()),
        }
    }
}

pub enum Cmd {
    Get(Skill),
    Set { skill: Skill, arg: i32 },
    Roll(Roll),
}

fn parse_skill(skill: Skill, tokens: &Vec<&str>) -> Result<Cmd, String> {
    let second = tokens.get(1);
    let third = tokens.get(2);

    let mut action: Option<Action> = None;
    let mut arg: Option<i32> = None;

    if let Some(second) = second {
        action = Action::try_from(*second).ok();
    }

    if let Some(num) = third
        && let Ok(i) = num.parse::<i32>()
    {
        arg = Some(i);
    }

    match (action, arg) {
        // no skill, error
        (None, None) => Ok(Cmd::Get(skill)),
        (Some(action), _) => {
            let arg = arg.unwrap_or(0);
            match action {
                Action::Get => Ok(Cmd::Get(skill)),
                Action::Set => Ok(Cmd::Set { skill, arg }),
            }
        }
        _ => Err("Invalid command".to_string()),
    }
}

pub fn parse(s: &str) -> Result<Cmd, String> {
    let tokens = s.split(" ").collect::<Vec<&str>>();

    let first = tokens.get(0);

    if first.is_none() {
        return Err("Unknown command".to_string());
    }
    let first = first.unwrap();

    if let Ok(skill) = Skill::try_from(*first) {
        return parse_skill(skill, &tokens);
    }

    if *first == "roll"
        && let Some(second) = tokens.get(1)
        && let Ok(roll) = Roll::try_from(*second)
    {
        return Ok(Cmd::Roll(roll));
    }

    Err("Unknown command".to_string())
}
