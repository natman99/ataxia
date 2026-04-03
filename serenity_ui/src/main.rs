use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use iced::theme::Style;
use iced::widget::{Column, Container, Text, button, column, container, row, text};

use iced::{Application, Background, Element, Task, Theme};
use log::{error, info, warn};
use parking_lot::Mutex;
use rfd::FileDialog;
use serenity_agent::MyClient;
use serenity_types::skills::Skill;
use serenity_types::{Character, sheet};

fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();
    iced::application(App::new, App::update, App::view)
        .run()
        .unwrap();
}

pub struct App {
    counter: i64,
    sheet: Arc<Mutex<Character>>,
    widgets: Vec<SWidget>,
    prev: Vec<Character>,
    client: MyClient,
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        let counter_text = text(format!("{}", self.counter));

        let button1 = button("press").on_press(Message::Increment);
        let button2 = button("press").on_press(Message::Decrement);
        let c = column![counter_text, button1, button2];

        let sheet = self.sheet.lock();

        let title: Text = text(sheet.name.clone()).size(20).style(|f: &Theme| {
            let e = f.palette();
            let mut style = text::Style::default();
            style.color = Some(e.success);
            style
        });

        let w = self
            .widgets
            .iter()
            .map(|w| match w {
                SWidget::Scores(s) => s.view(&sheet),
                SWidget::Skills(s) => s.view(&sheet),
                SWidget::Health(s) => s.view(&sheet),
            })
            .collect::<Vec<Element<_>>>();
        let mut r = row![c].padding(20).spacing(20);

        for i in w.into_iter() {
            r = r.push(i);
        }

        let cols = column![title, r].spacing(20).padding(20);

        cols.into()
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
            .set_directory("../")
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

        let sheet = Arc::new(Mutex::new(sheet));

        // CryptoProvider::install_default(rustls::crypto::aws_lc_rs::default_provider())
        //        .expect("failed to install crypto nonsense.");
        let client = ollama::Client::new(Nothing).expect("Ollama error");

        let client = serenity_agent::MyClient::new(
            file.unwrap_or(PathBuf::from("/")),
            client,
            sheet.clone(),
        );

        let client = pollster::block_on(client).unwrap();

        Self {
            counter: 0,
            sheet,
            widgets: vec![
                SWidget::Health(HealthWidget),
                SWidget::Scores(Scores),
                SWidget::Skills(SkillsWidget),
            ],
            prev: vec![],
            client,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

pub enum SWidget {
    Scores(Scores),
    Skills(SkillsWidget),
    Health(HealthWidget),
}

pub struct Scores;

impl Scores {
    pub fn view(&self, sheet: &Character) -> Element<'_, Message> {
        let list = sheet
            .ability_scores
            .iter()
            .map(|f| format!("{} ({:+})", f.get(), f.get_bonus()))
            .into_iter()
            .collect::<Vec<String>>();

        let names = ["str", "dex", "con", "int", "wis", "char"];

        let list = list
            .into_iter()
            .map(|f| iced::widget::text(f).into())
            .collect();
        let titles = Column::from_vec(
            names
                .into_iter()
                .map(|f| f.to_string())
                .map(|f| text(f).into())
                .collect::<Vec<Element<_>>>(),
        );
        let numbers = iced::widget::Column::from_vec(list);
        let r = row![titles, numbers].spacing(20).padding(10);

        Container::new(r)
            .style(|theme: &Theme| {
                let background = Background::Color(theme.extended_palette().primary.base.color);
                container::Style::default()
                    .background(background)
                    .color(theme.extended_palette().primary.base.text)
            })
            .padding(10)
            .into()
    }
}

pub struct SkillsWidget;

impl SkillsWidget {
    pub fn view(&self, sheet: &Character) -> Element<'_, Message> {
        let names = Skill::ALL_ITER.iter().map(|f| format!("{}", f));
        let numbers = Skill::ALL_ITER
            .iter()
            .map(|f| format!("+{}", sheet.skills.check(f, &sheet.ability_scores)));

        let names = names.map(|f| text(f).into()).collect::<Vec<Element<_>>>();

        let numbers = numbers.map(|f| text(f).into()).collect::<Vec<Element<_>>>();

        let c1 = Column::from_vec(names);

        let c2 = Column::from_vec(numbers);

        let row = row![c1, c2];

        let r = row.spacing(20).padding(10);

        Container::new(r)
            .style(|theme: &Theme| {
                let background = Background::Color(theme.extended_palette().primary.base.color);
                container::Style::default()
                    .background(background)
                    .color(theme.extended_palette().primary.base.text)
            })
            .padding(10)
            .into()
    }
}

pub struct HealthWidget;

impl HealthWidget {
    pub fn view(&self, sheet: &Character) -> Element<'_, Message> {
        let h = &sheet.health;

        let max = h.max;
        let current = h.current;
        let bonus = h.bonus;
        let c = format!("Current: {current}/{max}");
        let bonus = format!("Temp hp: {}", bonus);

        let c = text(c);
        let bonus = text(bonus);

        let r = row![c, bonus].spacing(20);

        Container::new(r)
            .style(|theme: &Theme| {
                let background = Background::Color(theme.extended_palette().secondary.base.color);
                container::Style::default()
                    .background(background)
                    .color(theme.extended_palette().secondary.base.text)
            })
            .padding(10)
            .into()
    }
}
