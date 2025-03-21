use iced::{
    widget::{button, column, container, row, Space},
    Alignment::Center,
    Element,
    Length::{Fill, FillPortion},
    Theme,
};

use crate::State;
use crate::{custom_widget, Message, MessageSetting};

impl State {
    pub fn setting_page<'a>(&self) -> Element<'a, Message> {
        let database_url = row![
            custom_widget::text("ฐานข้อมูล", 25),
            custom_widget::text_input("database_url", &self.setting.database_url, 25)
                .on_input(|input: String| Message::Setting(MessageSetting::DatabaseChanged(input)))
                .on_submit(Message::Setting(MessageSetting::ToDatabaseSubmitButton))
        ];

        let database_button = button(custom_widget::text("เชื่อมต่อ", 25))
            .on_press(Message::Setting(MessageSetting::DatabaseSubmit));

        container(row![
            Space::with_width(FillPortion(1)),
            container(
                column![row![
                    database_url.width(FillPortion(4)),
                    database_button.width(FillPortion(1))
                ]
                .spacing(25)
                .align_y(Center)]
                .align_x(Center)
                .padding(50),
            )
            .style(|_| container::bordered_box(&Theme::Light))
            .width(FillPortion(2)),
            Space::with_width(FillPortion(1)),
        ])
        .center(Fill)
        .into()
    }
}
