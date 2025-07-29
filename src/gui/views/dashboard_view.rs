use iced::{
    widget::{button, container, text, Column, Container},
    Alignment, Length,
};

use crate::gui::{morphiq::Morphiq, types::message::Message};

pub fn dashboard_view(_morphiq: &Morphiq) -> Container<'_, Message> {
    let content = Column::new()
        .push(text("Dashboard view").size(42))
        .push(
            button(text("Goto login").size(16).align_x(Alignment::Center))
                .width(Length::Fixed(450.0))
                .on_press(Message::ChangeRunningPage(super::RunningView::Login)),
        )
        .align_x(Alignment::Center);

    container(content)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
}
