use iced::widget::{column, container, text, Button};
use iced::Alignment::Center;
use iced::Element;
use iced::Length::Fill;
use iced::Theme;

use crate::{Message, MessageMain, State};

fn button(display_text: &str) -> Button<'_, Message> {
    iced::widget::button(
        text(display_text)
            .shaping(text::Shaping::Advanced)
            .center()
            .size(40),
    )
    .width(300)
}

impl State {
    pub fn main_page<'a>(&self) -> Element<'a, Message> {
        let sale_button = button("หน้าต่างการขาย").on_press(Message::Main(MessageMain::Sale));
        let stock_button = button("คลังสินค้า").on_press(Message::Main(MessageMain::Stock));
        let setting_button = button("ตั้งค่า").on_press(Message::Main(MessageMain::Setting));

        container(
            container(
                column![sale_button, stock_button, setting_button]
                    .spacing(20)
                    .align_x(Center),
            )
            .style(|_| container::bordered_box(&Theme::Light))
            .padding(50),
        )
        .center(Fill)
        .into()
    }
}
