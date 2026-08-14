use ataxia_types::{Inventory, Item, spells::Spell};
use iced::{
    Element, Length,
    widget::{
        Column, Row, container, row,
        table::{self, Table},
        text,
    },
};

use crate::Message;

#[derive(Clone, Debug, PartialEq)]
pub struct InventoryMessage {}

#[derive(Debug, Default)]
pub struct InventoryState {
    popup: Option<Spell>,
}

impl<'a> InventoryState {
    pub fn view(&'a self, inventory: &'a Inventory) -> Element<'a, Message> {
        let mut rows = vec![];
        // Item {
        //     name: todo!(),
        //     description: todo!(),
        //     quantity: todo!(),
        //     roll: todo!(),
        //     source: todo!(),
        //     feature: todo!(),
        // }

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
            let row = (name, *quantity, roll, feature);
            rows.push(row);
        }

        let table_cols = [
            table::column("Name", |e: (&str, _, _, _)| text(e.0)),
            table::column("Quantity", |e: (_, i32, _, _)| text(e.1)),
            table::column("Roll", |e: (_, _, String, _)| text(e.2)),
            table::column("Feature", |e: (_, _, _, String)| text(e.3)),
        ];

        let table = Table::new(table_cols, rows);

        container(table).center(Length::Fill).into()
    }
}
