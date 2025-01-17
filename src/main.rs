use iced::widget::{column, container, keyed_column, row, scrollable, text};
use iced::Element;
use iced::Length::{Fill, FillPortion};
use iced::Theme;

mod widget;
use widget::{labeled_text_box, labeled_text_input, labeled_value};

pub fn main() -> iced::Result {
    iced::application("A cool counter", State::update, State::view).run()
}

#[derive(Default)]
enum Pages {
    #[default]
    Main,
    Sale {
        is_paying: bool,
        item: Item,
        items: Vec<Item>,
        total: u32,
    },
    Stock,
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
}

#[derive(Debug, Clone)]
enum Message {
    Main(MessageMain),
    Sale(MessageSale),
    Stock(MessageStock),
}

#[derive(Debug, Clone)]
enum MessageSale {
    BarcodeChanged(String),
    BarcodeSubmit,
    AmountChanged(String),
    AmountSubmit,
}

#[derive(Debug, Clone)]
enum MessageStock {
    None,
}

#[derive(Debug, Clone)]
enum MessageMain {
    None,
}

impl State {
    fn update(&mut self, message: Message) {
        match (&mut self.pages, message) {
            (Pages::Main, Message::Main(message_main)) => match message_main {
                _ => {}
            },
            (
                Pages::Sale {
                    is_paying,
                    item,
                    items,
                    total,
                },
                Message::Sale(message_sale),
            ) => match message_sale {
                MessageSale::BarcodeChanged(barcode) => {
                    item.barcode = barcode;
                }
                MessageSale::AmountChanged(amount) => {
                    item.amount = amount;
                }
                _ => {}
            },
            (Pages::Stock, Message::Stock(message_stock)) => match message_stock {
                _ => {}
            },
            _ => {}
        }
    }

    fn view(&self) -> Element<Message> {
        // View start
        match &self.pages {
            Pages::Main => column![].into(),
            Pages::Sale {
                is_paying,
                item,
                items,
                total,
            } => {
                // Right
                let total_price = labeled_value("Total Price:".to_string(), &total);
                let current_price = labeled_value("Current Price:".to_string(), &item.price);

                // Bottom
                let amount = labeled_text_input(
                    "Amount".to_string(),
                    item.amount.clone(),
                    |input: String| Message::Sale(MessageSale::AmountChanged(input)),
                    Message::Sale(MessageSale::AmountSubmit),
                );
                let barcode = labeled_text_input(
                    "Barcode".to_string(),
                    item.barcode.clone(),
                    |input: String| Message::Sale(MessageSale::BarcodeChanged(input)),
                    Message::Sale(MessageSale::BarcodeSubmit),
                );

                let name = labeled_text_box("Name:".to_string(), &item.name);
                let price = labeled_text_box("Price:".to_string(), &item.price);
                let sum = labeled_text_box("Sum:".to_string(), &item.sum);

                // Grid
                let title = row![
                    text("order").width(Fill).center().size(20),
                    text("barcode").width(FillPortion(2)).center().size(20),
                    text("name").width(FillPortion(2)).center().size(20),
                    text("price").width(Fill).center().size(20),
                    text("amount").width(Fill).center().size(20),
                    text("sum").width(Fill).center().size(20),
                ];
                let list = keyed_column(items.iter().enumerate().map(|x| {
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
                container(column![
                    row![
                        column![title, scrollable(list)]
                            .height(FillPortion(2))
                            .width(FillPortion(4)),
                        container(column![total_price, current_price].width(Fill)),
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
            Pages::Stock => container(column![]).into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn default_sale_state() -> State {
        State {
            pages: Pages::Sale {
                is_paying: false,
                item: Item::default(),
                items: Vec::new(),
                total: 0,
            },
        }
    }

    #[test]
    fn barcode_changed() {
        let mut state = default_sale_state();

        state.update(Message::Sale(MessageSale::BarcodeChanged(
            "11011".to_string(),
        )));

        if let Pages::Sale {
            is_paying: _,
            item,
            items: _,
            total: _,
        } = state.pages
        {
            assert_eq!(item.barcode, "11011".to_string());
        }
    }

    #[test]
    fn amount_changed() {
        let mut state = default_sale_state();

        state.update(Message::Sale(MessageSale::AmountChanged("10".to_string())));

        if let Pages::Sale {
            is_paying: _,
            item,
            items: _,
            total: _,
        } = state.pages
        {
            assert_eq!(item.amount, "10".to_string());
        }
    }

    #[test]
    fn barcode_submit() {
        let mut state = default_sale_state();

        state.update(Message::Sale(MessageSale::BarcodeChanged(
            "100".to_string(),
        )));
        state.update(Message::Sale(MessageSale::BarcodeSubmit));
        if let Pages::Sale {
            is_paying: _,
            item: _,
            items,
            total: _,
        } = state.pages
        {
            assert_eq!(items.len(), 1);
        }
    }

    #[test]
    fn barcode_submit_empty() {
        let mut state = default_sale_state();

        state.update(Message::Sale(MessageSale::BarcodeSubmit));

        if let Pages::Sale {
            is_paying: _,
            item,
            items,
            total: _,
        } = state.pages
        {
            assert!(items.is_empty());
            assert!(item.barcode.is_empty());
        }

        // let mut state = State::default();

        // // usual case
        // state.item.barcode = "100".to_string();
        // state.update(Message::BarcodeSubmit);
        // assert_eq!(state.items.len(), 1);

        // // barcode empty
        // state = State::default();
        // state.update(Message::BarcodeSubmit);
        // assert!(state.item.barcode.is_empty());
        // assert!(state.items.is_empty());

        // // amount not u32
        // state = State::default();
        // state.item.amount = "a".to_string();
        // state.item.barcode = "123".to_string();
        // state.update(Message::BarcodeSubmit);
        // assert_eq!(state.item.amount, "a".to_string());
        // assert_eq!(state.item.barcode, "123".to_string());
        // assert!(state.items.is_empty());
    }
}
