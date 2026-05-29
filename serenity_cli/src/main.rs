use parking_lot::Mutex;
use schemars::JsonSchema;
use std::{cell::LazyCell, fs, io, path::PathBuf};

use clap::Parser;
use ratatui::crossterm;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{
        DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::{Block, Paragraph, ScrollbarState},
};
use serde::Serialize;
use serenity_types::Character;
use tui_input::{Input, backend::crossterm::EventHandler};

use crate::{
    command::{CommandError, CommandHandler, make_command_handler},
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
    Schema { kind: GenerateKind },
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

fn generate_schema<T: JsonSchema + Default>() -> Result<String, serde_json::Error> {
    let t = schemars::schema_for!(T);
    let s = serde_json::to_string_pretty(&t)?;
    Ok(s)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

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
                Command::Schema { kind } => {
                    let string = match kind {
                        GenerateKind::Spell => {
                            generate_schema::<serenity_types::database::spell::Spell>()
                        }
                        GenerateKind::Item => generate_schema::<serenity_types::Item>(),
                        GenerateKind::Feature => {
                            generate_schema::<serenity_types::feature::Feature>()
                        }
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
#[derive(Debug, Default)]
struct FocusedContent {
    // items: Vec<String>,
    buf: String,
    scroll: ScrollbarState,
}

impl FocusedContent {
    pub fn new(content: String) -> Self {
        let scroll = ScrollbarState::new(content.split("\n").count()).viewport_content_length(3);
        Self {
            // items: content,
            buf: content,
            scroll,
        }
    }
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
    focused_content: FocusedContent,
    prev_commands: Vec<String>,
    prev_command_idx: usize,
}

impl App {
    pub fn new(sheet: Character, path: PathBuf) -> Self {
        let data = Data::new().unwrap();

        Self {
            input: Input::new("".to_string()),
            exit: false,
            state: State::default(),
            sheet,
            path,
            command_output: String::new(),
            data,
            focused_content: Default::default(),
            prev_commands: Default::default(),
            prev_command_idx: 0,
        }
    }
    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        crossterm::execute!(terminal.backend_mut(), EnableMouseCapture)?;

        loop {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
            if self.exit {
                let s = serde_json::to_string_pretty(&self.sheet).expect("Serde error");
                fs::write(&self.path, s)?;

                crossterm::execute!(terminal.backend_mut(), DisableMouseCapture)?;

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

        self.state
            .render(frame, area, &self.sheet, &mut self.focused_content);
        frame.render_widget(Paragraph::new(self.command_output.as_str()), output);
        self.render_input(frame, text_box_frame);
    }

    /// Render the input text box.
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
    /// Handle input.
    fn handle_events(&mut self) -> io::Result<()> {
        let event = crossterm::event::read()?;

        if let Event::Mouse(e) = event {
            if e.kind.is_scroll_down() {
                self.focused_content.scroll.next();
                self.focused_content.scroll.next();
                self.focused_content.scroll.next();
            }

            if e.kind.is_scroll_up() {
                self.focused_content.scroll.prev();
                self.focused_content.scroll.prev();
                self.focused_content.scroll.prev();
            }
        }

        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                self.input.reset();
                self.prev_command_idx = 0;
                return Ok(());
            }

            match code {
                KeyCode::Esc => self.exit = true,
                KeyCode::Enter => self.execute_command(),
                KeyCode::Up => {
                    if self.prev_commands.is_empty() {
                        return Ok(());
                    }
                    self.prev_command_idx = self
                        .prev_command_idx
                        .saturating_add(1)
                        .min(self.prev_commands.len());

                    let idx = self
                        .prev_commands
                        .len()
                        .saturating_sub(self.prev_command_idx);

                    self.input = self
                        .input
                        .clone()
                        .with_value(self.prev_commands[idx].clone());
                }
                KeyCode::Down => {
                    if self.prev_commands.is_empty() {
                        return Ok(());
                    }
                    self.prev_command_idx = self.prev_command_idx.saturating_sub(1);

                    let idx = self
                        .prev_commands
                        .len()
                        .saturating_sub(self.prev_command_idx);

                    if idx == self.prev_commands.len() {
                        self.input.reset();
                        self.prev_command_idx = 0;
                        return Ok(());
                    }

                    self.input = self
                        .input
                        .clone()
                        .with_value(self.prev_commands[idx].clone());
                }
                KeyCode::Tab => self.autocomplete(),
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
            Ok(()) => self.command_output.clear(),
            Err(e) => self.command_output = e.to_string(),
        }
        self.prev_command_idx = 0;
        self.prev_commands.push(cmd);
    }

    fn autocomplete(&mut self) {
        use command::Result;
        let val = self.input.value();

        if val.trim().contains(" ") {
            return;
        }

        let res: Result<usize> = COMMAND_HANDLER.lock().find(val);
        match res {
            Ok(n) => {
                let binding = COMMAND_HANDLER;
                let binding2 = binding.lock();
                let cmd = binding2.get_name(n);
                if let Some(v) = cmd {
                    self.input = self.input.clone().with_value(v.to_string());
                }
                self.command_output.clear();
            }
            Err(e) => match e {
                CommandError::MultipleMatches(opts) => {
                    self.command_output = opts.join(", ");
                }
                _ => (),
            },
        };
    }
}
