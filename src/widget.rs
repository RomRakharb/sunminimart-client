use crate::Message;
use iced::{
    widget::{column, container, text, text_input, Column, Container},
    Alignment::Center,
    Length::Fill,
    Theme,
};

pub fn labeled_value<T: std::fmt::Display>(label: String, value: &T) -> Column<'static, Message> {
    column![
        text(label).size(20).width(Fill).center(),
        container(text(format!("{}", value)).size(27).width(Fill).center())
            .style(|_| container::bordered_box(&Theme::Light))
            .width(Fill),
    ]
}

pub fn labeled_text_box<T: std::fmt::Display>(label: String, value: T) -> Column<'static, Message> {
    column![
        text(label).size(20).width(Fill).center(),
        text_input("", &format!("{}", value))
            .size(20)
            .width(Fill)
            .align_x(Center)
    ]
    .align_x(Center)
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
