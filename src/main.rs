use iced::widget::{
    button, column, container, keyed_column, row, scrollable, text, text_input, Column, Row, Text,
    TextInput,
};
use iced::Alignment::Center;
use iced::Element;
use iced::Length::{Fill, FillPortion, Shrink};

mod widget;
use widget::LabeledTextBox;

pub fn main() -> iced::Result {
    iced::run("A cool counter", State::update, State::view)
}

#[derive(Default, Clone, PartialEq)]
struct Item {
    barcode: String,
    name: String,
    price: u32,
    amount: u32,
    total_price: u32,
}

#[derive(Default)]
struct State {
    item: Item,
    items: Vec<Item>,
    total_price: u32,

    received: u32,
    change: u32,
    content: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Submit,
    ContentChanged(String),
    BarcodeChanged(String),
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Submit => {
                self.item = Item {
                    barcode: self.item.barcode.clone(),
                    name: "name".to_string(),
                    price: 100,
                    amount: 1,
                    total_price: 100,
                };
                self.items.push(self.item.clone());
                self.item = Item::default();
            }
            Message::ContentChanged(content) => {}
            Message::BarcodeChanged(barcode) => {
                self.item.barcode = barcode;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // Right
        let total_price = column![
            text("Total Price"),
            text_input("", &format!("{}", self.total_price)).align_x(Center)
        ]
        .align_x(Center);
        let current_price = column![
            text("Current Price"),
            text_input("", &format!("{}", self.item.price)).align_x(Center)
        ]
        .align_x(Center);
        let received = column![
            text("Received:"),
            text_input("", &format!("{}", self.received)).align_x(Center)
        ]
        .align_x(Center);
        let change = column![
            text("Change:"),
            text_input("", &format!("{}", self.change)).align_x(Center)
        ]
        .align_x(Center);

        // Bottom
        let amount = column![
            text("Amount:"),
            text_input("", &format!("{}", self.item.amount)).align_x(Center)
        ]
        .align_x(Center);
        let barcode = column![
            text("Barcode:"),
            text_input("", &format!("{}", self.item.barcode))
                .on_input(|input: String| { Message::BarcodeChanged(input) })
                .on_submit(Message::Submit)
                .align_x(Center)
        ]
        .align_x(Center);
        let name = column![
            text("Name:"),
            text_input("", &format!("{}", self.item.name)).align_x(Center)
        ]
        .align_x(Center);
        let price = column![
            text("Price:"),
            text_input("", &format!("{}", self.item.price)).align_x(Center)
        ]
        .align_x(Center);
        let total_items_price = column![
            text("Total Price:"),
            text_input("", &format!("{}", self.item.total_price)).align_x(Center)
        ]
        .align_x(Center);

        let list = keyed_column(self.items.iter().enumerate().map(|x| {
            (
                x.0,
                row![
                    text!("{}", x.0 + 1).width(Fill),
                    text!("{}", x.1.barcode).width(FillPortion(2)),
                    text!("{}", x.1.name).width(Fill),
                    text!("{}", x.1.price).width(Fill),
                    text!("{}", x.1.amount).width(Fill),
                    text!("{}", x.1.total_price).width(Fill),
                ]
                .into(),
            )
        }));

        // View start
        container(column![
            row![
                scrollable(list)
                    .height(FillPortion(2))
                    .width(FillPortion(4)),
                column![total_price, current_price, received, change,].width(Fill)
            ]
            .spacing(10)
            .padding(10),
            row![amount, barcode, name, price, total_items_price,]
                .spacing(10)
                .padding(10)
        ])
        .center(Fill)
        .into()
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn it_counts_properly() {
//         let mut counter = State::default();

//         counter.update(Message::Increment);
//         counter.update(Message::Increment);
//         counter.update(Message::Decrement);

//         assert_eq!(counter.value, 1);
//     }
// }
