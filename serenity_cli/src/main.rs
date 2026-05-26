use parking_lot::Mutex;
use std::{
    cell::{LazyCell, RefCell},
    collections::HashMap,
    fs, io,
    path::PathBuf,
    sync::Arc,
};

use clap::{Parser, Subcommand};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyEvent, KeyModifiers},
    },
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::{Block, Paragraph, Table},
};
use serde::Serialize;
use serenity_types::{Character, database::spell::Spell};
use tui_input::{Input, backend::crossterm::EventHandler};

use crate::{
    command::{CommandHandler, make_command_handler},
    data::Data,
    widgets::State,
};

mod command;
mod commands;
mod data;
mod search;
mod widgets;

const COMMAND_HANDLER: LazyCell<Mutex<CommandHandler>> =
    LazyCell::new(|| Mutex::new(make_command_handler()));

#[derive(clap::Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    subcommand: Option<Command>,
    path: Option<PathBuf>,
}
#[derive(clap::Subcommand, Clone, Debug)]
enum Command {
    New { name: String },
    Generate { kind: GenerateKind },
}
#[derive(clap::ValueEnum, Debug, Clone, Copy)]
enum GenerateKind {
    Spell,
    Item,
    Feature,
}

fn generate<T: Serialize + Default>() -> Result<String, serde_json::Error> {
    let t = T::default();
    let s = serde_json::to_string_pretty(&t)?;
    Ok(s)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    println!("Hello, world!");

    let sheet = {
        if let Some(command) = cli.subcommand {
            match command {
                Command::New { name } => {
                    let mut c = Character::default();
                    c.name = name;
                    Some(c)
                }
                Command::Generate { kind } => {
                    let string = match kind {
                        GenerateKind::Spell => generate::<serenity_types::database::spell::Spell>(),
                        GenerateKind::Item => generate::<serenity_types::Item>(),
                        GenerateKind::Feature => generate::<serenity_types::feature::Feature>(),
                    };
                    let string = string.unwrap();
                    println!("{}", string);
                    return Ok(());
                }
            }
        } else if let Some(ref path) = cli.path {
            let str = fs::read_to_string(path)?;

            let sheet = serde_json::from_str(&str)?;
            Some(sheet)
        } else {
            None
        }
    };

    if sheet.is_none() {
        println!("Expected command or path.");
        return Ok(());
    }
    let sheet = sheet.unwrap();
    let str = sheet.name.clone() + ".json";
    let path = cli.path.unwrap_or(PathBuf::from(str));

    let spells = {
        let s = fs::read_to_string(PATH).unwrap();
        let s: Vec<Spell> = serde_json::from_str(&s).unwrap();
        s
    };

    let mut app = App::new(sheet, path, spells);
    ratatui::run(|term| app.run(term))?;
    Ok(())
}
#[derive(Debug, Default, Clone)]
struct SpellsBuffer<'a> {
    pub len: usize,
    pub buff: Table<'a>,
}

#[derive(Debug)]
struct App {
    input: Input,
    exit: bool,
    sheet: Character,
    state: State,
    path: PathBuf,
    command_output: String,
    data: Data,
}

impl App {
    pub fn new(sheet: Character, path: PathBuf, spells: Vec<Spell>) -> Self {
        let spells = Arc::new(spells);
        Self {
            input: Input::new("".to_string()),
            exit: false,
            state: State::default(),
            sheet,
            path,
            command_output: String::new(),
            spells,
        }
    }
    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
            if self.exit {
                let s = serde_json::to_string_pretty(&self.sheet).expect("Serde error");
                fs::write(&self.path, s)?;

                break Ok(());
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let (area, output, text_box_frame) = {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Max(1), Constraint::Max(3)]);
            let split = layout.split(frame.area());
            (split[0], split[1], split[2])
        };

        self.state.render(frame, area, &self.sheet);
        frame.render_widget(Paragraph::new(self.command_output.as_str()), output);
        self.render_input(frame, text_box_frame);
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        let width = area.width.max(3) - 1;
        let scroll = self.input.visual_scroll(width as usize);
        let style = Color::Yellow;
        let input = Paragraph::new(self.input.value())
            .style(style)
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, area);

        let x = self.input.visual_cursor().max(scroll) - scroll + 1;
        frame.set_cursor_position((area.x + x as u16, area.y + 1));
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let event = crossterm::event::read()?;
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                self.input.reset();
                return Ok(());
            }
            match code {
                KeyCode::Esc => self.exit = true,
                KeyCode::Enter => self.execute_command(),
                _ => {
                    self.input.handle_event(&event);
                }
            }
        }
        Ok(())
    }

    fn execute_command(&mut self) {
        let cmd = self.input.value_and_reset();
        match COMMAND_HANDLER.lock().execute(&cmd, self) {
            Ok(()) => (),
            Err(e) => self.command_output = e.to_string(),
        }
    }
}
