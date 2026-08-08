use ataxia_types::{Character, Score, skills::Skill};
use itertools::Itertools;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    symbols::merge::MergeStrategy,
    text::Text,
    widgets::{Block, Cell, Padding, Paragraph, Row, Scrollbar, Table, Wrap},
};

use crate::FocusedContent;

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Main,
    Spells,
    Features,
    Inventory,
    // Stats,
    // Skills,
}
impl State {
    pub fn render<'a>(
        &self,
        frame: &mut Frame,
        area: Rect,
        sheet: &'a Character,
        focused_content: &mut FocusedContent,
    ) {
        match self {
            State::Main => render_main(frame, area, sheet, focused_content),
            State::Spells => render_spells(frame, area, sheet),
            State::Features => frame.render_widget("todo", area),
            State::Inventory => frame.render_widget("todo", area),
            // State::Stats => todo!(),
            // State::Skills => todo!(),
        }
    }
}

fn render_main(frame: &mut Frame, area: Rect, sheet: &Character, content: &mut FocusedContent) {
    let (top, bottom) = {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            // 18 skills + 2 border
            .constraints([Constraint::Min(20), Constraint::Percentage(60)])
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
    render_focused_content(frame, bottom, content);
}

fn render_focused_content(frame: &mut Frame, area: Rect, content: &mut FocusedContent) {
    let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight);

    // let lines = content
    // .items
    // .iter()
    // .map(|f| Paragraph::from(f.as_str()))
    // .collect::<Vec<Paragraph>>();

    let p = Paragraph::new(content.buf.as_str())
        .wrap(Wrap { trim: false })
        .scroll((content.scroll.get_position() as u16, 0))
        .block(Block::bordered());

    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut content.scroll,
    );

    frame.render_widget(p, area);
}

fn render_main_left(sheet: &Character, frame: &mut Frame, area: Rect) {
    let (top, bottom) = {
        let split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(11), Constraint::Fill(1)])
            // .margin(1)
            .split(area);
        (split[0], split[1])
    };

    let titles = [
        "Class",
        "Health",
        "Armor Class",
        "Race",
        "Initiative",
        "Proficiency Mod",
        "Walking speed",
        "Save DC",
        "Passive",
        "Senses",
    ];

    let values = [
        format!("{}", sheet.classes),
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
        format!(
            "Per {}, Inv {}, Ins {}",
            sheet.senses.perception, sheet.senses.investigation, sheet.senses.insight
        ),
        format!("{}", sheet.senses.extra),
    ];

    let rows = titles
        .into_iter()
        .zip(values)
        .map(|(f, m)| Row::new([f.to_string(), m]));
    let rows = Vec::from_iter(rows);

    let widths = [Constraint::Length(15), Constraint::Length(30)];

    let table = Table::new(rows, widths).column_spacing(1);

    frame.render_widget(
        Block::bordered()
            .title(sheet.name.as_str())
            // .merge_borders(MergeStrategy::Exact)
            .title_bottom("Info"),
        area,
    );

    let table = table.block(Block::new().padding(Padding::new(1, 0, 1, 0)));

    frame.render_widget(table, top);

    let conditions = sheet
        .conditions
        .conditions
        .iter()
        .map(|f| f.to_string())
        .join(", ");

    let cond = Paragraph::new(conditions).block(
        Block::bordered()
            .merge_borders(MergeStrategy::Exact)
            .title("Conditions"),
    );
    frame.render_widget(cond, bottom);
}

fn render_main_right(sheet: &Character, frame: &mut Frame, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Min(10)])
        .split(area);

    let left = layout[0];
    let right = layout[1];

    render_scores_and_lang(sheet, frame, left);

    render_skills(sheet, frame, right);
}

fn render_scores_and_lang(sheet: &Character, frame: &mut Frame, area: Rect) {
    let (top, bottom) = {
        let split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Fill(1)])
            .split(area);
        (split[0], split[1])
    };

    frame.render_widget(scores(sheet), top);

    let p = Paragraph::new(format!("{}", sheet.languages))
        .wrap(Wrap { trim: false })
        .block(Block::bordered().title_bottom("Languages"));

    frame.render_widget(p, bottom);
}

