use iced::widget::{column, container, row, Space};
use iced::Alignment::Center;
use iced::Element;
use iced::Length::{Fill, FillPortion};
use iced::Theme;

use crate::{custom_widget, Message, MessageMain, State};

impl State {
    pub fn main_page<'a>(&self) -> Element<'a, Message> {
        let sale_button =
            custom_widget::button("หน้าต่างการขาย", 40).on_press(Message::Main(MessageMain::Sale));
        let stock_button =
            custom_widget::button("คลังสินค้า", 40).on_press(Message::Main(MessageMain::Stock));
        let setting_button =
            custom_widget::button("ตั้งค่า", 40).on_press(Message::Main(MessageMain::Setting));

        container(row![
            Space::with_width(FillPortion(1)),
            container(row![
                Space::with_width(FillPortion(1)),
                column![sale_button, stock_button, setting_button]
                    .spacing(20)
                    .align_x(Center)
                    .width(FillPortion(3)),
                Space::with_width(FillPortion(1)),
            ])
            .style(|_| container::bordered_box(&Theme::Light))
            .padding(50)
            .width(FillPortion(3)),
            Space::with_width(FillPortion(1)),
        ])
        .center(Fill)
        .into()
    }
}
