use iced::widget::{
    button, column, container, row, text, text_input, Column, Row, Text, TextInput,
};
use iced::Alignment::Center;
use iced::Element;
use iced::Length::{Fill, FillPortion};

mod widget;
use widget::LabeledTextBox;

pub fn main() -> iced::Result {
    iced::run("A cool counter", State::update, State::view)
}

#[derive(Default, Clone)]
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
    item_list: Vec<Item>,
    total_price: u32,

    received: u32,
    change: u32,
    content: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Submit,
    ContentChanged(String),
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Submit => {
                // self.content = self.content;
            }
            Message::ContentChanged(content) => {
                // self.content = content.parse().expect("parse");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // Right
        let total_price = column![
            text("Total Price"),
            text_input("", &format!("{}", self.total_price)).align_x(Center)
        ];
        let current_price = column![
            text("Current Price"),
            text_input("", &format!("{}", self.item.price)).align_x(Center)
        ];
        let received = column![
            text("Received:"),
            text_input("", &format!("{}", self.received)).align_x(Center)
        ];
        let change = column![
            text("Change:"),
            text_input("", &format!("{}", self.change)).align_x(Center)
        ];

        // Bottom
        let amount = column![
            text("Amount:"),
            text_input("", &format!("{}", self.item.amount)).align_x(Center)
        ];
        let barcode = column![
            text("Barcode:"),
            text_input("", &format!("{}", self.item.barcode)).align_x(Center)
        ];
        let name = column![
            text("Name:"),
            text_input("", &format!("{}", self.item.name)).align_x(Center)
        ];
        let price = column![
            text("Price:"),
            text_input("", &format!("{}", self.item.price)).align_x(Center)
        ];
        let total_items_price = column![
            text("Total Price:"),
            text_input("", &format!("{}", self.item.total_price)).align_x(Center)
        ];

        // View start
        container(column![
            row![
                text(&self.content).width(FillPortion(3)),
                column![total_price, current_price, received, change,]
                    .width(Fill)
                    .clip(true)
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
