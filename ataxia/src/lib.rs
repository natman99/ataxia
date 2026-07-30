use std::{collections::BTreeMap, sync::Arc};

use ataxia_types::{Character, ability_score::AbilityScore, skills::Skill};
use iced::{
    Element, Length, Subscription, Task, Theme,
    widget::{button, container, text},
    window::{self, Id},
};

use crate::{
    global::Global,
    views::{BasicMessage, BasicState, basic},
};

mod global;
mod views;

#[derive(Clone, Debug)]
pub enum Message {
    Window(WindowMessage),
    Basic(BasicMessage),
}

#[derive(Clone, Debug)]
pub enum WindowMessage {
    WindowClosed(Id),
    OpenWindow(Id),
}

#[derive(Default, Debug)]
pub struct State {
    view: View,
}

#[derive(Debug, Clone, Default)]
pub enum View {
    #[default]
    Basic,
    Inventory,
    Spells,
}

#[derive(Debug)]
pub struct App {
    windows: BTreeMap<Id, State>,
    global: Option<Arc<Global>>,
    sheet: Option<Character>,
    basic_state: BasicState,
}

impl<'a> App {
    pub fn new() -> (Self, Task<Message>) {
        let (_, open) = window::open(window::Settings::default());
        let mut sheet = Character::default();
        sheet.skills.proficiencies.insert(Skill::Athletics);

        sheet.skills.proficiencies.insert(Skill::Acrobatics);
        sheet.skills.expertise.insert(Skill::Athletics);

        sheet.ability_scores.cha = AbilityScore::new(8, 0);

        sheet.ability_scores.str = AbilityScore::new(16, 0);

        (
            Self {
                windows: BTreeMap::new(),
                global: None,
                sheet: Some(sheet),
                basic_state: Default::default(),
            },
            open.map(WindowMessage::OpenWindow).map(Message::Window),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Window(window_message) => match window_message {
                WindowMessage::WindowClosed(id) => {
                    self.windows.remove(&id);
                    if self.windows.is_empty() {
                        iced::exit()
                    } else {
                        Task::none()
                    }
                }
                WindowMessage::OpenWindow(id) => {
                    self.windows.insert(id, State::default());
                    Task::none()
                }
            },
            Message::Basic(basic_message) => match basic_message {
                BasicMessage::InputChanged(s) => {
                    if s.parse::<i32>().is_ok() || s.is_empty() || s == "-" {
                        self.basic_state.hp_input = s;
                    }
                    Task::none()
                }
                BasicMessage::Heal => {
                    if let Some(s) = &mut self.sheet {
                        if let Ok(n) = self.basic_state.hp_input.parse() {
                            s.health.heal(n);
                        }
                    }
                    Task::done(Message::Basic(BasicMessage::ResetInput))
                }
                BasicMessage::Damage => {
                    if let Some(s) = &mut self.sheet {
                        if let Ok(n) = self.basic_state.hp_input.parse() {
                            s.health.heal(n);
                        }
                    }
                    Task::done(Message::Basic(BasicMessage::ResetInput))
                }
                BasicMessage::ResetInput => {
                    self.basic_state.hp_input.clear();
                    Task::none()
                }
            },
        }
    }

    pub fn view(&'_ self, window_id: Id) -> Element<'_, Message> {
        let Some(state) = self.windows.get(&window_id) else {
            return text!("Some error occured").into();
        };

        let Some(sheet) = &self.sheet else {
            return text!("No sheet loaded").into();
        };

        let c = match state.view {
            View::Basic => basic(sheet, &self.basic_state),
            View::Inventory => todo!(),
            View::Spells => todo!(),
        };

        container(c).center(Length::Fill).into()
    }

    pub fn theme(&self, _window_id: Id) -> Option<Theme> {
        Some(Theme::CatppuccinMacchiato)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        window::close_events()
            .map(WindowMessage::WindowClosed)
            .map(Message::Window)
    }
}
