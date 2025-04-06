use iced::widget::{button, column, container, keyed_column, row, scrollable, Space};
use iced::Element;
use iced::Length::{Fill, FillPortion};
use iced::Theme;

use crate::custom_widget;
use crate::{Message, MessageSale, Pages, Sale, State};

impl State {
    pub fn sale_page<'a>(&self, sale: &'a Sale) -> Element<'a, Message> {
        // Right
        let total_price = column![
            custom_widget::text("รวม", 40),
            custom_widget::boxed_text(format!("{}", &sale.total), 40)
        ];
        let current_price = column![
            custom_widget::text("ราคา", 40),
            custom_widget::boxed_text(sale.item.price, 40)
        ];
        let received = column![
            custom_widget::text("รับเงิน", 40),
            custom_widget::text_input("received", &sale.received, 40)
                .on_input(|input: String| Message::Sale(MessageSale::Receive(input)))
                .on_submit(Message::Sale(MessageSale::Pay)),
        ];
        let change = column![
            custom_widget::text("เงินทอน", 40),
            custom_widget::boxed_text(sale.change, 40)
        ];
        let pay_button = button(custom_widget::text("จ่ายเงิน", 40)).on_press(match sale.paying {
            false => Message::Sale(MessageSale::EnterPay),
            true => Message::Sale(MessageSale::Pay),
        });

        // Bottom
        let amount = column![
            custom_widget::text("จำนวน", 25),
            custom_widget::text_input("amount", &sale.item.amount, 25)
                .on_input(|input: String| Message::Sale(MessageSale::AmountChanged(input)))
                .on_submit(Message::Sale(MessageSale::AmountSubmit))
        ];
        let barcode = column![
            custom_widget::text("รหัสสินค้า", 25),
            custom_widget::text_input("barcode", &sale.item.barcode, 25)
                .on_input(|input: String| Message::Sale(MessageSale::BarcodeChanged(input)))
                .on_submit(Message::Sale(MessageSale::BarcodeSubmit)),
        ];

        // Grid
        let title = row![
            custom_widget::text("ลำดับ", 20).width(Fill),
            custom_widget::text("รหัสสินค้า", 20).width(FillPortion(2)),
            custom_widget::text("ชื่อ", 20).width(FillPortion(2)),
            custom_widget::text("ราคาสินค้า", 20).width(Fill),
            custom_widget::text("จำนวน", 20).width(Fill),
            custom_widget::text("รวม", 20).width(Fill),
        ];
        let list = keyed_column(sale.items.iter().enumerate().map(|x| {
            (
                x.0,
                container(row![
                    custom_widget::text(format!("{}", x.0 + 1), 25).width(Fill),
                    custom_widget::text(&x.1.barcode, 25).width(FillPortion(2)),
                    custom_widget::text(&x.1.name, 25).width(FillPortion(2)),
                    custom_widget::text(format!("{}", x.1.price), 25).width(Fill),
                    custom_widget::text(x.1.amount.to_string(), 25).width(Fill),
                    custom_widget::text(format!("{}", x.1.sum), 25).width(Fill),
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
                    if !sale.paying {
                        column![
                            Space::with_height(FillPortion(3)),
                            container(pay_button).height(Fill)
                        ]
                        .height(Fill)
                    } else {
                        column![
                            Space::with_height(Fill),
                            received.height(Fill),
                            change.height(Fill),
                            container(pay_button).height(Fill)
                        ]
                        .height(Fill)
                    }
                ]
                .width(Fill)
            ]
            .spacing(10)
            .padding(10),
            // Bottom Text Input
            row![
                Space::with_width(Fill),
                amount,
                barcode,
                Space::with_width(Fill)
            ]
            .spacing(10)
            .padding(10)
        ])
        .center(Fill)
        .into()
    }
}
