use std::fs;
use std::hash::Hash;
use std::ops::Sub;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use iced::futures::{FutureExt, SinkExt, Stream, TryFutureExt};
use iced::theme::Style;
use iced::widget::{self, Column, Container, Text, button, column, container, row, text};

use iced::{Application, Background, Border, Color, Element, Subscription, Task, Theme};
use iced_futures::MaybeSend;
use log::{debug, error, info, warn};
use rfd::{FileDialog, FileHandle};
use rig::client::{Client, Nothing};
use rig::providers::ollama::{self, OllamaExt};
use rustls::crypto::CryptoProvider;
use serenity_agent::MyClient;
use serenity_types::skills::Skill;
use serenity_types::{Character, sheet};
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::sync::{Mutex, mpsc};

mod chat_widget;

fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();
    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
        .run()
        .unwrap();
}

pub struct App {
    counter: i64,
    sheet: Arc<parking_lot::Mutex<Character>>,
    widgets: Vec<SWidget>,
    prev: Vec<Character>,
    client: Option<ClientWrapper<PathBuf>>,
    client_thinking: bool,
    client_lock: bool,
    chat_history: Vec<String>,
    tool_history: Vec<String>,
    path: PathBuf,
    text_box: String,
    tool_recv: Option<Receiver<String>>,
    tool_sender: Sender<String>,
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        let counter_text = text(format!("{}", self.counter));

        let save_button = button("save").on_press(Message::SaveSheet);
        let open_button = button("open").on_press(Message::ChooseFile);
        let reload_button = button("reload").on_press(Message::ReloadSheet);

        let control_buttons = row![save_button, open_button, reload_button];

        let button1 = button("press").on_press(Message::Increment);
        let button2 = button("press").on_press(Message::Decrement);
        let c = column![counter_text, button1, button2];

        let sheet = self.sheet.lock();

        let mut t = sheet.name.clone();

        if self.client.is_none() {
            t.push_str(" Client not connected");
        }

        let title: Text = text(t).size(20).style(|f: &Theme| {
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

        let chat_window = chat_widget::chat_widget(&self);

        let cols = column![title, control_buttons, r, chat_window]
            .spacing(20)
            .padding(20);

        cols.into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => self.counter += 1,
            Message::Decrement => self.counter += -1,
            Message::CreateClient(my_client) => {
                if let Some(m) = my_client {
                    debug!("Created client");
                    self.client = Some(m);
                    self.client_lock = false;
                }
            }
            Message::TextChanged(s) => self.text_box = s,
            Message::PromptSent => {
                if self.text_box.starts_with("/") {
                    match self.text_box.replace("/", "").as_str() {
                        "save" => return Task::done(Message::SaveSheet),
                        "reload" => return Task::done(Message::ReloadSheet),
                        _ => (),
                    }
                    self.text_box.clear();
                } else {
                    if let Some(ref client) = self.client {
                        let s = self.text_box.clone();
                        self.text_box.clear();
                        self.client_thinking = true;
                        let client = client.clone();
                        return Task::future(async move {
                            let s = s;
                            let c = client.clone();
                            c.prompt(s).await
                        })
                        .map(|f| match f {
                            Ok(e) => e,
                            Err(e) => {
                                error!("{:?}", e);
                                "Error".to_string()
                            }
                        })
                        .map(Message::PromptFinished);
                    }
                }
            }
            Message::PromptFinished(e) => {
                self.client_thinking = false;

                self.chat_history.push(e);
            }
            Message::ToolCalled(e) => self.tool_history.push(e),
            Message::Nothing => (),
            Message::ToolStreamEvent(event) => match event {
                Event::Ready(sender) => {
                    info!("Channel sent");
                    let p = self
                        .tool_recv
                        .take()
                        .expect("This should always be there once");
                    return Task::perform(async move { sender.send(p).await }, |f| {
                        f.expect("Sender failed");
                        Message::Nothing
                    });
                }
                Event::Item(e) => self.tool_history.push(e),
            },
            Message::SheetLoaded(character) => {
                *self.sheet.lock() = character;

                let s = self.tool_sender.clone();
                return Task::future(async move { s.send("Sheet reloaded".to_string()).await })
                    .map(|_| Message::Nothing);
                // TODO add history for undo
            }
            Message::SheetSaved(_) => {
                let s = self.tool_sender.clone();
                return Task::future(async move { s.send("Sheet saved".to_string()).await })
                    .map(|_| Message::Nothing);
            }
            Message::FileSelected(file_handle) => {
                let future = load(file_handle.path().to_path_buf());
                return Task::perform(future, |f| match f {
                    Ok(e) => Message::SheetLoaded(e),
                    Err(_) => Message::Nothing,
                });
            }
            Message::ChooseFile => {
                return Task::future(file_dialog()).map(|f| match f {
                    Some(e) => Message::FileSelected(e),
                    None => Message::Nothing,
                });
            }
            Message::SaveSheet => {
                let f = { self.sheet.lock().clone() };
                let p = self.path.clone();
                return Task::future(async move {
                    let f = f;
                    save(&f, p).await
                })
                .map(|f| {
                    if let Ok(m) = f {
                        Message::SheetSaved(m)
                    } else {
                        Message::Nothing
                    }
                });
            }
            Message::ReloadSheet => {
                return Task::perform(load(self.path.clone()), |f| {
                    if let Ok(c) = f {
                        Message::SheetLoaded(c)
                    } else {
                        Message::Nothing
                    }
                });
            }
        }

        if self.client.is_none() && !self.client_lock {
            let path = self.path.clone();
            let sheet = self.sheet.clone();
            info!("Loading client");
            self.client_lock = true;
            let sender = self.tool_sender.clone();
            Task::future(async {
                let client = ollama::Client::new(Nothing).expect("Ollama error");

                match ClientWrapper::new(path, client, sheet, sender).await {
                    Ok(client) => Some(client),
                    Err(e) => {
                        error!("{:?}", e);
                        None
                    }
                }
            })
            .map(Message::CreateClient)
        } else {
            Task::none()
        }
    }

