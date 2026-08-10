use std::{
    collections::BTreeMap,
    env,
    ops::Deref,
    path::PathBuf,
    sync::Arc,
};

use ataxia_scripting::Interface;
use ataxia_types::Character;
use iced::{
    Element, Length, Subscription, Task, Theme,
    futures::{FutureExt, SinkExt, Stream},
    stream,
    widget::{button, column, container, text},
    window::{self, Id},
};
use notify::{
    EventKind, RecommendedWatcher, Watcher,
    event::{AccessKind, AccessMode, DataChange},
};
use rfd::FileHandle;
use tokio::{fs, io, task};

use crate::{
    global::Global,
    views::{BasicMessage, BasicState},
};

mod global;
mod views;

#[derive(Clone, Debug)]
pub enum Message {
    Window(WindowMessage),
    Basic(BasicMessage),
    Meter(MeterMessage),
    OpenFilePicker,
    FilePicked(Option<FileHandle>),
    FileLoaded(Arc<io::Result<String>>),
    FileModified,
    WatcherCreated(tokio::sync::mpsc::Sender<PathBuf>),
}

#[derive(Clone, Debug)]
pub enum WindowMessage {
    WindowClosed(Id),
    OpenWindow(Id),
}

#[derive(Clone, Debug)]
pub enum MeterMessage {
    Spend(String),
    Restore(String),
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    Rhai,
    Json,
}

#[derive(Debug)]
pub struct App {
    windows: BTreeMap<Id, State>,
    global: Option<Arc<Global>>,
    sheet: Option<Character>,
    basic_state: BasicState,
    sheet_path: Option<PathBuf>,
    sheet_mode: Mode,
    rhai_interface: Interface,
}

