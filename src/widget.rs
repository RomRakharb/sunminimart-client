use crate::Message;
use iced::{
    widget::{column, text, text_input, Column, Text, TextInput},
    Alignment::Center,
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
