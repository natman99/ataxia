use ataxia_types::{Character, Score, skills::Skill};
use iced::{
    Alignment, Element, Length, Task,
    widget::{
        Column, Container, Row, Text, button, checkbox, column, container, row, text, text_input,
    },
};

use crate::{Message, MeterMessage};

#[derive(Debug, Default, Clone)]
pub struct BasicState {
    pub hp_input: String,
    pub reaction: bool,
}

impl<'a> BasicState {
    pub fn view(&'a self, sheet: &'a Character) -> Element<'a, Message> {
        let x = row![scores(sheet), health(sheet, self)].spacing(20);

        let z = column![x, reaction(self)];

        let c1 = container(z)
            // .center(320)
            .center_x(300)
            .center_y(200)
            .style(container::bordered_box)
            .padding(12);

        let s = passive_senses(sheet);

        let s = row![
            s.center(Length::Fill),
            ac_init_prof_speed(sheet).center(Length::Fill)
        ]
        .height(160)
        .width(300);

        let c1 = column![c1, s];

        let c2 = skills(sheet);

        let saving = saving_throws(sheet).width(Length::Fill);
        let details = character_details(sheet).width(Length::Fill);
        let meters_temp = meters(sheet).width(Length::Fill);
        let left_col = column![details, saving, meters_temp].width(200);
        let row = row![left_col, c1, c2].spacing(8);

        container(row).center(Length::Fill).into()
    }

    pub fn update(&mut self, message: BasicMessage, sheet: &mut Character) -> Task<Message> {
        match message {
            BasicMessage::InputChanged(s) => {
                if s.parse::<i32>().is_ok() || s.is_empty() || s == "-" {
                    self.hp_input = s;
                }
                Task::none()
            }
            BasicMessage::Heal => {
                if let Ok(n) = self.hp_input.parse() {
                    sheet.health.heal(n);
                }
                Task::done(Message::Basic(BasicMessage::ResetInput))
            }
            BasicMessage::Damage => {
                if let Ok(n) = self.hp_input.parse() {
                    sheet.health.hit(n);
                }
                Task::done(Message::Basic(BasicMessage::ResetInput))
            }
            BasicMessage::ResetInput => {
                self.hp_input.clear();
                Task::none()
            }
            BasicMessage::Reaction(b) => {
                self.reaction = b;
                Task::none()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum BasicMessage {
    InputChanged(String),
    Heal,
    Damage,
    ResetInput,
    Reaction(bool),
}

pub fn scores(sheet: &Character) -> Container<'_, Message> {
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

    let labels = ["hp", "of", "max", "tmp"];

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
    let t = Row::from_iter(t).spacing(12);

    let r = column![t, input_col];
    container(r)
}

pub fn skills(sheet: &Character) -> Container<'_, Message> {
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

    let rows = prof
        .into_iter()
        .zip(skill_names)
        .zip(skills)
        .map(|((a, b), c)| row![a, b.width(120), c.width(12)].spacing(12).into())
        .collect();

    let r = Column::from_vec(rows);

    container(r).style(container::secondary).padding(12)
}

fn reaction(basic: &BasicState) -> Container<'_, Message> {
    let x = checkbox(basic.reaction)
        .label("Reaction")
        .on_toggle(|f| Message::Basic(BasicMessage::Reaction(f)));
    container(x)
}

fn passive_senses(sheet: &Character) -> Container<'_, Message> {
    let x = text("Passive Senses")
        .style(text::secondary)
        // .size(14)
        .center();

    let senses = sheet.passive_senses();
    let z = column![
        text!("Perception {}", senses.perception),
        text!("Investigation {}", senses.investigation),
        text!("Insight {}", senses.insight),
        x,
    ];

    // let c = column![x, z];

    container(z).style(container::bordered_box).padding(12)
}

