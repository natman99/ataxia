use ataxia_types::{Character, Inventory, Item, spells::Spell};
use iced::{
    Element, Length,
    widget::{
        Button, Checkbox, Column, Row, button, checkbox, container, row, scrollable,
        table::{self, Table},
        text,
    },
};

use crate::{Message, views::basic::Pinnable};

#[derive(Clone, Debug, PartialEq)]
pub struct InventoryMessage {}

#[derive(Debug, Default)]
pub struct InventoryState {
    popup: Option<Spell>,
}

impl<'a> InventoryState {
    pub fn view(&'a self, sheet: &'a Character) -> Element<'a, Message> {
        let mut rows = vec![];
        let inventory = &sheet.inventory;

        for item in inventory.0.values() {
            let name = item.name.as_str();
            let quantity = &item.quantity;
            let roll = if let Some(r) = item.roll.as_ref() {
                r.to_string()
            } else {
                "-".to_string()
            };
            let feature = if let Some(f) = item.feature.as_ref() {
                let a: Vec<String> = f.effects.iter().map(|f| f.to_string()).collect();
                a.join(", ")
            } else {
                "-".to_string()
            };

            let to_hit = if let Some(f) = item.to_hit(&sheet.ability_scores, &sheet.skills) {
                format!("{f:+}")
            } else {
                "-".to_string()
            };

            let b = Pinnable::Item(item.clone());

            let row = (name, *quantity, roll, feature, to_hit, b);
            rows.push(row);
        }

        let table_cols = [
            table::column("Pin", |e: (&str, i32, String, String, String, Pinnable)| {
                button("Pin").on_press(Message::Basic(crate::views::BasicMessage::Pin(e.5)))
            }),
            table::column("Name", |e: (&str, _, _, _, _, _)| text(e.0)),
            table::column("Quantity", |e: (_, i32, _, _, _, _)| text(e.1)),
            table::column("Roll", |e: (_, _, String, _, _, _)| text(e.2)),
            table::column("To hit", |e: (_, _, _, _, String, _)| text(e.4)),
            table::column("Feature", |e: (_, _, _, String, _, _)| text(e.3)),
        ];

        let table = Table::new(table_cols, rows);

        let s = scrollable(table);

        container(s).center(Length::Fill).padding(12).into()
    }
}
