use std::fs;

use iced::widget::{Column, Text, button, column, row, text};

use iced::{Application, Element, Task};
use log::{error, info, warn};
use rfd::FileDialog;
use serenity_types::Character;
use serenity_types::skills::Skill;

pub struct App {
    counter: i64,
    sheet: Character,
    widgets: Vec<SWidget>,
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        let text = text(format!("{}", self.counter));

        let button1 = button("press").on_press(Message::Increment);
        let button2 = button("press").on_press(Message::Decrement);
        let c = column![text, button1, button2];

        let mut w = self
            .widgets
            .iter()
            .map(|w| match w {
                SWidget::Scores(s) => s.view(&self.sheet),
                SWidget::Skills(s) => s.view(&self.sheet),
            })
            .collect::<Vec<Element<_>>>();
        let r = row![c, w.pop().unwrap(), w.pop().unwrap()]
            .padding(20)
            .spacing(10);

        r.into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => self.counter += 1,
            Message::Decrement => self.counter += -1,
        }
        Task::none()
    }

    fn new() -> Self {
        let file = FileDialog::new()
            .set_directory("/")
            .add_filter("json", &["json"])
            .pick_file();

        let sheet = if let Some(f) = file {
            let s = fs::read_to_string(&f).expect("Invalid file path");
            let c: Character = serde_json::from_str(&s).unwrap_or_default();
            if c.name.is_empty() {
                warn!("Loading likely failed!");
            }
            c
        } else {
            Character::default()
        };

        Self {
            counter: 0,
            sheet: sheet,
            widgets: vec![SWidget::Scores(Scores), SWidget::Skills(SkillsWidget)],
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}
fn main() {
    iced::application(App::new, App::update, App::view)
        .run()
        .unwrap();
}

pub enum SWidget {
    Scores(Scores),
    Skills(SkillsWidget),
}

pub struct Scores;

impl Scores {
    pub fn view(&self, sheet: &Character) -> Element<'_, Message> {
        let list = sheet
            .ability_scores
            .iter()
            .map(|f| format!("{} ({})", f.get(), f.get_bonus()))
            .into_iter()
            .collect::<Vec<String>>();
        let list = list
            .into_iter()
            .map(|f| iced::widget::text(f).into())
            .collect();
        iced::widget::Column::from_vec(list).into()
    }
}

pub struct SkillsWidget;

impl SkillsWidget {
    pub fn view(&self, sheet: &Character) -> Element<'_, Message> {
        let s = Skill::ALL_ITER.iter().map(|f| format!("{}", f));
        let s2 = Skill::ALL_ITER
            .iter()
            .map(|f| format!("{}", sheet.skills.check(f, &sheet.ability_scores)));

        let names = s.map(|f| text(f).into()).collect::<Vec<Element<_>>>();
        let c1 = Column::from_vec(names);

        let c2 = column![text("A")];

        let row = row![c1, c2];

        row.spacing(10).padding(10).into()
    }
}
