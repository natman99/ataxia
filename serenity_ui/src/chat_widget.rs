use iced::{
    Border, Color, Element, Length, Theme,
    alignment::Horizontal::Left,
    widget::{self, Column, Container, Scrollable, column, container, row, text},
};

use crate::{App, Message};

pub fn chat_widget<'a>(app: &'a App) -> Element<'a, Message> {
    let tokens = {
        if let Some(ref client) = app.client {
            client.get_tokens()
        } else {
            0
        }
    };

    let context_message = text(format!("Total tokens: {}", tokens));

    let chat_history: Element<_> = {
        let texts = app
            .chat_history
            .iter()
            .map(|f| {
                let c = container(text(f))
                    .center(Length::Shrink)
                    .style(|f: &Theme| {
                        let pair = f.extended_palette().primary.base;
                        let color = f.extended_palette().primary.weak.color;
                        container::Style::default()
                            .background(pair.color)
                            .color(pair.text)
                            .border(Border::default().width(8).rounded(12).color(color))
                    })
                    .padding(20);
                c.into()
            })
            .collect();
        let content = Column::from_vec(texts).spacing(10);
        let s = iced::widget::scrollable(content)
            .height(400)
            .anchor_bottom()
            .width(600)
            .spacing(10);
        s.into()
    };

    let chat_box: Element<_> = {
        let b = iced::widget::text_input("Prompt", &app.text_box)
            .on_input(Message::TextChanged)
            .on_submit(Message::PromptSent);
        b.into()
    };

    let tool_history: Element<_> = {
        let tools = app
            .tool_history
            .iter()
            .map(|f| text(f).into())
            .collect::<Vec<Element<_>>>();
        let i = Column::from_vec(tools).spacing(10).width(100);
        let i = Scrollable::new(i).anchor_bottom();
        let c = Container::new(i).style(|f| {
            let pair = f.extended_palette().secondary.strong;
            widget::container::Style::default()
                .color(pair.text)
                .background(pair.color)
        });

        c.into()
    };

    let top_row = row![context_message];

    let chat_col = column![chat_history, chat_box];

    let chat_window = row![chat_col, tool_history].spacing(20);

    let main_row = column![top_row, chat_window];

    let container = Container::new(main_row)
        .style(|f| {
            let pair = f.extended_palette().primary.base;

            let pair2 = f.extended_palette().secondary.base;
            let i = widget::container::Style::default()
                .border(Border::default().rounded(25).color(pair2.color).width(8))
                .background(pair.color)
                .color(pair.text);

            i
        })
        .padding(20);
    container.into()
}
