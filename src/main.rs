use iced::font::Family;
use iced::widget::{button, column, container, keyed_column, row, scrollable, text};
use iced::Alignment::Center;
use iced::Length::{Fill, FillPortion};
use iced::Theme;
use iced::{Element, Font};

mod widget;
use widget::{labeled_text_box, labeled_text_input, labeled_value};

pub fn main() -> iced::Result {
    iced::application("Sunminimart", State::update, State::view)
        .default_font(Font {
            family: Family::Name("TH Sarabun New"),
            weight: iced::font::Weight::Normal,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::Normal,
        })
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
}

#[derive(Debug, Clone)]
enum MessageStock {
    None,
}

#[derive(Debug, Clone)]
enum MessageMain {
    GotoSale,
    GotoStock,
}

fn thai_font() -> Font {
    Font {
        family: Family::Name("TH Sarabun New"),
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
            },
            (Pages::Stock, Message::Stock(message_stock)) => match message_stock {
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
            Pages::Main => {
                let sale_button = button(text("ขาย").font(thai_font()).center().size(40))
                    .width(300)
                    .on_press(Message::Main(MessageMain::GotoSale));
                let stock_button = button(text("คลังสินค้า").font(thai_font()).center().size(40))
                    .width(300)
                    .on_press(Message::Main(MessageMain::GotoStock));

                container(
                    container(
                        column![sale_button, stock_button]
                            .spacing(20)
                            .align_x(Center),
                    )
                    .style(|_| container::bordered_box(&Theme::Light))
                    .padding(50),
                )
                .center(Fill)
                .into()
            }

            Pages::Sale(sale) => {
                // Right
                let total_price =
                    labeled_value("รวม:".to_string(), "Total Price".to_string(), &sale.total);
                let current_price =
                    labeled_value("ราคา:".to_string(), "Price".to_string(), &sale.item.price);
                let received = labeled_text_input(
                    "รับเงิน / Received".to_string(),
                    &sale.received,
                    |input: String| Message::Sale(MessageSale::Receive(input)),
                    Message::Sale(MessageSale::Pay),
                );
                let change = ();
                let pay_button = container(
                    button(text("จ่ายเงิน / Pay").center().size(40).width(Fill))
                        .on_press(Message::Sale(MessageSale::Pay)),
                )
                .height(Fill)
                .align_y(Center);

                // Bottom
                let amount = labeled_text_input(
                    "จำนวน / Amount".to_string(),
                    sale.item.amount.clone(),
                    |input: String| Message::Sale(MessageSale::AmountChanged(input)),
                    Message::Sale(MessageSale::AmountSubmit),
                );
                let barcode = labeled_text_input(
                    "บาร์โค๊ด / Barcode".to_string(),
                    sale.item.barcode.clone(),
                    |input: String| Message::Sale(MessageSale::BarcodeChanged(input)),
                    Message::Sale(MessageSale::BarcodeSubmit),
                );
                let name = labeled_text_box("ชื่อสินค้า / Name".to_string(), &sale.item.name);
                let price = labeled_text_box("ราคา / Price".to_string(), sale.item.price);
                let sum = labeled_text_box("รวม / Sum".to_string(), sale.item.sum);

                // Grid
                let title = row![
                    text("order").width(Fill).center().size(25),
                    text("barcode").width(FillPortion(2)).center().size(25),
                    text("name").width(FillPortion(2)).center().size(25),
                    text("price").width(Fill).center().size(25),
                    text("amount").width(Fill).center().size(25),
                    text("sum").width(Fill).center().size(25),
                ];
                let list = keyed_column(sale.items.iter().enumerate().map(|x| {
                    (
                        x.0,
                        container(row![
                            text!("{}", x.0 + 1).width(Fill).center().size(25),
                            text!("{}", x.1.barcode)
                                .width(FillPortion(2))
                                .center()
                                .size(25),
                            text!("{}", x.1.name)
                                .width(FillPortion(2))
                                .center()
                                .size(25),
                            text!("{}", x.1.price).width(Fill).center().size(25),
                            text!("{}", x.1.amount).width(Fill).center().size(25),
                            text!("{}", x.1.sum).width(Fill).center().size(25),
                        ])
                        .style(|_| container::bordered_box(&Theme::Light))
                        .into(),
                    )
                }));

                // Sale view starts here
                container(column![
                    row![
                        // Grid
                        column![title, scrollable(list)]
                            .height(FillPortion(2))
                            .width(FillPortion(4)),
                        // Right panel
                        column![
                            total_price,
                            current_price,
                            if let Pages::Sale(sale) = &self.pages {
                                if !sale.paying {
                                    column![pay_button].height(Fill)
                                } else {
                                    column![text("1")]
                                }
                            } else {
                                column![text("2")]
                            }
                        ]
                        .width(Fill)
                    ]
                    .spacing(10)
                    .padding(10),
                    // Bottom Text Input
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
