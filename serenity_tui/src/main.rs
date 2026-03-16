mod terminal;

use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use rand::SeedableRng;

use serde::Serialize;
use serenity_types::ability::Ability;
use serenity_types::{Item, sheet};

use serenity_types::r#trait::Feature;
use sheet::HitPoints;
use sheet::{AbilityScores, Character, class::Class, skills::Skill};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{self, event::KeyCode},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Padding, Paragraph, Row, Table, Wrap},
};

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

    if let Some(path) = cli.path {
        let file = fs::read_to_string(path).expect("Failed to read file.");
        let sheet = serde_json::from_str(&file).expect("Failed to parse file.");
        let mut app = App::new(sheet);
        ratatui::run(|terminal| app.run(terminal)).expect("App crashed");
    }

    if let Some(command) = cli.command {
        match command {
            CliCommands::New { name } => {
                let level = 1;
                let health = HitPoints::new(8);
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
            } // CliCommands::Template { template } => {
              //     match template {
              //         serenity_types::builder::ElementType::Item => {
              //             default_save::<Item>("item_template.json")
              //                 .expect("Failed to save file. Does one exist already?");
              //         }
              //         serenity_types::builder::ElementType::Spell => {
              //             default_save::<Spell>("spell_template.json")
              //                 .expect("Failed to save file. Does one exist already?");
              //         }
              //         serenity_types::builder::ElementType::Trait => {
              //             default_save::<Trait>("trait_template.json")
              //                 .expect("Failed to save file. Does one exist already?");
              //         }
              //         serenity_types::builder::ElementType::Ability => {
              //             default_save::<Ability>("ability_template.json")
              //                 .expect("Failed to save file. Does one exist already?");
              //         }
              //         serenity_types::builder::ElementType::Other(_) => (),
              //     }
              //     println!("Generated template");
              // }
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

#[derive(Default)]
struct Terminal<'a> {
    text: String,
    output: Vec<Line<'a>>,
}

impl<'a> Terminal<'a> {
    pub fn complete(&mut self) {
        let opt = Skill::ALL_STR;

        let split = self.text.split(" ").collect::<Vec<&str>>();
        let target = split.last();

        if let Some(w) = target {
            let complete = opt.iter().find(|f| f.starts_with(*w));

            if let Some(complete) = complete {
                self.text = self.text.replace(w, *complete);
            }
        }
    }
}

struct App<'a> {
    sheet: Character,
    counter: i32,
    input: bool,
    terminal: Terminal<'a>,
    quit: bool,
    rng: rand::rngs::StdRng,
}

