use iced::{
    Border, Color, Element,
    widget::{self, Column, Container, column, container, row, text},
};

use crate::{App, Message};

pub fn chat_widget<'a>(app: &'a App) -> Element<'a, Message> {
    let chat_history: Element<_> = {
        let texts = app
            .chat_history
            .iter()
            .map(|f| container(f).style(container::rounded_box).into())
            .collect();
        let content = Column::from_vec(texts);
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
        let c = Container::new(i).style(|f| {
            let pair = f.extended_palette().secondary.strong;
            widget::container::Style::default()
                .color(pair.text)
                .background(pair.color)
        });

        c.into()
    };

    let chat_window = row![column![chat_history, chat_box], tool_history];

    let container = Container::new(chat_window)
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
