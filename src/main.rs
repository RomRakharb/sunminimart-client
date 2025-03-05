use iced::font::Family;
use iced::keyboard::key::Named;
use iced::keyboard::Key;
use iced::{keyboard, Subscription};
use iced::{Element, Font};

mod widget;
mod pages {
    pub mod main;
    pub mod sale;
    pub mod stock;
}

use crate::pages::{main::main as main_page, sale::sale as sale_page, stock::stock as stock_page};

pub fn main() -> iced::Result {
    iced::application("Sunminimart", State::update, State::view)
        .default_font(thai_font())
        .subscription(State::subscription)
        .run()
}

#[derive(Default, PartialEq, Debug)]
enum Pages {
    #[default]
    Main,
    Sale(Sale),
    Stock,
}

#[derive(PartialEq, Default, Debug)]
struct Sale {
    paying: bool,
    item: Item,
    items: Vec<Item>,
    received: String,
    change: u32,
    total: u32,
}

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Default, PartialEq, Debug)]
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
    EnterPay,
    Receive(String),
    Pay,
    Back,
}

#[derive(Debug, Clone)]
enum MessageStock {
    Back,
    None,
}

#[derive(Debug, Clone)]
enum MessageMain {
    GotoSale,
    GotoStock,
}

fn thai_font() -> Font {
    Font {
        family: Family::Name("Sarabun"),
        weight: iced::font::Weight::Normal,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match (&mut self.pages, message) {
            (Pages::Main, Message::Main(message_main)) => match message_main {
                MessageMain::GotoSale => {
                    *self = State {
                        pages: Pages::Sale(Sale::default()),
                    };
                }
                MessageMain::GotoStock => {
                    *self = State {
                        pages: Pages::Stock,
                    }
                }
            },
            (Pages::Sale(sale), Message::Sale(message_sale)) => match message_sale {
                MessageSale::BarcodeChanged(barcode) => {
                    sale.item.barcode = barcode;
                }
                MessageSale::AmountChanged(amount) => {
                    sale.item.amount = amount;
                }
                MessageSale::BarcodeSubmit => {
                    if !sale.item.barcode.is_empty()
                        && sale.item.amount.parse::<u32>().is_ok_and(|x| x != 0)
                    {
                        sale.items.push(sale.item.clone());
                    }
                }
                MessageSale::AmountSubmit => {}
                MessageSale::EnterPay => {
                    sale.paying = true;
                }
                MessageSale::Receive(received) => {
                    sale.received = received;
                }
                MessageSale::Pay => {}
                MessageSale::Back => *self = State { pages: Pages::Main },
            },
            (Pages::Stock, Message::Stock(message_stock)) => match message_stock {
                MessageStock::Back => *self = State { pages: Pages::Main },
                _ => {}
            },
            _ => {
                // panic!();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // View start
        match &self.pages {
            Pages::Main => main_page(),
            Pages::Sale(sale) => sale_page(&self, &sale),
            Pages::Stock => stock_page(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.pages {
            Pages::Main => keyboard::on_key_release(|key, _| match key {
                _ => None,
            }),
            Pages::Sale(_) => keyboard::on_key_release(|key, _| match key {
                Key::Named(Named::Escape) => Some(Message::Sale(MessageSale::Back)),
                _ => None,
            }),
            Pages::Stock => keyboard::on_key_release(|key, _| match key {
                Key::Named(Named::Escape) => Some(Message::Stock(MessageStock::Back)),
                _ => None,
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn barcode_changed() {
        let mut state = State::default();
        state.update(Message::Main(MessageMain::GotoSale));

        state.update(Message::Sale(MessageSale::BarcodeChanged(
            "11011".to_string(),
        )));

        if let Pages::Sale(sale) = state.pages {
            assert_eq!(sale.item.barcode, "11011".to_string());
        }
    }

    #[test]
    fn amount_changed() {
        let mut state = State::default();
        state.update(Message::Main(MessageMain::GotoSale));

        state.update(Message::Sale(MessageSale::AmountChanged("10".to_string())));

        if let Pages::Sale(sale) = state.pages {
            assert_eq!(sale.item.amount, "10".to_string());
        }
    }

    #[test]
    fn barcode_submit() {
        let mut state = State::default();
        state.update(Message::Main(MessageMain::GotoSale));

        state.update(Message::Sale(MessageSale::BarcodeChanged(
            "100".to_string(),
        )));
        state.update(Message::Sale(MessageSale::BarcodeSubmit));
        if let Pages::Sale(sale) = state.pages {
            assert_eq!(sale.items.len(), 1);
        }
    }

    #[test]
    fn barcode_submit_empty() {
        let mut state = State::default();
        state.update(Message::Main(MessageMain::GotoSale));

        state.update(Message::Sale(MessageSale::BarcodeSubmit));

        if let Pages::Sale(sale) = state.pages {
            assert!(sale.item.barcode.is_empty());
            assert!(sale.items.is_empty());
        }
    }

    #[test]
    fn barcode_submit_amount_not_number() {
        let mut state = State::default();
        state.update(Message::Main(MessageMain::GotoSale));

        state.update(Message::Sale(MessageSale::BarcodeChanged(
            "100".to_string(),
        )));
        state.update(Message::Sale(MessageSale::AmountChanged("a".to_string())));
        state.update(Message::Sale(MessageSale::BarcodeSubmit));

        if let Pages::Sale(sale) = state.pages {
            assert!(sale.items.is_empty());
        }
    }

    #[test]
    fn goto_sale() {
        let mut state = State::default();
        state.update(Message::Main(MessageMain::GotoSale));
        assert_eq!(
            state,
            State {
                pages: Pages::Sale(Sale::default())
            }
        );
    }

    #[test]
    fn goto_stock() {
        let mut state = State::default();
        state.update(Message::Main(MessageMain::GotoStock));
        assert_eq!(
            state,
            State {
                pages: Pages::Stock
            }
        )
    }

    #[test]
    fn enter_pay() {
        let mut state = State::default();
        state.update(Message::Main(MessageMain::GotoSale));

        state.update(Message::Sale(MessageSale::EnterPay));
        if let Pages::Sale(sale) = state.pages {
            assert!(sale.paying);
        }
    }

    // #[test]
    // fn pay() {
    //     let mut state = State::default();
    //     state.update(Message::Main(MessageMain::GotoSale));

    //     state.update(Message::Sale(MessageSale::EnterPay));
    //     if let Pages::Sale(sale) = state.pages {
    //         assert!(!sale.paying);
    //     }
    // }
}
