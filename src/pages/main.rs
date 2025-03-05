use iced::widget::{button, column, container, text};
use iced::Alignment::Center;
use iced::Element;
use iced::Length::Fill;
use iced::Theme;

use crate::{thai_font, Message, MessageMain};

pub fn main() -> Element<'static, Message> {
    let sale_button = button(text("ขาย").font(thai_font()).center().size(40))
        .width(300)
        .on_press(Message::Main(MessageMain::GotoSale));
    let stock_button = button(text("คลังสินค้า").font(thai_font()).center().size(40))
        .width(300)
        .on_press(Message::Main(MessageMain::GotoStock));

    container(
        container(
            column![sale_button, stock_button]
                .spacing(20)
                .align_x(Center),
        )
        .style(|_| container::bordered_box(&Theme::Light))
        .padding(50),
    )
    .center(Fill)
    .into()
}
