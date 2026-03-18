use iced::widget::{button, column, text};
use iced::{Application, Element, Task};
use serenity_types::Character;

pub struct App {
    counter: i64,
    sheet: Character,
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        let text = text(format!("{}", self.counter));

        let button1 = button("pressme").on_press(Message::Increment);
        let button2 = button("pressme").on_press(Message::Decrement);
        let c = column![text, button1, button2];

        c.into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => self.counter += 1,
            Message::Decrement => self.counter += -1,
        }
        Task::none()
    }

    fn new() -> Self {
        Self {
            counter: 0,
            sheet: Character::default(),
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
    pub fn view(&self, sheet: Character) -> Element<'_, Message> {
        
    }
}