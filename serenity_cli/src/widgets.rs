use std::io;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Text,
    widgets::{Block, Cell, Paragraph, Row, Table},
};
use serenity_types::{
    Character,
    class::{Class, ClassType},
    skills::{Skill, Skills},
};

use crate::{SpellsBuffer, commands::spell};

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Main,
    Spells(Option<usize>),
    Features(Option<usize>),
    Inventory,
    // Stats,
    // Skills,
}
impl State {
    pub fn render<'a>(&self, frame: &mut Frame, area: Rect, sheet: &'a Character) {
        match self {
            State::Main => render_main(frame, area, sheet),
            State::Spells(page) => render_spells(frame, area, sheet, *page),
            State::Features(_) => todo!(),
            State::Inventory => todo!(),
            // State::Stats => todo!(),
            // State::Skills => todo!(),
        }
    }
}

fn render_main(frame: &mut Frame, area: Rect, sheet: &Character) {
    let (top, bottom) = {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);
        (layout[0], layout[1])
    };

    let (top_left, top_right) = {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(top);
        (layout[0], layout[1])
    };

    render_main_left(sheet, frame, top_left);
    render_main_right(sheet, frame, top_right);
}

fn render_main_left(sheet: &Character, frame: &mut Frame, area: Rect) {
    let titles = [
        "Class",
        "Health",
        "Armor Class",
        "Race",
        "Initiative",
        "Proficiency Mod",
        "Walking speed",
        "Save DC",
    ];

    let values = [
        format!("{}", sheet.class),
        format!(
            "{}/{} ({})",
            sheet.health, sheet.health.max, sheet.health.bonus
        ),
        format!("{}", sheet.armor_class),
        format!("{}", sheet.race),
        format!("{}", sheet.initiative),
        format!("{}", sheet.skills.proficiency_bonus),
        format!("{}", sheet.walking_speed),
        format!("{}", sheet.save_dc()),
    ];

    let rows = titles
        .into_iter()
        .zip(values)
        .map(|(f, m)| Row::new([f.to_string(), m]));
    let rows = Vec::from_iter(rows);

    let widths = [Constraint::Length(15), Constraint::Length(10)];

    let table = Table::new(rows, widths)
        .block(
            Block::bordered()
                .title(sheet.name.as_str())
                .title_bottom("Info"),
        )
        .column_spacing(1);

    frame.render_widget(table, area);
}

fn render_main_right(sheet: &Character, frame: &mut Frame, area: Rect) {
    let skills: Vec<i32> = Skill::ALL_ITER
        .iter()
        .map(|f| sheet.skills.check(f, &sheet.ability_scores))
        .collect();

    let names = Skill::ALL_STR_PRETTY;

    let rows = names.into_iter().zip(skills).map(|(f, m)| {
        Row::new([
            Cell::new(f.to_string()),
            Cell::new(
                Text::from(format!("{}", m)).alignment(ratatui::layout::HorizontalAlignment::Right),
            ),
        ])
    });

    let widths = [Constraint::Length(16), Constraint::Length(4)];

    let table = Table::new(rows, widths)
        .block(Block::bordered().title_bottom("Skills"))
        .column_spacing(1);

    frame.render_widget(table, area);
}

fn render_spells<'a>(frame: &mut Frame, area: Rect, sheet: &'a Character, page: Option<usize>) {
    let mut rows = vec![];

    let header = Row::new(["Name", "Time", "Range", "Hit/DC", "Effect", "Duration"]);

    let widths = [
        Constraint::Length(16),
        Constraint::Length(5),
        Constraint::Length(10),
        Constraint::Length(16),
        Constraint::Length(20),
        Constraint::Length(10),
    ];

    for i in sheet.spells.spells.iter() {
        let s = i.1;
        let dc = {
            if let Some(ref dc) = s.dc {
                format!("{dc}")
            } else {
                let spell_modifier = sheet.attack_bonus();
                format!("{}", spell_modifier)
            }
        };

        let damage = {
            let mut output = String::new();
            if let Some(ref d) = s.damage {
                if let Some(ref s) = d.damage_at_character_level {
                    // s.n1, s.n5, s.n11, s.n17,
                    let out = match sheet.class[0].level.0 {
                        0..5 => &s.n1,
                        5..11 => &s.n5,
                        11..17 => &s.n11,
                        17..200 => &s.n17,
                        _ => unreachable!("There should never be a level higher than 20."),
                    };
                    output.push_str(&out);
                    output.push(' ');
                }

                if d.damage_at_slot_level.is_some() {
                    output.push_str("Scaling");
                }

                if let Some(ref s) = d.damage_type {
                    output.push_str(s.name.as_str());
                    output.push(' ');
                }
            }

            if output.is_empty() {
                output.push_str("None");
            }

            output
        };
        let r = Row::new([
            Cell::new(s.name.as_str()),
            Cell::new(s.casting_time.as_str()),
            Cell::new(s.range.as_str()),
            Cell::new(dc),
            Cell::new(damage),
        ]);
        rows.push(r);
    }

    let t = Table::new(rows, widths).header(header.clone());

    frame.render_widget(t, area);
}
