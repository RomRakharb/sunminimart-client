use iced::widget::{button, column, container, keyed_column, row, scrollable, text};
use iced::Alignment::Center;
use iced::Element;
use iced::Length::{Fill, FillPortion};
use iced::Theme;

use crate::widget::{labeled_text_input, labeled_value, Position};
use crate::{Message, MessageSale, Pages, Sale, State};

impl State {
    pub fn sale_page<'a>(&self, sale: &Sale) -> Element<'a, Message> {
        // Right
        let total_price = labeled_value("รวม", 40, Position::Top, &sale.total);
        let current_price = labeled_value("ราคา", 40, Position::Top, &sale.item.price);
        let received = labeled_text_input(
            "received",
            "รับเงิน",
            40,
            Position::Top,
            &sale.received,
            |input: String| Message::Sale(MessageSale::Receive(input)),
            Message::Sale(MessageSale::Pay),
        );
        let change = labeled_value("เงินทอน", 40, Position::Top, &sale.change);
        let pay_button = container(
            button(text("จ่ายเงิน").center().size(40).width(Fill)).on_press(match sale.paying {
                false => Message::Sale(MessageSale::EnterPay),
                true => Message::Sale(MessageSale::Pay),
            }),
        )
        .height(Fill)
        .align_y(Center);

        // Bottom
        let amount = labeled_text_input(
            "amount",
            "จำนวน",
            25,
            Position::Top,
            sale.item.amount.clone(),
            |input: String| Message::Sale(MessageSale::AmountChanged(input)),
            Message::Sale(MessageSale::AmountSubmit),
        );
        let barcode = labeled_text_input(
            "barcode",
            "รหัสสินค้า",
            25,
            Position::Top,
            sale.item.barcode.clone(),
            |input: String| Message::Sale(MessageSale::BarcodeChanged(input)),
            Message::Sale(MessageSale::BarcodeSubmit),
        );
        let name = labeled_value("ชื่อสินค้า", 25, Position::Top, &sale.item.name);
        let price = labeled_value("ราคา", 25, Position::Top, &sale.item.price);
        let sum = labeled_value("รวม", 25, Position::Top, &sale.item.sum);

        // Grid
        let title = row![
            text("ลำดับ").width(Fill).center().size(20),
            text("รหัสสินค้า").width(FillPortion(2)).center().size(20),
            text("ชื่อสินค้า").width(FillPortion(2)).center().size(20),
            text("ราคาสินค้า").width(Fill).center().size(20),
            text("จำนวน").width(Fill).center().size(20),
            text("รวม").width(Fill).center().size(20),
        ];
        let list = keyed_column(sale.items.iter().enumerate().map(|x| {
            (
                x.0,
                container(row![
                    text!("{}", x.0 + 1).width(Fill).center().size(25),
                    text!("{}", x.1.barcode)
                        .width(FillPortion(2))
                        .center()
                        .size(25),
                    text!("{}", x.1.name)
                        .width(FillPortion(2))
                        .center()
                        .size(25),
                    text!("{}", x.1.price).width(Fill).center().size(25),
                    text!("{}", x.1.amount).width(Fill).center().size(25),
                    text!("{}", x.1.sum).width(Fill).center().size(25),
                ])
                .style(|_| container::bordered_box(&Theme::Light))
                .into(),
            )
        }));

        // Sale view starts here
        container(column![
            row![
                // Grid
                column![title, scrollable(list)]
                    .height(FillPortion(2))
                    .width(FillPortion(4)),
                // Right panel
                column![
                    total_price,
                    current_price,
                    if let Pages::Sale(sale) = &self.pages {
                        if !sale.paying {
                            column![pay_button].height(Fill)
                        } else {
                            column![received, change, pay_button].height(Fill)
                        }
                    } else {
                        column![text("2")]
                    }
                ]
                .width(Fill)
            ]
            .spacing(10)
            .padding(10),
            // Bottom Text Input
            row![amount, barcode, name, price, sum,]
                .spacing(10)
                .padding(10)
        ])
        .center(Fill)
        .into()
    }
}
