use crate::Message;
use iced::{
    border,
    widget::{column, container, text, text_input, Column, Text, TextInput},
    Alignment::Center,
    Border, Color,
    Length::Fill,
    Theme,
};

pub fn labeled_value<T: std::fmt::Display>(label: String, value: &T) -> Column<'static, Message> {
    column![
        text(label).size(20).width(Fill).center(),
        container(text(format!("{}", value)).size(20).width(Fill).center())
            .style(|_| container::bordered_box(&Theme::Light))
            .width(Fill),
    ]
}

pub fn labeled_text_input<T: std::fmt::Display, F>(
    label: String,
    value: T,
    on_input: F,
    on_submit: Message,
) -> Column<'static, Message>
where
    F: Fn(String) -> Message + 'static,
{
    column![
        text(label).size(20).width(Fill).center(),
        text_input("", &format!("{}", value))
            .size(20)
            .width(Fill)
            .align_x(Center)
            .on_input(on_input)
            .on_submit(on_submit)
    ]
    .align_x(Center)
}