impl<'a> App<'a> {
    pub fn new(sheet: Character) -> Self {
        Self {
            sheet: sheet,
            quit: false,
            counter: 0,
            input: false,
            terminal: Default::default(),
            rng: rand::make_rng(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        // textarea.set_cursor_line_style();
        //

        while !self.quit {
            terminal.draw(|frame| self.render(frame))?;
            let event = crossterm::event::read()?;
            match &event {
                // crossterm::event::Event::FocusGained => todo!(),
                // crossterm::event::Event::FocusLost => todo!(),
                crossterm::event::Event::Key(key_event) => {
                    if key_event.kind.is_release() {
                        // normal hotkeys
                        if !self.input {
                            match key_event.code {
                                KeyCode::Char('u') => self.counter += 1,
                                KeyCode::Char('k') => self.counter = 0,
                                KeyCode::Char('j') => self.counter -= 1,

                                KeyCode::Char('`') => {
                                    self.input = true;
                                    // skip the rest of events to avoid doubling terminal input.
                                    continue;
                                }

                                KeyCode::Char('q') => self.quit = true,

                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            }

            if self.input {
                let term = &mut self.terminal;
                if let Some(input) = event.as_key_release_event() {
                    match input.code {
                        KeyCode::Esc => self.input = false,
                        KeyCode::Enter => self.submit(),
                        KeyCode::Backspace => _ = term.text.pop(),
                        KeyCode::Tab => {
                            self.terminal.complete();
                        }
                        c => {
                            if let Some(c) = c.as_char() {
                                term.text.push(c);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn submit(&mut self) {
        let text = &self.terminal.text;
        let command = terminal::parse(&text);

        self.terminal
            .output
            .push(Line::from(format!("> {}", text.clone())));

        match command {
            Ok(cmd) => {
                match cmd {
                    terminal::Cmd::Get(skill) => {
                        let n = self.sheet.skills.check(&skill, &self.sheet.ability_scores);

                        self.terminal
                            .output
                            .push(Line::from(format!("{} bonus: {}", skill, n)));
                    }
                    terminal::Cmd::Set { skill, arg } => {
                        //
                        //
                        self.terminal.output.push(Line::from(format!(
                            "set {} bonus to {} (but not)",
                            skill, arg
                        )));
                    }
                    terminal::Cmd::Roll(roll) => {
                        let result = roll.roll(&mut self.rng);

                        let s = format!("Results: {:?}", result.results,);
                        let s2 = format!("Total {}", result.total);

                        self.terminal.output.push(Line::from(s));
                        self.terminal.output.push(Line::from(s2));
                    }
                }
            }

            Err(e) => self
                .terminal
                .output
                .push(Line::from(format!("Error: {}", e)).red()),
        }

        self.terminal.text.clear();
    }

    fn render(&mut self, frame: &mut Frame) {
        let sheet = &self.sheet;
        let outer = Block::default()
            .title(Line::from(sheet.name.as_str()))
            .borders(Borders::ALL)
            .border_style(Style::new().blue())
            .padding(Padding::uniform(2))
            .merge_borders(ratatui::symbols::merge::MergeStrategy::Fuzzy);

        let area = outer.inner(frame.area());

        let rows = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Fill(2), Constraint::Fill(3)])
            .split(area);

        let top_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Fill(2),
                Constraint::Fill(4),
                Constraint::Fill(3),
            ])
            .split(rows[0]);

        let attr = &self.sheet.ability_scores;

        let green = Block::new()
            .borders(Borders::ALL)
            .border_style(Style::new().light_green());

        let red = Block::bordered().border_style(Style::new().red());

        let ability_scores_table = ability_scores_table(attr);

        let ability_scores_table = ability_scores_table.block(green.title("Ability scores"));

        draw_basic_block(&sheet, Color::Yellow, top_chunks[1], frame);

        // render outer border
        frame.render_widget(outer, frame.area());

        frame.render_widget(ability_scores_table, top_chunks[0]);
        // frame.render_widget(yellow, basic_senses_area[1]);

        let bottom_block = rows[1];

        let bottom_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(4), Constraint::Fill(2), Constraint::Fill(6)])
            .split(bottom_block);

        let bottom_top_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(4), Constraint::Fill(2), Constraint::Fill(6)])
            .split(bottom_layout[0]);

        frame.render_widget(&red, bottom_block);

        let counter_widget: Line = format!("{}", self.counter).into();

        let counter_widget = Paragraph::new(counter_widget)
            .block(red.clone().title("Counter").title_bottom("u/j/k"));

        frame.render_widget(counter_widget, bottom_top_row[0]);

        let text_box = Line::from(self.terminal.text.as_str());

        let text_box = Paragraph::new(text_box).block(
            if self.input {
                Block::bordered().yellow()
            } else {
                red
            }
            .title("Input"),
        );

        frame.render_widget(text_box, bottom_top_row[1]);

        let h = bottom_layout[1].height;

        let lines = Paragraph::new(self.terminal.output.clone()).scroll((
            (self.terminal.output.len() as u16).saturating_sub(h.saturating_sub(2)),
            0,
        ));

        let output = lines;

        let output = output.block(if self.input {
            Block::bordered().yellow()
        } else {
            Block::bordered().light_red()
        });

        frame.render_widget(output, bottom_layout[1]);

        let skills_block = Block::bordered()
            .style(Style::new().cyan())
            .title("Skills")
            .padding(Padding::uniform(1));

        let area = skills_block.inner(top_chunks[2]);

        frame.render_widget(&skills_block, top_chunks[2]);

        // draw border

        // draw contents
        render_skills_block(sheet, frame, area);
    }
}

fn draw_basic_block<'a>(sheet: &Character, color: Color, area: Rect, frame: &mut Frame) {
    let block = Block::bordered().style(Style::new().fg(color));

    // frame.render_widget(&block, area);

    let block = block.merge_borders(ratatui::symbols::merge::MergeStrategy::Exact);

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let class = format!("Class: {}", sheet.class);
    let class = Span::from(class).style(Style::new().fg(color));
    let level = format!("Level: {}", sheet.level).into();

    let initiative = format!("Initiative: {}", sheet.initiative).into();
    let armor = format!("AC: {}", sheet.armor_class).into();
    let health = format!(
        "Hit points: {}/{} ({})",
        sheet.health.current, sheet.health.max, sheet.health.bonus
    )
    .into();
    let b = Text::from_iter([class, level, initiative, armor, health]);

    let p = Paragraph::new(b).block(block.clone().title("Basic"));
    frame.render_widget(p, inner_area[0]);

    let senses = &sheet.senses;

    let extra = senses
        .extra
        .iter()
        .map(|e| e.to_string())
        .collect::<String>();

    let t = Text::from(vec![
        format!("Passive perception: {}", senses.perception).into(),
        format!("Passive investigaation: {}", senses.investigation).into(),
        format!("Passive insight: {}", senses.insight).into(),
        extra.into(),
    ]);

    let p = Paragraph::new(t).block(block.title("Senses"));
    frame.render_widget(p, inner_area[1]);
}

fn render_skills_block(sheet: &Character, frame: &mut Frame, area: Rect) {
    // draw everything else inside

    use sheet::skills::Skill as S;

    let scores = &sheet.ability_scores;

    let skills = &sheet.skills;

    let str = Text::from(vec![
        format!("Athletics: {}", skills.check(&S::Athletics, scores)).into(),
    ]);

    let wrap = Wrap { trim: false };

    let str = Paragraph::new(str)
        .block(Block::bordered().title("Str"))
        .wrap(wrap);

    let dex = Text::from(vec![
        format!("Acrobatics: {}\n", skills.check(&S::Acrobatics, scores)).into(),
        format!(
            "Sleight Of Hand: {}\n",
            skills.check(&S::SleightOfHand, scores)
        )
        .into(),
        format!("Stealth: {}", skills.check(&S::Stealth, scores)).into(),
    ]);

    let dex = Paragraph::new(dex)
        .block(Block::bordered().title("Dex"))
        .wrap(wrap);

    let int_rows = [
        Row::new(vec![
            Cell::from("Arcana"),
            Cell::from(skills.check(&S::Arcana, scores).to_string()),
        ]),
        Row::new(vec![
            Cell::from("History"),
            Cell::from(skills.check(&S::History, scores).to_string()),
        ]),
        Row::new(vec![
            Cell::from("Investigation"),
            Cell::from(skills.check(&S::Investigation, scores).to_string()),
        ]),
        Row::new(vec![
            Cell::from("Nature"),
            Cell::from(skills.check(&S::Nature, scores).to_string()),
        ]),
        Row::new(vec![
            Cell::from("Religion"),
            Cell::from(skills.check(&S::Religion, scores).to_string()),
        ]),
    ];

    let widths = [Constraint::Length(14), Constraint::Length(4)];

    let int = Table::new(int_rows, widths)
        .block(Block::bordered().title("Int"))
        .column_spacing(2);

    let wis = Text::from(vec![
        format!(
            "Animal Handling: {}\n",
            skills.check(&S::AnimalHandling, scores)
        )
        .into(),
        format!("Insight: {}\n", skills.check(&S::Insight, scores)).into(),
        format!("Medicine: {}\n", skills.check(&S::Medicine, scores)).into(),
        format!("Perception: {}\n", skills.check(&S::Perception, scores)).into(),
        format!("Survival: {}\n", skills.check(&S::Survival, scores)).into(),
    ]);

    let wis = Paragraph::new(wis)
        .block(Block::bordered().title("Wis"))
        .wrap(wrap);

    let char = Text::from(vec![
        format!("Deception: {}\n", skills.check(&S::Deception, scores)).into(),
        format!("Intimidation: {}\n", skills.check(&S::Intimidation, scores)).into(),
        format!("Performance: {}\n", skills.check(&S::Performance, scores)).into(),
        format!("Persuasion: {}\n", skills.check(&S::Persuasion, scores)).into(),
    ]);

    let char = Paragraph::new(char)
        .block(Block::bordered().title("Char"))
        .wrap(wrap);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(4),
            Constraint::Fill(1),
        ])
        .split(area);

    let chunks_top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(4), Constraint::Fill(5)])
        .split(rows[0]);

