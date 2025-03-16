use std::fs::File;
use std::io::prelude::*;

use iced::keyboard::key::Named;
use iced::keyboard::Key;
use iced::Element;
use iced::{keyboard, Subscription};
use serde::{Deserialize, Serialize};
use serde_json;

mod widget;
mod pages {
    pub mod main;
    pub mod sale;
    pub mod setting;
    pub mod stock;
}
mod database;

use crate::widget::thai_font;

pub fn main() -> iced::Result {
    iced::application("Sunminimart", State::update, State::view)
        .default_font(thai_font())
        .subscription(State::subscription)
        .run()
}

#[derive(PartialEq, Debug)]
struct State {
    pages: Pages,
    setting: Setting,
}

impl Default for State {
    fn default() -> Self {
        let setting = if let Ok(setting) = Setting::get_setting() {
            setting
        } else {
            Setting::default()
        };
        State {
            pages: Pages::default(),
            setting,
        }
    }
}

#[derive(Default, PartialEq, Debug)]
enum Pages {
    #[default]
    Main,
    Sale(Sale),
    Stock,
    Setting,
}

#[derive(PartialEq, Default, Debug)]
struct Sale {
    paying: bool,
    item: Item,
    items: Vec<Item>,
    received: String,
    change: i32,
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

#[derive(Debug, Clone)]
enum Message {
    Main(MessageMain),
    Sale(MessageSale),
    Stock(MessageStock),
    Setting(MessageSetting),
}

#[derive(Debug, Clone)]
enum MessageMain {
    Sale,
    Stock,
    Setting,
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
}

#[derive(Debug, Clone)]
enum MessageSetting {
    DatabaseChanged(String),
    ToDatabaseSubmitButton,
    DatabaseSubmit,
    Back,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct Setting {
    database_url: String,
}

impl Setting {
    pub fn get_setting() -> std::io::Result<Self> {
        let mut file = File::open("setting.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let setting: Setting = serde_json::from_str(&contents)?;
        Ok(setting)
    }

    pub fn set_setting(&self) -> std::io::Result<()> {
        let setting = serde_json::to_string(self)?;
        let mut file = File::create("setting.json")?;
        file.write_all(setting.as_bytes())?;
        Ok(())
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match (&mut self.pages, message) {
            (Pages::Main, Message::Main(message_main)) => match message_main {
                MessageMain::Sale => {
                    self.pages = Pages::Sale(Sale::default());
                }
                MessageMain::Stock => {
                    self.pages = Pages::Stock;
                }
                MessageMain::Setting => {
                    self.pages = Pages::Setting;
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
                    if let (Ok(amount), Ok(price)) = (
                        sale.item.amount.parse::<u32>(),
                        sale.item.barcode.parse::<u32>(),
                    ) {
                        if amount > 0 && price > 0 && price <= 1000 {
                            sale.item.name = format!("สินค้าราคา {} บาท", price);
                            sale.item.price = price;
                            sale.item.sum = amount * price;
                            sale.items.push(sale.item.clone());
                        } else {
                        }
                        sale.total += sale.item.sum;
                        sale.item.barcode = "".to_string();
                        sale.item.amount = "1".to_string();
                    }
                }
                MessageSale::AmountSubmit => {}
                MessageSale::EnterPay => {
                    sale.paying = true;
                }
                MessageSale::Receive(received) => {
                    sale.received = received;
                    if let Ok(received) = sale.received.parse::<u32>() {
                        sale.change = received as i32 - sale.total as i32;
                    }
                }
                MessageSale::Pay => self.pages = Pages::Sale(Sale::default()),
                MessageSale::Back => self.pages = Pages::Main,
            },
            (Pages::Stock, Message::Stock(message_stock)) => match message_stock {
                MessageStock::Back => self.pages = Pages::Main,
            },
            (Pages::Setting, Message::Setting(message_setting)) => match message_setting {
                MessageSetting::DatabaseChanged(database) => self.setting.database_url = database,
                MessageSetting::ToDatabaseSubmitButton => {}
                MessageSetting::DatabaseSubmit => {
                    let _ = self.setting.set_setting();
                }
                MessageSetting::Back => self.pages = Pages::Main,
            },
            _ => {
                panic!();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // View start
        match &self.pages {
            Pages::Main => self.main_page(),
            Pages::Sale(sale) => self.sale_page(sale),
            Pages::Stock => self.stock_page(),
            Pages::Setting => self.setting_page(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.pages {
            Pages::Main => keyboard::on_key_release(|_, _| None),
            Pages::Sale(_) => keyboard::on_key_release(|key, _| match key {
                Key::Named(Named::Escape) => Some(Message::Sale(MessageSale::Back)),
                _ => None,
            }),
            Pages::Stock => keyboard::on_key_release(|key, _| match key {
                Key::Named(Named::Escape) => Some(Message::Stock(MessageStock::Back)),
                _ => None,
            }),
            Pages::Setting => keyboard::on_key_release(|key, _| match key {
                Key::Named(Named::Escape) => Some(Message::Setting(MessageSetting::Back)),
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
        state.update(Message::Main(MessageMain::Sale));

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
        state.update(Message::Main(MessageMain::Sale));

        state.update(Message::Sale(MessageSale::AmountChanged("10".to_string())));

        if let Pages::Sale(sale) = state.pages {
            assert_eq!(sale.item.amount, "10".to_string());
        }
    }

    #[test]
    fn barcode_submit() {
        let mut state = State::default();
        state.update(Message::Main(MessageMain::Sale));

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
        state.update(Message::Main(MessageMain::Sale));

        state.update(Message::Sale(MessageSale::BarcodeSubmit));

        if let Pages::Sale(sale) = state.pages {
            assert!(sale.item.barcode.is_empty());
            assert!(sale.items.is_empty());
        }
    }

    #[test]
    fn barcode_submit_amount_not_number() {
        let mut state = State::default();
        state.update(Message::Main(MessageMain::Sale));

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
        state.update(Message::Main(MessageMain::Sale));
        assert_eq!(state.pages, Pages::Sale(Sale::default()),);
    }

    #[test]
    fn goto_stock() {
        let mut state = State::default();
        state.update(Message::Main(MessageMain::Stock));
        assert_eq!(state.pages, Pages::Stock)
    }

    #[test]
    fn enter_pay() {
        let mut state = State::default();
        state.update(Message::Main(MessageMain::Sale));

        state.update(Message::Sale(MessageSale::EnterPay));
        if let Pages::Sale(sale) = state.pages {
            assert!(sale.paying);
        }
    }

    // #[test]
    // fn pay() {
    //     let mut state = State::default();
    //     state.update(Message::Main(MessageMain::Sale));

    //     state.update(Message::Sale(MessageSale::EnterPay));
    //     if let Pages::Sale(sale) = state.pages {
    //         assert!(!sale.paying);
    //     }
    // }
}