fn scores<'a>(sheet: &'a Character) -> Table<'a> {
    let names = [
        Score::Str,
        Score::Dex,
        Score::Con,
        Score::Int,
        Score::Wis,
        Score::Cha,
    ];
    let scores = sheet.ability_scores.iter().map(|f| (f.get(), f.modifier()));

    let rows = names.iter().zip(scores).map(|(name, (n, modifier))| {
        Row::new([
            Cell::new(Text::from(name.to_string())),
            Cell::new(
                Text::from(format!("{}", n)).alignment(ratatui::layout::HorizontalAlignment::Right),
            ),
            Cell::new(Text::from(format!("({:+})", modifier))),
        ])
    });

    let widths = [
        Constraint::Length(5),
        Constraint::Length(2),
        Constraint::Length(6),
    ];

    let table = Table::new(rows, widths)
        .block(Block::bordered().title_bottom("Scores"))
        .column_spacing(1);

    table
}

fn render_skills(sheet: &Character, frame: &mut Frame, area: Rect) {
    let skills: Vec<i32> = Skill::ALL_ITER
        .iter()
        .map(|f| sheet.skills.check(f, &sheet.ability_scores))
        .collect();

    let names = Skill::ALL_STR_PRETTY;

    let rows = names.into_iter().zip(skills).map(|(f, m)| {
        Row::new([
            Cell::new(f),
            Cell::new(
                Text::from(format!("{:+}", m))
                    .alignment(ratatui::layout::HorizontalAlignment::Right),
            ),
        ])
    });

    let widths = [Constraint::Length(16), Constraint::Length(4)];

    let table = Table::new(rows, widths)
        .block(Block::bordered().title_bottom("Skills"))
        .column_spacing(1);

    frame.render_widget(table, area);
}

fn render_spells(frame: &mut Frame, area: Rect, sheet: &Character) {
    let (top, bottom) = {
        let split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(area);

        (split[0], split[1])
    };

    render_spells_list(frame, top, sheet);
    render_meters(frame, bottom, sheet);
}

fn render_meters(frame: &mut Frame, area: Rect, sheet: &Character) {
    const EMPTY_BOX: char = '\u{25A1}';
    const FILLED_BOX: char = '\u{25A0}';

    let widths = [Constraint::Length(8), Constraint::Fill(1)];

    let mut rows = vec![];

    for i in sheet.meters.meters.iter() {
        let name = i.0.as_str();

        let max = i.1.slot_number;
        let filled = max - i.1.spent;
        let not_filled = max - filled;

        let mut out = String::new();

        for _ in 0..not_filled {
            out.push(FILLED_BOX);

            // out.push(' ');
        }
        for _ in 0..filled {
            out.push(EMPTY_BOX);

            // out.push(' ');
        }

        rows.push(Row::new([Cell::new(name), Cell::new(out)]));
    }

    let table = Table::new(rows, widths).block(Block::bordered().title("Meters"));

    frame.render_widget(table, area);
}

fn render_spells_list<'a>(frame: &mut Frame, area: Rect, sheet: &'a Character) {
    let mut rows = vec![];

    let header = Row::new([
        "Name", "Time", "Range", "Hit/DC", "Effect", "Duration", "Level",
    ]);

    let widths = [
        Constraint::Length(16),
        Constraint::Length(5),
        Constraint::Length(10),
        Constraint::Length(14),
        Constraint::Length(16),
        Constraint::Length(14),
        Constraint::Length(5),
    ];

    for i in sheet
        .spells
        .spells
        .iter()
        .sorted_by(|a, b| a.1.level.cmp(&b.1.level))
    {
        let s = i.1;
        let dc = {
            if let Some(ref dc) = s.dc {
                format!("{} {}, {}", dc.dc_type.name, sheet.save_dc(), dc.dc_success)
            } else {
                let spell_modifier = sheet.spell_bonus();

                format!("{:+}", spell_modifier)
            }
        };

        let damage = {
            let mut output = String::new();
            if let Some(ref d) = s.damage {
                if let Some(ref s) = d.damage_at_character_level {
                    // s.n1, s.n5, s.n11, s.n17,
                    let out = match sheet.classes[0].level.0 {
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
                    output.push_str("Scaling ");
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
            Cell::new(s.duration.as_str()),
            Cell::new(s.level.to_string()),
        ]);

        rows.push(r);
    }

    let t = Table::new(rows, widths)
        .header(header.clone())
        .block(Block::bordered().title("Spells"));

    frame.render_widget(t, area);
}
