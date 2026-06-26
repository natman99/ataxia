use std::{
    path::PathBuf,
    sync::{Arc, Once},
};

use clap::{Parser, clap_derive::ValueEnum};
use egui::{DragValue, mutex::Mutex};
use serenity_types::{
    Item,
    database::spell::{Dc, DcType, Spell},
    feature::Feature,
    roll::Roll,
};
fn main() {
    let cli = Cli::parse();
    let native_options = eframe::NativeOptions::default();
    let data = match cli.kind {
        Kind::Spell => KindData::Spell(Default::default()),
        Kind::Item => KindData::Item(Default::default()),
        Kind::Feature => KindData::Feature(Default::default()),
    };

    let data = Arc::new(Mutex::new(data));
    eframe::run_native(
        "Dnd",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, data.clone())))),
    )
    .unwrap();

    let e = match &*data.lock() {
        KindData::Spell(spell) => serde_json::to_string_pretty(&spell),
        KindData::Item(item) => serde_json::to_string_pretty(&item),
        KindData::Feature(feature) => serde_json::to_string_pretty(&feature),
    };

    println!("{}", e.unwrap_or("failed".to_string()));
}

struct App {
    data: Arc<Mutex<KindData>>,
    check_1: bool,
    text_buf: String,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>, data: Arc<Mutex<KindData>>) -> Self {
        Self {
            data,
            check_1: false,
            text_buf: String::new(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label("Dnd");
            let mut data = self.data.lock();
            match *data {
                KindData::Spell(_) => ui.label("Spell"),
                KindData::Item(_) => ui.label("Item"),
                KindData::Feature(_) => ui.label("Feature"),
            };

            match &mut *data {
                KindData::Spell(spell) => {
                    ui.label("Name");
                    if ui.text_edit_singleline(&mut spell.name).changed() {
                        spell.index = spell.name.to_lowercase().replace(" ", "_");
                    }
                    ui.checkbox(&mut spell.concentration, "Concentration");

                    let mut buf = spell.desc.join("\n");

                    ui.label("Description");
                    if ui.text_edit_multiline(&mut buf).changed() {
                        spell.desc = buf.split("\n").map(|f| f.to_string()).collect();
                    };

                    ui.label("Range");
                    ui.text_edit_singleline(&mut spell.range);

                    ui.label("Cast time");
                    ui.text_edit_singleline(&mut spell.casting_time);

                    ui.label("Duration");
                    ui.text_edit_singleline(&mut spell.duration);

                    let level = egui::widgets::DragValue::new(&mut spell.level)
                        .clamp_existing_to_range(true)
                        .range(0..=9)
                        .prefix("lvl");
                    ui.add(level);

                    if ui.checkbox(&mut self.check_1, "Has DC").changed() {
                        if !self.check_1 {
                            spell.dc = None;
                            spell.damage = Some(Default::default())
                        } else {
                            spell.dc = Some(Default::default());
                            spell.damage = None;
                        }
                    }

                    static SET: Once = Once::new();

                    SET.call_once(|| {
                        spell.damage = Some(Default::default());

                        let dmg = &mut spell.damage.as_mut().unwrap();

                        dmg.damage_at_slot_level = Some(Default::default());
                    });

                    if self.check_1 {
                        if spell.dc.is_none() {
                            spell.dc = Some(Dc {
                                dc_type: DcType {
                                    index: "str".to_string(),
                                    name: "Str".to_string(),
                                    url: "".to_string(),
                                },
                                dc_success: "Half".to_string(),
                                desc: Some("Default".to_string()),
                            })
                        }
                        let selected = spell.dc.as_mut().expect("Always some");

                        ui.label("DC type (str, dex, ect)");
                        if ui
                            .text_edit_singleline(&mut selected.dc_type.name)
                            .changed()
                        {
                            selected.dc_type.index = selected.dc_type.name.to_lowercase();
                        };

                        ui.label("DC success (half, none)");
                        ui.text_edit_singleline(&mut selected.dc_success);
                        if selected.desc.is_none() {
                            selected.desc = Some(String::new())
                        }

                        // "Name", "Time", "Range", "Hit/DC", "Effect", "Duration", "Level",
                    } else {
                        let dmg = &mut spell.damage.as_mut().unwrap();

                        let d = dmg.damage_at_slot_level.as_mut().unwrap();

                        if d.n1.is_none() {
                            d.n1 = Some("1d4".to_string());
                        }

                        ui.label("Damage");
                        ui.text_edit_singleline(d.n1.as_mut().expect("Always some"));
                    }
                }
                KindData::Item(item) => {
                    if ui.text_edit_singleline(&mut item.name).changed() {
                        item.id = item.name.replace(" ", "_").replace("-", "_").to_lowercase();
                    }

                    ui.text_edit_multiline(&mut item.description);

                    ui.add(
                        DragValue::new(&mut item.quantity)
                            .range(1..=1000)
                            .prefix("count: "),
                    );

                    if ui.checkbox(&mut self.check_1, "Has roll").changed() {
                        if self.check_1 {
                            item.roll = Some(Roll::try_from("1d20").expect("Should not fail"));
                            self.text_buf = item.roll.as_ref().unwrap().to_string();
                        } else {
                            item.roll = None;
                            self.text_buf.clear();
                        }
                    }

                    if self.check_1 {
                        let t = egui::widgets::TextEdit::singleline(&mut self.text_buf);
                        if ui.add(t).lost_focus() {
                            match Roll::try_from(self.text_buf.as_str()) {
                                Ok(e) => item.roll = Some(e),
                                Err(_) => (),
                            }
                        }
                    }
                }
                KindData::Feature(feature) => todo!(),
            }
        });
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(value_enum)]
    kind: Kind,
    file: Option<PathBuf>,
}

#[derive(ValueEnum, Debug, Clone)]
enum Kind {
    Spell,
    Item,
    Feature,
}

enum KindData {
    Spell(Spell),
    Item(Item),
    Feature(Feature),
}