    let chunks_bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(2), Constraint::Fill(2)])
        .split(rows[1]);

    // top row
    frame.render_widget(str, chunks_top[0]);
    frame.render_widget(dex, chunks_top[1]);

    // middle row
    frame.render_widget(int, chunks_bottom[0]);
    frame.render_widget(wis, chunks_bottom[1]);

    // bottom row
    frame.render_widget(char, rows[2]);
}

fn ability_scores_table<'a>(attr: &'a AbilityScores) -> Table<'a> {
    let table_rows = vec![
        Row::new(vec![
            Cell::from("str:"),
            Cell::from(Line::from(format!("{:}", attr.str.get())).alignment(Alignment::Right)),
            Cell::from(format!("({:+})", attr.str.modifier())),
        ]),
        Row::new(vec![
            Cell::from("dex:"),
            Cell::from(Line::from(format!("{:}", attr.dex.get())).alignment(Alignment::Right)),
            Cell::from(format!("({:+})", attr.dex.modifier())),
        ]),
        Row::new(vec![
            Cell::from("con:"),
            Cell::from(Line::from(format!("{:}", attr.con.get())).alignment(Alignment::Right)),
            Cell::from(format!("({:+})", attr.con.modifier())),
        ]),
        Row::new(vec![
            Cell::from("int:"),
            Cell::from(Line::from(format!("{:}", attr.int.get())).alignment(Alignment::Right)),
            Cell::from(format!("({:+})", attr.int.modifier())),
        ]),
        Row::new(vec![
            Cell::from("wis:"),
            Cell::from(Line::from(format!("{:}", attr.wis.get())).alignment(Alignment::Right)),
            Cell::from(format!("({:+})", attr.wis.modifier())),
        ]),
        Row::new(vec![
            Cell::from("char:"),
            Cell::from(Line::from(format!("{:}", attr.char.get())).alignment(Alignment::Right)),
            Cell::from(format!("({:+})", attr.char.modifier())),
        ]),
    ];

    let widths = [
        Constraint::Length(5),
        Constraint::Length(2),
        Constraint::Length(4),
    ];

    let table = Table::new(table_rows, widths)
        .column_spacing(1)
        .highlight_symbol(">> ")
        .style(Style::new().light_green());
    table
}
