use iced::widget::{column, container};
use iced::Element;

use crate::{Message, State};

impl State {
    pub fn stock_page<'a>(&self) -> Element<'a, Message> {
        container(column![]).into()
    }
}
