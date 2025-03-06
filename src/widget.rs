use crate::Message;
use iced::{
    font::Family,
    widget::{column, container, row, text, text_input, Column},
    Alignment::Center,
    Element, Font,
    Length::Fill,
    Pixels, Theme,
};

#[derive(Default)]
pub enum Position {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

pub fn thai_font() -> Font {
    Font {
        family: Family::Name("Sarabun"),
        weight: iced::font::Weight::Normal,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    }
}

pub fn labeled_value<T: std::fmt::Display>(
    label_value: &'static str,
    font_size: u32,
    label_position: Position,
    value: &T,
) -> Element<'static, Message> {
    let label = text(label_value)
        .size(Pixels(font_size as f32))
        .width(Fill)
        .center();
    let text = container(text(format!("{}", value)).size(50).width(Fill).center())
        .style(|_| container::bordered_box(&Theme::Light))
        .width(Fill);

    match label_position {
        Position::Top => container(column![label, text]).into(),
        Position::Bottom => container(column![text, label]).into(),
        Position::Left => container(row![text, label]).into(),
        Position::Right => container(row![label, text]).into(),
    }
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
