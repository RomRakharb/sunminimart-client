use iced::widget::{
    button, column, container, keyed_column, row, scrollable, text, text_input, Column, Row, Text,
    TextInput,
};
use iced::Alignment::Center;
use iced::Element;
use iced::Length::{Fill, FillPortion, Shrink};

mod widget;
use widget::{labeled_text_input, labeled_value};

pub fn main() -> iced::Result {
    iced::run("A cool counter", State::update, State::view)
}

#[derive(Default, Clone, PartialEq)]
struct Item {
    barcode: String,
    name: String,
    price: u32,
    amount: u32,
    sum: u32,
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
    BarcodeSubmit,
    AmountSubmit,
    BarcodeChanged(String),
    AmountChanged(u32),
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::BarcodeChanged(barcode) => {
                self.item.barcode = barcode;
            }
            Message::BarcodeSubmit => {
                if !self.item.barcode.is_empty() {
                    self.item = Item {
                        barcode: self.item.barcode.clone(),
                        name: "name".to_string(),
                        price: 100,
                        amount: 1,
                        sum: 100,
                    };
                    self.total_price += self.item.sum;
                    self.items.push(self.item.clone());
                    self.item.barcode = String::new()
                }
            }
            Message::AmountChanged(amount) => {
                self.item.amount = amount;
            }
            Message::AmountSubmit => {}
        }
    }

    fn view(&self) -> Element<Message> {
        // Right
        let total_price = labeled_value("Total Price:".to_string(), &self.total_price);
        let current_price = labeled_value("Current Price:".to_string(), &self.item.price);

        // let received = column![
        //     text("Received:"),
        //     text_input("", &format!("{}", self.received)).align_x(Center)
        // ]
        // .align_x(Center);
        // let change = column![
        //     text("Change:"),
        //     text_input("", &format!("{}", self.change)).align_x(Center)
        // ]
        // .align_x(Center);

        // Bottom
        let amount = labeled_text_input(
            "Amount".to_string(),
            self.item.amount,
            |input: String| Message::AmountChanged(input.parse().unwrap_or(1)),
            Message::AmountSubmit,
        );

        let barcode = labeled_text_input(
            "Barcode".to_string(),
            self.item.barcode.clone(),
            |input: String| Message::BarcodeChanged(input),
            Message::BarcodeSubmit,
        );

        let name = labeled_value("Name:".to_string(), &self.item.name);
        let price = labeled_value("Price:".to_string(), &self.item.price);
        let sum = labeled_value("Sum:".to_string(), &self.item.sum);

        let list = keyed_column(self.items.iter().enumerate().map(|x| {
            (
                x.0,
                row![
                    text!("{}", x.0 + 1).width(Fill),
                    text!("{}", x.1.barcode).width(FillPortion(2)),
                    text!("{}", x.1.name).width(Fill),
                    text!("{}", x.1.price).width(Fill),
                    text!("{}", x.1.amount).width(Fill),
                    text!("{}", x.1.sum).width(Fill),
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
                column![total_price, current_price].width(Fill)
            ]
            .spacing(10)
            .padding(10),
            row![amount, barcode, name, price, sum,]
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