impl<'a> App {
    pub fn new() -> (Self, Task<Message>) {
        let (_, open) = window::open(window::Settings::default());
        (
            Self {
                windows: BTreeMap::new(),
                global: None,
                sheet: None,
                basic_state: Default::default(),
                sheet_path: None,
                sheet_mode: Default::default(),
                rhai_interface: Interface::new(),
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
            Message::Basic(basic_message) => {
                if let Some(sheet) = &mut self.sheet {
                    self.basic_state.update(basic_message, sheet)
                } else {
                    Task::none()
                }
            }
            Message::OpenFilePicker => {
                let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                let t = rfd::AsyncFileDialog::new()
                    .add_filter("rhai", &["rhai"])
                    .add_filter("json", &["json"])
                    .set_title("Select Character")
                    .set_directory(current_dir)
                    .pick_file();
                println!("Opening file picker");
                Task::perform(t, Message::FilePicked)
            }
            Message::FilePicked(f) => {
                println!("File picked");
                if let Some(f) = f {
                    match f.path().extension().and_then(|f| f.to_str()) {
                        Some("json") => {
                            self.sheet_mode = Mode::Json;
                            let f = f.path().to_owned();
                            self.sheet_path = Some(f.clone());

                            let t = fs::read_to_string(f);
                            Task::perform(t, |f| Message::FileLoaded(Arc::new(f)))
                        }
                        Some(_) => {
                            let f = f.path().to_owned();
                            self.sheet_path = Some(f.clone());

                            let t = fs::read_to_string(f);
                            Task::perform(t, |f| Message::FileLoaded(Arc::new(f)))
                        }
                        None => {
                            println!("Invalid file");
                            Task::none()
                        }
                    }
                } else {
                    println!("No file chosen");
                    Task::none()
                }
            }
            Message::FileLoaded(file) => {
                println!("File loaded");
                match &*file {
                    Ok(f) => match self.sheet_mode {
                        Mode::Rhai => {
                            match self.rhai_interface.execute(f, self.sheet.clone()) {
                                Ok(sheet) => self.sheet = Some(sheet),
                                Err(e) => println!("Rhai failed: {e:?}"),
                            }
                            Task::none()
                        }
                        Mode::Json => {
                            match serde_json::from_str::<Character>(f) {
                                Ok(c) => self.sheet = Some(c),
                                Err(e) => println!("Json failed: {e:?}"),
                            }
                            Task::none()
                        }
                    },
                    Err(e) => {
                        println!("{e:?}");
                        Task::none()
                    }
                }
            }
            Message::FileModified => {
                println!("Reloading file");

                if let Some(e) = &self.sheet_path {
                    let e = e.to_owned();
                    let t = fs::read_to_string(e);
                    Task::perform(t, |f| Message::FileLoaded(Arc::new(f)))
                } else {
                    Task::none()
                }
            }
            Message::WatcherCreated(sender) => {
                if let Some(f) = &self.sheet_path {
                    let f = f.to_owned();
                    Task::future(async move {
                        let _ = sender.send(f).await;
                    })
                    .discard()
                } else {
                    println!("No path to send to watcher!");
                    Task::none()
                }
            }
            Message::Meter(meter_message) => {
                let Some(sheet) = &mut self.sheet else {
                    return Task::none();
                };

                let meters = &mut sheet.meters;
                match meter_message {
                    MeterMessage::Spend(s) => {
                        if let Some(m) = meters.meters.get_mut(&s) {
                            m.spend();
                        }
                    }
                    MeterMessage::Restore(s) => {
                        if let Some(m) = meters.meters.get_mut(&s) {
                            m.restore();
                        }
                    }
                }

                Task::none()
            }
        }
    }

    pub fn view(&'_ self, window_id: Id) -> Element<'_, Message> {
        let Some(state) = self.windows.get(&window_id) else {
            return text!("Some error occured").into();
        };

        let Some(sheet) = &self.sheet else {
            let b = button("Select file").on_press(Message::OpenFilePicker);
            let c = column![text("No sheet loaded"), b];
            let c = container(c).center(Length::Fill);
            return c.into();
        };

        let c = match state.view {
            View::Basic => self.basic_state.view(sheet),
            View::Inventory => todo!(),
            View::Spells => todo!(),
        };

        container(c).center(Length::Fill).into()
    }

    pub const fn theme(&self, _window_id: Id) -> Option<Theme> {
        Some(Theme::CatppuccinMacchiato)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let w = window::close_events()
            .map(WindowMessage::WindowClosed)
            .map(Message::Window);
        let watcher = if self.sheet_path.is_some() {
            Subscription::run(create_file_watcher)
        } else {
            Subscription::none()
        };
        Subscription::batch([w, watcher])
    }
}

fn create_file_watcher() -> impl Stream<Item = Message> {
    println!("Creating watcher");
    stream::channel(100, async move |mut output| {
        let path = {
            let (tx, mut rx) = tokio::sync::mpsc::channel(1);
            output
                .send(Message::WatcherCreated(tx))
                .await
                .expect("Got file");
            rx.recv().await
        };

        println!("Watcher got file");

        let Some(path) = path else {
            println!("Watcher init failed");
            return;
        };

        // Spawn a task that owns the watcher and bridges its blocking std::sync channel
        // into an async tokio channel. This avoids blocking the main async task,
        // which would prevent output.send() from being polled.
        let (bridge_tx, mut bridge_rx) = tokio::sync::mpsc::channel(16);
        task::spawn_blocking(move || {
            let (event_tx, event_rx) = std::sync::mpsc::channel();

            let Ok(mut watcher) = RecommendedWatcher::new(
                move |res| {
                    if let Ok(event) = res {
                        let _ = event_tx.send(event);
                    }
                },
                notify::Config::default(),
            ) else {
                return;
            };
            if let Err(e) = watcher.watch(&path, notify::RecursiveMode::NonRecursive) {
                println!("Watcher failed: {e:?}");
                return;
            }

            while let Ok(event) = event_rx.recv() {
                if bridge_tx.blocking_send(event).is_err() {
                    break;
                }
            }
        });

        while let Some(event) = bridge_rx.recv().await {
            let send = match event.kind {
                EventKind::Access(AccessKind::Close(AccessMode::Write)) => true,
                EventKind::Modify(notify::event::ModifyKind::Data(DataChange::Content)) => true,
                _ => false,
            };
            if send {
                println!("{event:?}");
                if event.paths.first().is_some() {
                    println!("File changed");
                    let _ = output.send(Message::FileModified).await;
                }
            }
        }
    })
}
