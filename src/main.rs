use iced::keyboard::key::Named;
use iced::keyboard::Key;
use iced::widget::text_input::focus;
use iced::{keyboard, Subscription};
use iced::{Element, Task};

mod custom_widget;
mod pages {
    pub mod main;
    pub mod sale;
    pub mod setting;
    pub mod stock;
}
mod api;
mod setting;

use crate::custom_widget::thai_font;
use crate::setting::Setting;

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
        State {
            pages: Pages::default(),
            setting: Setting::get_setting().unwrap_or_default(),
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
    ExitPay,
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

impl State {
    fn update(&mut self, message: Message) -> Task<Message> {
        let mut task = Task::none();
        match (&mut self.pages, message) {
            (Pages::Main, Message::Main(message_main)) => match message_main {
                MessageMain::Sale => {
                    self.pages = Pages::Sale(Sale::default());
                    task = focus("barcode");
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
                            todo!()
                        }
                        sale.total += sale.item.sum;
                        sale.item.barcode = "".to_string();
                        sale.item.amount = "1".to_string();
                    }
                }
                MessageSale::AmountSubmit => {}
                MessageSale::EnterPay => {
                    sale.paying = true;
                    task = focus("received");
                }
                MessageSale::ExitPay => {
                    sale.paying = false;
                    sale.received = "0".to_string();
                    sale.change = -(sale.total as i32);
                    task = focus("barcode");
                }
                MessageSale::Receive(received) => {
                    sale.received = received;
                    if let Ok(received) = sale.received.parse::<u32>() {
                        sale.change = received as i32 - sale.total as i32;
                    }
                }
                MessageSale::Pay => {
                    self.pages = Pages::Sale(Sale::default());
                    task = focus("barcode");
                }
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
        task
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
        match &self.pages {
            Pages::Main => keyboard::on_key_release(|_, _| None),
            Pages::Sale(sale) => match !sale.paying {
                true => keyboard::on_key_release(|key, _| match key {
                    Key::Named(Named::Escape) => Some(Message::Sale(MessageSale::Back)),
                    Key::Named(Named::F12) => Some(Message::Sale(MessageSale::EnterPay)),
                    _ => None,
                }),
                false => keyboard::on_key_release(|key, _| match key {
                    Key::Named(Named::Escape) => Some(Message::Sale(MessageSale::Back)),
                    Key::Named(Named::F12) => Some(Message::Sale(MessageSale::ExitPay)),
                    _ => None,
                }),
            },
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
