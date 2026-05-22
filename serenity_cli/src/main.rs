use std::{fs, io, path::PathBuf};

use clap::{Parser, Subcommand};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyEvent, KeyModifiers},
    },
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::{Block, Paragraph},
};
use serde::Serialize;
use serenity_types::Character;
use tui_input::{Input, backend::crossterm::EventHandler};

mod widgets;

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

    let mut app = App::new(sheet, path);
    ratatui::run(|term| app.run(term))?;
    Ok(())
}
#[derive(Debug)]
struct App {
    input: Input,
    exit: bool,
    sheet: Character,
    state: State,
    path: PathBuf,
}

impl App {
    pub fn new(sheet: Character, path: PathBuf) -> Self {
        Self {
            input: Input::new("".to_string()),
            exit: false,
            state: State::default(),
            sheet,
            path,
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
        let (area, text_box_frame) = {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Max(3)]);
            let split = layout.split(frame.area());
            (split[0], split[1])
        };

        frame.render_widget("hello world", area);
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
                KeyCode::Enter => return Ok(()),
                _ => {
                    self.input.handle_event(&event);
                }
            }
        }
        Ok(())
    }
}
