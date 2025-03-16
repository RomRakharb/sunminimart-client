use crate::Message;
use iced::{
    font::Family,
    widget::{column, container, row, text, text_input},
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
    let text = container(
        text(format!("{}", value))
            .size(Pixels((font_size + 8) as f32)) // +8 to adjust size
            .width(Fill)
            .center(),
    )
    .style(|_| container::bordered_box(&Theme::Light))
    .width(Fill);

    match label_position {
        Position::Top => container(column![label, text]).into(),
        Position::Bottom => container(column![text, label]).into(),
        Position::Left => container(row![label, text]).into(),
        Position::Right => container(row![text, label]).into(),
    }
}

pub fn labeled_text_input<T: std::fmt::Display, F>(
    label_value: &'static str,
    font_size: u32,
    label_position: Position,
    value: T,
    on_input: F,
    on_submit: Message,
) -> Element<'static, Message>
where
    F: Fn(String) -> Message + 'static,
{
    let label = text(label_value).size(Pixels(font_size as f32)).center();
    let input = text_input("", &format!("{}", value))
        .size(Pixels(font_size as f32))
        .align_x(Center)
        .on_input(on_input)
        .on_submit(on_submit);

    match label_position {
        Position::Top => container(column![label.width(Fill), input.width(Fill)]).into(),
        Position::Bottom => container(column![input, label]).into(),
        Position::Left => container(row![label, input].spacing(25)).into(),
        Position::Right => container(row![input, label]).into(),
    }
}
