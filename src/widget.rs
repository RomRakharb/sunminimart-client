use crate::Message;
use iced::{
    border,
    widget::{column, container, text, text_input, Column, Text, TextInput},
    Alignment::Center,
    Border, Color,
    Length::Fill,
    Theme,
};

pub fn LabeledTextBox<T: std::fmt::Display>(
    label: String,
    placeholder: String,
    value: &T,
) -> Column<'static, Message> {
    column![
        text(label),
        text_input(&placeholder, &format!("{}", value)).align_x(Center)
    ]
}

pub fn labeled_value<T: std::fmt::Display>(label: String, value: &T) -> Column<'static, Message> {
    column![
        text(label).size(20).width(Fill).center(),
        container(text(format!("{}", value)).size(50).width(Fill).center())
            .style(|_| container::bordered_box(&Theme::Light))
            .width(Fill),
    ]
}
