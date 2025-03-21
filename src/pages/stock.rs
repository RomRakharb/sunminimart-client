use iced::widget::{button, column, container, row, Row, Space};
use iced::Alignment::Center;
use iced::Length::{Fill, FillPortion};
use iced::Theme;
use iced::{Element, Pixels};

use crate::custom_widget;
use crate::{Message, MessageStock, State};

fn header<'a>() -> Row<'a, Message> {
    row![
        button("ff").height(Fill),
        button("ff").height(Fill),
        button("ff").height(Fill),
        button("ff").height(Fill),
    ]
    .spacing(Pixels(5.0))
}

impl State {
    pub fn stock_page<'a>(&self) -> Element<'a, Message> {
        let barcode = row![
            custom_widget::text("รหัสสินค้า", 25),
            custom_widget::text_input("barcode", &self.setting.database_url, 25)
        ];
        let name = row![
            custom_widget::text("ชื่อสินค้า", 25),
            custom_widget::text_input("barcode", &self.setting.database_url, 25)
        ];

        container(column![
            header().height(FillPortion(1)),
            row![
                Space::with_width(FillPortion(1)),
                container(
                    column![barcode, name]
                        .spacing(25)
                        .align_x(Center)
                        .padding(50),
                )
                .style(|_| container::bordered_box(&Theme::Light))
                .width(FillPortion(2)),
                Space::with_width(FillPortion(1)),
            ]
            .height(FillPortion(9))
        ])
        .center(Fill)
        .into()
    }
}
