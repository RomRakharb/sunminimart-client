use iced::widget::{column, container, row, Row, Space};
use iced::Alignment::Center;
use iced::Length::{Fill, FillPortion};
use iced::Theme;
use iced::{Element, Pixels};

use crate::{custom_widget, MessageStock, Pages};
use crate::{Message, State};

#[derive(PartialEq, Debug)]
pub enum Stock {
    Stock(Vec<Item>),
    AddProduct(Item),
    Edit,
    Restock,
}

#[derive(PartialEq, Debug, Default)]
pub struct Item {
    pub barcode: String,
    name: String,
    cost: f32,
    price: u32,
    amount: u32,
    exp: u32,
}

impl State {
    pub fn stock_page<'a>(&self, stock: &Stock) -> Element<'a, Message> {
        let header = row![
            custom_widget::button("คลังสินค้า", 25).on_press(Message::Stock(MessageStock::Back)),
            custom_widget::button("เพิ่มสินค้า", 25)
                .on_press(Message::Stock(MessageStock::ToAddProduct)),
            custom_widget::button("แก้ไขสินค้า", 25).on_press(Message::Stock(MessageStock::Back)),
            custom_widget::button("นำเข้าสินค้า", 25).on_press(Message::Stock(MessageStock::Back)),
        ]
        .spacing(Pixels(5.0))
        .padding(5);

        let content: Row<'a, Message> = match stock {
            Stock::Stock(_) => row![],
            Stock::AddProduct(item) => {
                let barcode = row![
                    custom_widget::text("รหัสสินค้า", 25),
                    custom_widget::text_input("barcode", &item.barcode, 25).on_input(
                        |input: String| Message::Stock(MessageStock::BarcodeChanged(input))
                    )
                ];

                let name = row![
                    custom_widget::text("ชื่อสินค้า", 25),
                    custom_widget::text_input("name", &item.name, 25)
                ];

                let cost = row![
                    custom_widget::text("ราคาทุน", 25),
                    custom_widget::text_input("cost", &item.cost.to_string(), 25)
                ];

                row![
                    Space::with_width(FillPortion(1)),
                    container(
                        column![barcode, name, cost]
                            .spacing(25)
                            .align_x(Center)
                            .padding(50),
                    )
                    .style(|_| container::bordered_box(&Theme::Light))
                    .width(FillPortion(2)),
                    Space::with_width(FillPortion(1)),
                ]
            }
            Stock::Edit => row![],
            Stock::Restock => row![],
        };

        container(
            column![
                header.height(FillPortion(1)),
                content.height(FillPortion(9))
            ]
            .align_x(Center),
        )
        .center(Fill)
        .into()
    }
}
