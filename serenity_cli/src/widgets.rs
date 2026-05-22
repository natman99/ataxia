use std::io;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect}, widgets::Table,
};
use serenity_types::Character;

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Main,
    Spells(Option<usize>),
    Features(Option<usize>),
    Inventory,
    Stats,
    Skills,
}
impl State {
    fn render(&self, frame: &mut Frame, area: Rect, sheet: &Character) -> Result<(), io::Error> {
        match self {
            State::Main => todo!(),
            State::Spells(_) => todo!(),
            State::Features(_) => todo!(),
            State::Inventory => todo!(),
            State::Stats => todo!(),
            State::Skills => todo!(),
        }
    }
}

fn render_main(frame: &mut Frame, area: Rect, sheet: &Character) -> Result<(), io::Error> {
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
            .split(area);
        (layout[0], layout[1])
    };

    let rows = [
        sheet.name.to_string(),
        format!("{}", sheet.class),
        format!("{}", sheet.armor_class),
        format!("{}", sheet.health),
        format!("{}", sheet.race),
        format!("{}", sheet.initiative),
        format!("{}", sheet.senses),
    ]

    Ok(())
}
