use iced::widget::{button, column, container, row, text, text_input, Row};
use iced::Alignment::Center;
use iced::Element;
use iced::Length::{Fill, FillPortion};

pub fn main() -> iced::Result {
    iced::run("A cool counter", State::update, State::view)
}

#[derive(Default, Clone)]
struct Item {
    barcode: String,
    name: String,
    price: u32,
    amount: u32,
    sale_type: SaleType,
    total_price: u32,
}

#[derive(Default, Clone)]
enum SaleType {
    #[default]
    Retail,
    Wholesale,
}

#[derive(Default)]
struct State {
    item: Item,
    item_list: Vec<Item>,
    total_price: u32,
    content: String,
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
                self.content = format!("{}!!", self.content);
            }
            Message::ContentChanged(content) => {
                self.content = content;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(column![
            row![
                text(&self.content).width(FillPortion(2)),
                column![
                    text(&self.content),
                    text(&self.content),
                    text(&self.content)
                ]
                .width(Fill)
                .clip(true)
            ]
            .spacing(10),
            row![
                text_input("", &self.content)
                    .on_input(Message::ContentChanged)
                    .on_submit(Message::Submit),
                text_input("", &self.content)
                    .on_input(Message::ContentChanged)
                    .on_submit(Message::Submit),
                text_input("", &self.content)
                    .on_input(Message::ContentChanged)
                    .on_submit(Message::Submit),
                text_input("", &self.content)
                    .on_input(Message::ContentChanged)
                    .on_submit(Message::Submit),
            ]
            .spacing(10)
            .padding(10)
        ])
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
