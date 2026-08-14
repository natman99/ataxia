use std::{
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

use ataxia_scripting::Interface;
use ataxia_types::Character;
use iced::{
    Element, Length, Subscription, Task, Theme,
    futures::{SinkExt, Stream},
    stream,
    widget::{Container, button, column, container, row, text},
    window::{self, Id},
};
use log::warn;
use notify_debouncer_full::{
    DebounceEventResult, new_debouncer,
    notify::{EventKind, RecursiveMode, event::ModifyKind},
};
use rfd::FileHandle;
use tokio::{fs, io, task};

use crate::views::{BasicMessage, BasicState, InventoryState};

mod views;

#[derive(Clone, Debug)]
pub enum Message {
    Window(WindowMessage),
    Basic(BasicMessage),
    Meter(MeterMessage),
    OpenFilePicker,
    FilePicked(Option<FileHandle>),
    FileLoaded {
        main: Arc<io::Result<String>>,
        backup: Option<Arc<io::Result<String>>>,
    },
    FileModified,
    WatcherCreated(tokio::sync::mpsc::Sender<PathBuf>),
    AutoSave(Instant),
    ViewChanged(View, Id),
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

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy)]
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
    // global: Option<Arc<Global>>,
    sheet: Option<Character>,
    basic_state: BasicState,
    inventory_state: InventoryState,
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
                // global: None,
                sheet: None,
                basic_state: Default::default(),
                sheet_path: None,
                sheet_mode: Default::default(),
                rhai_interface: Interface::new(),
                inventory_state: Default::default(),
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
                        println!("Closing window");
                        if let Some(ref s) = self.sheet {
                            let c = self
                                .sheet_path
                                .as_ref()
                                .expect("Always a path when a character exists");
                            let c = get_backup_path(&c);
                            let contents =
                                serde_json::to_string_pretty(s).expect("Should never fail");
                            println!("Saving file");

                            self.sheet_path = None;

                            let start = Task::future(async move { fs::write(c, contents).await });
                            // TODO kill the notify thread and exit properly
                            start.then(|_| std::process::exit(0))
                        } else {
                            std::process::exit(0)
                        }
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
                            Task::perform(t, |f| Message::FileLoaded {
                                main: Arc::new(f),
                                backup: None,
                            })
                        }
                        Some(_) => {
                            let f = f.path().to_owned();
                            self.sheet_path = Some(f.clone());

                            let t2 = fs::read_to_string(get_backup_path(f.as_path()));
                            let t = fs::read_to_string(f);
                            let t = async move {
                                let t = t.await;
                                let t2 = t2.await;
                                (t, t2)
                            };
                            Task::perform(t, |(main, backup)| Message::FileLoaded {
                                main: Arc::new(main),
                                backup: Some(Arc::new(backup)),
                            })
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
            Message::FileLoaded { main, backup } => {
                println!("File loaded");
                match &*main {
                    Ok(f) => match self.sheet_mode {
                        Mode::Rhai => {
                            let second = if let Some(backup) = backup
                                && let Ok(b) = Arc::try_unwrap(backup)
                                    .expect("There should only ever be one copy")
                                && let Ok(b) = serde_json::from_str::<Character>(&b)
                            {
                                Some(b)
                            } else {
                                warn!("Failed to load file backup");
                                self.sheet.clone()
                            };
                            match self.rhai_interface.execute(f, second) {
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
                    Task::perform(t, |f| Message::FileLoaded {
                        main: Arc::new(f),
                        backup: None,
                    })
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
            Message::AutoSave(_instant) => {
                let Some(ref sheet) = self.sheet else {
                    return Task::none();
                };
                let r = serde_json::to_string_pretty(&sheet);
                if let Ok(r) = r {
                    let path = get_backup_path(
                        self.sheet_path
                            .as_ref()
                            .expect("We always have this when we have a character")
                            .clone(),
                    );
                    let task = async move {
                        log::info!("Saving file");
                        let _ = fs::write(path, r).await;
                    };
                    return Task::future(task).discard();
                } else {
                    warn!("Json tostring failed");
                }

                Task::none()
            }
            Message::ViewChanged(view, id) => {
                if let Some(w) = self.windows.get_mut(&id) {
                    w.view = view;
                    Task::none()
                } else {
                    Task::none()
                }
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
            View::Inventory => self.inventory_state.view(&sheet),
            View::Spells => todo!(),
        };
        let c = column![mode_switcher(self, window_id), c].spacing(8);
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

        let auto_save = iced::time::every(Duration::from_secs(5)).map(Message::AutoSave);

        Subscription::batch([w, watcher, auto_save])
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

            let Ok(mut debouncer) = new_debouncer(
                Duration::from_secs(1),
                None,
                move |res: DebounceEventResult| {
                    if let Ok(event) = res {
                        let _ = event_tx.send(event);
                    }
                },
            ) else {
                log::error!("File watcher failed!");
                return;
            };

            if let Err(e) = debouncer.watch(&path, RecursiveMode::NonRecursive) {
                println!("Watcher failed: {e:?}");
                return;
            }

            while let Ok(event) = event_rx.recv() {
                if bridge_tx.blocking_send(event).is_err() {
                    break;
                }
            }
        });

        while let Some(events) = bridge_rx.recv().await {
            for event in events {
                let send = match event.kind {
                    EventKind::Modify(_) => true,
                    EventKind::Create(_) => true,
                    EventKind::Remove(_) => true,
                    _ => false,
                };

                // println!("{event:?}");
                if send {
                    let _ = output.send(Message::FileModified).await;
                }
            }
        }
    })
}
fn get_backup_path(f: impl AsRef<Path>) -> String {
    format!("{}.json", f.as_ref().display())
}

fn mode_switcher(app: &'_ App, id: Id) -> Container<'_, Message> {
    let b1 = button("Main").on_press_maybe(if app.windows[&id].view == View::Basic {
        None
    } else {
        Some(Message::ViewChanged(View::Basic, id))
    });
    let b2 = button("Inventory").on_press_maybe(if app.windows[&id].view == View::Inventory {
        None
    } else {
        Some(Message::ViewChanged(View::Inventory, id))
    });

    container(row![b1, b2].spacing(8)).padding(12)
}

// fn mode_style(theme: &Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
//     if status == iced::widget::button::Status::Disabled {
//         Style::default().
//     }
// }