fn ac_init_prof_speed(sheet: &Character) -> Container<'_, Message> {
    let c = column![
        text!("Proficiency bonus {}", sheet.skills.proficiency_bonus),
        text!("AC {}", sheet.armor_class),
        text!("Initiative {:+}", sheet.initiative()),
        text!("Walking speed {}", sheet.walking_speed),
        text!("More info").style(text::secondary).center(),
    ];

    container(c).style(container::bordered_box).padding(12)
}

pub fn saving_throws(sheet: &Character) -> Container<'_, Message> {
    let prof: Vec<Element<Message>> = Score::ALL
        .iter()
        .map(|f| {
            let checked = sheet.saving_throws.proficient(f);
            checkbox(checked).style(checkbox::primary)
        })
        .map(|f| container(f).into())
        .collect();
    let skills = Score::ALL
        .iter()
        .map(|f| {
            sheet
                .saving_throws
                .check(f, &sheet.ability_scores, &sheet.skills)
        })
        .map(|f| text!("{f}").align_x(Alignment::End))
        .collect::<Vec<Text>>();

    let score_names: Vec<Text> = Score::ALL_STR.iter().map(|f| text!("{f}")).collect();

    let rows = prof
        .into_iter()
        .zip(score_names)
        .zip(skills)
        .map(|((a, b), c)| row![a, b.width(120), c.width(12)].spacing(12).into())
        .collect();

    let r = Column::from_vec(rows);

    container(r).style(container::secondary).padding(12)
}

fn character_details(sheet: &Character) -> Container<'_, Message> {
    let subclasses = sheet
        .classes
        .iter()
        .map(|f| format!("{} ", f.subclass))
        .collect::<String>();
    let a = [
        sheet.name.clone(),
        sheet.race.clone(),
        format!("{}", sheet.classes),
        subclasses,
    ]
    .into_iter()
    .map(|f| text(f).into())
    .collect::<Vec<Element<Message>>>();
    let c = Column::from_vec(a);

    container(c).style(container::bordered_box).padding(12)
}

fn badge<'a>(top: &'a str, bottom: &'a str) -> Container<'a, Message> {
    let c = column![text(top), text(bottom).style(text::secondary)];

    container(c)
}

fn meters(sheet: &Character) -> Container<'_, Message> {
    let mut rows = vec![];
    const BOX: char = '■';
    for m in sheet.meters.meters.values() {
        let max = m.slot_number;
        let remaining = max - m.spent;

        let mut r = row![text!("{} ", m.name.clone())];

        for _ in 0..m.spent {
            r = r.push(
                checkbox(true)
                    .size(22)
                    .on_toggle(|_| Message::Meter(MeterMessage::Restore(m.name.clone()))),
            );
        }

        for _ in 0..remaining {
            r = r.push(
                checkbox(false)
                    .size(22)
                    .on_toggle(|_| Message::Meter(MeterMessage::Spend(m.name.clone()))),
            );
        }

        rows.push(r.padding(6));
    }

    let mut cols: Column<'_, Message> = Column::new();

    for i in rows {
        cols = cols.push(i);
    }

    container(cols).style(container::bordered_box).padding(12)
}

fn pinned_widget<'a>(state: &BasicState, sheet: &'a Character) -> Container<'a, Message> {
    let mut cols = Column::new().padding(8);

    for i in state.pinned {
        match i {
            Pinnable::Spell(spell) => todo!(),
            Pinnable::Item(item) => {
                let roll = if let Some(f) = item.roll {
                    f.to_string()
                } else {
                    "-".to_string()
                };

                let to_hit = if let Some(f) = item.to_hit(&sheet.ability_scores, &sheet.skills) {
                    format!("{f:+}")
                } else {
                    "-".to_string()
                };

                let b =
                    button("x").on_press_with(|| Message::Basic(BasicMessage::Unpin(i.clone())));
                let r = row![text(&item.name), text(to_hit), text(roll), b].spacing(6);
                cols = cols.push(r);
            }
        }
    }
    Container::new(cols).padding(8)
}
