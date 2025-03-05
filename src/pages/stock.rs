use iced::widget::{column, container};
use iced::Element;

use crate::Message;

pub fn stock() -> Element<'static, Message> {
    container(column![]).into()
}
