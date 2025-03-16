use iced::{
    widget::{button, column, container, row, text, Space},
    Alignment::Center,
    Element,
    Length::{Fill, FillPortion},
    Pixels, Theme,
};

use crate::State;
use crate::{
    widget::{labeled_text_input, Position},
    Message, MessageSetting,
};

impl State {
    pub fn setting_page<'a>(&self) -> Element<'a, Message> {
        let database_url = labeled_text_input(
            "database url",
            25,
            Position::Left,
            self.setting.database_url.clone(),
            |input: String| Message::Setting(MessageSetting::DatabaseChanged(input)),
            Message::Setting(MessageSetting::ToDatabaseSubmitButton),
        );

        let database_button = button(text("เชื่อมต่อ / connect").size(Pixels(25.0)))
            .on_press(Message::Setting(MessageSetting::DatabaseSubmit));

        container(row![
            Space::with_width(FillPortion(1)),
            container(
                column![row![database_url, database_button]
                    .spacing(25)
                    .align_y(Center)]
                .align_x(Center)
                .padding(50),
            )
            .style(|_| container::bordered_box(&Theme::Light))
            .width(FillPortion(3)),
            Space::with_width(FillPortion(1)),
        ])
        .center(Fill)
        .into()
    }
}