    fn new() -> Self {
        let file = FileDialog::new()
            .set_directory("../")
            .add_filter("json", &["json"])
            .pick_file();

        let sheet = if let Some(ref f) = file {
            let s = fs::read_to_string(&f).expect("Invalid file path");
            let c: Character = serde_json::from_str(&s).unwrap_or_default();
            if c.name.is_empty() {
                warn!("Loading likely failed!");
            }
            c
        } else {
            Character::default()
        };

        let sheet = Arc::new(parking_lot::Mutex::new(sheet));

        let (sender, recv) = channel(20);

        CryptoProvider::install_default(rustls::crypto::aws_lc_rs::default_provider())
            .expect("failed to install crypto nonsense.");

        Self {
            counter: 0,
            sheet,
            widgets: vec![
                SWidget::Health(HealthWidget),
                SWidget::Scores(Scores),
                SWidget::Skills(SkillsWidget),
            ],
            prev: vec![],
            client_lock: false,
            client_thinking: false,
            client: None,
            path: file.unwrap_or(PathBuf::from("/")),
            chat_history: vec![],
            tool_history: vec![],
            text_box: String::new(),
            tool_recv: Some(recv),
            tool_sender: sender,
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![];

        let s = Subscription::run(some_worker).map(Message::ToolStreamEvent);
        subs.push(s);

        Subscription::batch(subs)
        // }
    }
}
#[derive(Clone)]
enum Event {
    Ready(Sender<Receiver<String>>),
    Item(String),
}
fn some_worker() -> impl Stream<Item = Event> {
    let (sender, mut recv) = channel::<Receiver<String>>(10);
    iced::stream::channel(20, async move |mut output| {
        output.send(Event::Ready(sender)).await.expect("Send error");

        tokio::time::sleep(Duration::from_millis(300)).await;

        let mut recv = recv.recv().await.expect("We should always get a value");

        loop {
            let val = recv.recv().await;

            if let Some(val) = val {
                let _ = output.send(Event::Item(val)).await;
            } else {
                error!("Channel closed");
                break;
            }
        }
    })
}

#[derive(Clone)]
pub enum Message {
    Increment,
    Decrement,
    CreateClient(Option<ClientWrapper<PathBuf>>),
    TextChanged(String),
    PromptSent,
    PromptFinished(String),
    ToolCalled(String),
    Nothing,
    ToolStreamEvent(Event),
    SheetLoaded(Character),
    SheetSaved(()),
    FileSelected(FileHandle),
    SaveSheet,
    ReloadSheet,
    ChooseFile,
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
#[derive(Clone)]
pub struct ClientWrapper<T: AsRef<Path> + Clone> {
    pub inner: Arc<Mutex<MyClient<T>>>,
}

impl<T: AsRef<Path> + Clone> ClientWrapper<T> {
    pub async fn prompt<S: AsRef<str>>(&self, s: S) -> anyhow::Result<String> {
        let mut guard = self.inner.lock().await;

        guard.prompt(s).await
    }

    pub async fn new(
        path: T,
        client: Client<OllamaExt>,
        sheet: Arc<parking_lot::Mutex<Character>>,
        sender: Sender<String>,
    ) -> anyhow::Result<Self> {
        let inner = MyClient::new(path, client, sheet, sender).await?;
        Ok(Self {
            inner: Arc::new(Mutex::new(inner)),
        })
    }
}

async fn save<T: AsRef<Path>>(sheet: &Character, path: T) -> anyhow::Result<()> {
    let s = serde_json::to_string_pretty(&sheet)?;
    tokio::fs::write(path.as_ref(), s).await?;
    Ok(())
}

async fn load<T: AsRef<Path>>(path: T) -> anyhow::Result<Character> {
    let s = tokio::fs::read_to_string(path.as_ref()).await?;
    let c = serde_json::from_str::<Character>(&s)?;
    Ok(c)
}

async fn file_dialog() -> Option<FileHandle> {
    let res = rfd::AsyncFileDialog::new()
        .add_filter("json", &["json"])
        .set_directory("/")
        .pick_file()
        .await;
    res
}
