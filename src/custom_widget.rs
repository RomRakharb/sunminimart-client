use std::ascii::AsciiExt;
use std::borrow::Cow;

use crate::Message;
use iced::font::Family;
use iced::widget::{self, container, text as iced_text, Container, Text, TextInput};
use iced::Alignment::Center;
use iced::Length::Fill;
use iced::{Font, Pixels, Theme};

pub fn thai_font() -> Font {
    Font {
        family: Family::Name("Anakotmai, Light"),
        weight: iced::font::Weight::Normal,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    }
}

pub fn text<'a>(
    display_value: impl iced_text::IntoFragment<'a> + std::clone::Clone,
    font_size: u32,
) -> Text<'a> {
    let text = iced_text(display_value.clone())
        .size(Pixels(font_size as f32))
        .width(Fill)
        .center();

    match display_value.into_fragment().is_ascii() {
        true => text,
        false => text.shaping(iced_text::Shaping::Advanced),
    }
}

pub fn boxed_text<'a>(
    display_value: impl iced_text::IntoFragment<'a> + std::clone::Clone,
    font_size: u32,
) -> Container<'a, Message> {
    container(text(display_value, font_size + 8))
        .style(|_| container::bordered_box(&Theme::Light))
        .width(Fill)
}

pub fn text_input<'a>(
    id: &'static str,
    display_value: &str,
    font_size: u32,
) -> TextInput<'a, Message> {
    widget::text_input("", display_value)
        .id(id)
        .size(Pixels(font_size as f32))
        .width(Fill)
        .align_x(Center)
}
