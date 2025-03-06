use iced::widget::{column, container};
use iced::Element;

use crate::Message;

pub fn stock_page<'a>() -> Element<'a, Message> {
    container(column![]).into()
}
