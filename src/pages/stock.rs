use iced::widget::{column, container, row, Space};
use iced::Alignment::Center;
use iced::Element;
use iced::Length::{Fill, FillPortion};
use iced::Theme;

use crate::widget::{labeled_text_input, Position};
use crate::{Message, MessageStock, State};

impl State {
    pub fn stock_page<'a>(&self) -> Element<'a, Message> {
        let barcode = labeled_text_input(
            "barcode",
            "รหัสสินค้า",
            25,
            Position::Left,
            self.setting.database_url.clone(),
            |input: String| Message::Stock(MessageStock::Back),
            Message::Stock(MessageStock::Back),
        );
        let name = labeled_text_input(
            "name",
            "ชื่อสินค้า",
            25,
            Position::Left,
            self.setting.database_url.clone(),
            |input: String| Message::Stock(MessageStock::Back),
            Message::Stock(MessageStock::Back),
        );

        container(row![
            Space::with_width(FillPortion(1)),
            container(
                column![row![barcode, name].spacing(25).align_y(Center)]
                    .align_x(Center)
                    .padding(50),
            )
            .style(|_| container::bordered_box(&Theme::Light))
            .width(FillPortion(2)),
            Space::with_width(FillPortion(1)),
        ])
        .center(Fill)
        .into()
    }
}
