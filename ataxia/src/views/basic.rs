use ataxia_types::{
    AbilityScores, Character, Score,
    skills::{Skill, Skills},
};
use iced::{
    Alignment, Element, Length,
    alignment::{Horizontal::Left, Vertical},
    widget::{
        self, Column, Container, Grid, Row, Rule, Text, button, checkbox, column, container, row,
        rule, scrollable, table::table, text, text_input, toggler,
    },
};

use crate::Message;

#[derive(Debug, Default, Clone)]
pub struct BasicState {
    pub hp_input: String,
}
#[derive(Clone, Debug)]
pub enum BasicMessage {
    InputChanged(String),
    Heal,
    Damage,
    ResetInput,
}

pub fn basic<'a>(sheet: &'a Character, state: &'a BasicState) -> Element<'a, Message> {
    let x = row![scores(sheet), health(sheet, state)].spacing(20);
    let c1 = container(x)
        .max_width(320)
        .max_height(200)
        .center(Length::Fill)
        .style(container::bordered_box)
        .padding(12);

    let c2 = skills(sheet);
    let row = row![c1, c2].spacing(20);

    container(row).center(Length::Shrink).into()
}

pub fn scores<'a>(sheet: &'a Character) -> Container<'a, Message> {
    let labels: Vec<Element<Message>> = ["str", "dex", "con", "int", "wis", "cha"]
        .iter()
        .map(|f| text!("{f}").into())
        .collect();

    let labels = Column::from_vec(labels);

    let scores: Vec<Element<Message>> = sheet
        .ability_scores
        .iter()
        .iter()
        .map(|f| text!("{}", f.get()).align_x(Alignment::End).into())
        .collect();

    let scores = Column::from_vec(scores);

    let mods: Vec<Element<Message>> = sheet
        .ability_scores
        .iter()
        .iter()
        .map(|f| text!("({:+})", f.modifier()).into())
        .collect();

    let mods = Column::from_vec(mods);

    let rows = row![labels, scores, mods].spacing(12);

    container(rows).padding(8).height(Length::Fill)
}

pub fn health<'a>(sheet: &'a Character, state: &'a BasicState) -> Container<'a, Message> {
    let health = &sheet.health;
    let current = health.current;
    let max = health.max;
    let bonus = health.bonus;

    let heal = button(text("Heal").center())
        .style(button::success)
        .on_press(Message::Basic(BasicMessage::Heal))
        .width(Length::Fill);
    let damage = button(text("Damage").center())
        .style(button::danger)
        .on_press(Message::Basic(BasicMessage::Damage))
        .width(Length::Fill);
    let input = text_input("hp", &state.hp_input)
        .on_input(|f| Message::Basic(BasicMessage::InputChanged(f)));
    let input_col = column![heal, input, damage];
    let input_col = container(input_col).center(Length::Fill);

    let labels = vec!["hp", "of", "max", "tmp"];

    let hp = vec![
        text!("{current}"),
        text!("/"),
        text!("{max}"),
        text!("{bonus}"),
    ];

    let t = labels.iter().zip(hp).map(|(f, g)| {
        column![g, text!("{f}").style(text::secondary)]
            .align_x(Alignment::Center)
            .into()
    });
    let t = Row::from_iter(t.into_iter()).spacing(12);

    let r = column![t, input_col];
    container(r)
}

pub fn skills<'a>(sheet: &'a Character) -> Container<'a, Message> {
    let skill_names = Skill::ALL_STR_PRETTY;

    let prof: Vec<Element<Message>> = Skill::ALL
        .iter()
        .map(|f| {
            let checked = sheet.skills.proficiencies.contains(*f);
            if checked {
                if sheet.skills.expertise.contains(*f) {
                    checkbox(true).style(checkbox::danger)
                } else {
                    checkbox(true).style(checkbox::success)
                }
            } else {
                checkbox(false).style(checkbox::primary)
            }
        })
        .map(|f| container(f).into())
        .collect();
    let skills = Skill::ALL
        .iter()
        .map(|f| sheet.skills.check(f, &sheet.ability_scores))
        .map(|f| text!("{f}").align_x(Alignment::End))
        .collect::<Vec<Text>>();

    let skill_names: Vec<Text> = skill_names.iter().map(|f| text!("{f}")).collect();

    // let c1 = Column::from_vec(prof).spacing(5);
    // let c2 = Column::from_vec(skill_names);
    // let c3 = Column::from_vec(skills).align_x(Alignment::End);
    // let r = row![c1, c2, c3].spacing(8);

    let rows = prof
        .into_iter()
        .zip(skill_names)
        .zip(skills)
        .map(|((a, b), c)| row![a, b.width(120), c.width(12)].spacing(12).into())
        .collect();

    let r = Column::from_vec(rows);

    container(r).into()
}
