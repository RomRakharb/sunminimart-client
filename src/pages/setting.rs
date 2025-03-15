use iced::{
    widget::{column, container, row},
    Element,
};

use crate::Message;

pub fn setting_page<'a>() -> Element<'a, Message> {
    container(column![row![]]).into()
}
