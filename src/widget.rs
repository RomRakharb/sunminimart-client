use crate::Message;
use iced::{
    widget::{column, container, text, text_input, Column, Container},
    Alignment::Center,
    Length::Fill,
    Pixels, Theme,
};

pub fn labeled_value<T: std::fmt::Display>(
    thai_label: &'static str,
    english_label: &'static str,
    value: &T,
) -> Column<'static, Message> {
    column![
        text(thai_label).size(40).width(Fill).center(),
        text(english_label).size(40).width(Fill).center(),
        container(text(format!("{}", value)).size(50).width(Fill).center())
            .style(|_| container::bordered_box(&Theme::Light))
            .width(Fill),
    ]
}

pub fn labeled_text_box<T: std::fmt::Display>(
    label: &'static str,
    value: T,
) -> Column<'static, Message> {
    column![
        text(label).size(25).width(Fill).center(),
        text_input("", &format!("{}", value))
            .size(25)
            .width(Fill)
            .align_x(Center)
    ]
    .align_x(Center)
}

pub fn labeled_text_input<T: std::fmt::Display, F>(
    label: &'static str,
    font_size: u32,
    value: T,
    on_input: F,
    on_submit: Message,
) -> Column<'static, Message>
where
    F: Fn(String) -> Message + 'static,
{
    column![
        text(label)
            .size(Pixels(font_size as f32))
            .width(Fill)
            .center(),
        text_input("", &format!("{}", value))
            .size(Pixels(font_size as f32))
            .width(Fill)
            .align_x(Center)
            .on_input(on_input)
            .on_submit(on_submit)
    ]
    .align_x(Center)
}
