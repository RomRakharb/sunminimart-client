use iced::widget::{column, container, keyed_column, row, scrollable, text};
use iced::Element;
use iced::Length::{Fill, FillPortion};
use iced::Theme;

mod widget;
use widget::{labeled_text_input, labeled_value};

pub fn main() -> iced::Result {
    iced::application("A cool counter", State::update, State::view).run()
}

enum Pages {
    Main,
    Sale(SaleStatus),
    Stock,
}

impl Default for Pages {
    fn default() -> Self {
        Pages::Sale(SaleStatus::Buy)
    }
}

#[derive(Default)]
enum SaleStatus {
    #[default]
    Buy,
    Pay,
}

#[derive(Clone, PartialEq)]
struct Item {
    barcode: String,
    name: String,
    price: u32,
    amount: String,
    sum: u32,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            barcode: "".to_string(),
            name: "".to_string(),
            price: 0,
            amount: "1".to_string(),
            sum: 0,
        }
    }
}

#[derive(Default)]
struct State {
    pages: Pages,
    item: Item,
    items: Vec<Item>,
    total_price: u32,
}

#[derive(Debug, Clone)]
enum Message {
    BarcodeSubmit,
    AmountSubmit,
    BarcodeChanged(String),
    AmountChanged(String),
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::BarcodeChanged(barcode) => {
                self.item.barcode = barcode;
            }
            Message::BarcodeSubmit => {
                if !self.item.barcode.is_empty()
                    && self.item.amount.trim().parse::<u32>().is_ok_and(|x| x != 0)
                {
                    self.item = Item {
                        barcode: self.item.barcode.clone(),
                        name: "name".to_string(),
                        price: 100,
                        amount: "1".to_string(),
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

        // Bottom
        let amount = labeled_text_input(
            "Amount".to_string(),
            self.item.amount.clone(),
            |input: String| Message::AmountChanged(input),
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

        // Grid
        let title = row![
            text("order").width(Fill).center().size(20),
            text("barcode").width(FillPortion(2)).center().size(20),
            text("name").width(FillPortion(2)).center().size(20),
            text("price").width(Fill).center().size(20),
            text("amount").width(Fill).center().size(20),
            text("sum").width(Fill).center().size(20),
        ];
        let list = keyed_column(self.items.iter().enumerate().map(|x| {
            (
                x.0,
                container(row![
                    text!("{}", x.0 + 1).width(Fill).center().size(20),
                    text!("{}", x.1.barcode)
                        .width(FillPortion(2))
                        .center()
                        .size(20),
                    text!("{}", x.1.name)
                        .width(FillPortion(2))
                        .center()
                        .size(20),
                    text!("{}", x.1.price).width(Fill).center().size(20),
                    text!("{}", x.1.amount).width(Fill).center().size(20),
                    text!("{}", x.1.sum).width(Fill).center().size(20),
                ])
                .style(|_| container::bordered_box(&Theme::Light))
                .into(),
            )
        }));

        // View start
        match &self.pages {
            Pages::Main => column![].into(),
            Pages::Sale(status) => container(column![
                row![
                    column![title, scrollable(list)]
                        .height(FillPortion(2))
                        .width(FillPortion(4)),
                    container(column![total_price, current_price].width(Fill)),
                    {
                        match status {
                            SaleStatus::Pay => {
                                column![]
                            }
                            SaleStatus::Buy => {
                                column![]
                            }
                        }
                    }
                ]
                .spacing(10)
                .padding(10),
                row![amount, barcode, name, price, sum,]
                    .spacing(10)
                    .padding(10)
            ])
            .center(Fill)
            .into(),
            Pages::Stock => container(column![]).into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn barcode_changed() {
        let mut state = State::default();

        state.update(Message::BarcodeChanged("11011".to_string()));

        assert_eq!(state.item.barcode, "11011".to_string());
    }

    #[test]
    fn amount_changed() {
        let mut state = State::default();

        state.update(Message::AmountChanged("12".to_string()));

        assert_eq!(state.item.amount, "12".to_string());
    }

    #[test]
    fn barcode_submit() {
        let mut state = State::default();

        // usual case
        state.item.barcode = "100".to_string();
        state.update(Message::BarcodeSubmit);
        assert_eq!(state.items.len(), 1);

        // barcode empty
        state = State::default();
        state.update(Message::BarcodeSubmit);
        assert!(state.item.barcode.is_empty());
        assert!(state.items.is_empty());

        // amount not u32
        state = State::default();
        state.item.amount = "a".to_string();
        state.item.barcode = "123".to_string();
        state.update(Message::BarcodeSubmit);
        assert_eq!(state.item.amount, "a".to_string());
        assert_eq!(state.item.barcode, "123".to_string());
        assert!(state.items.is_empty());
    }
}
