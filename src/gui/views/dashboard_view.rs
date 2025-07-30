use iced::{
    widget::{button, container, text, Column, Container},
    Alignment, Length,
};

use crate::gui::{components::header::header_view, morphiq::Morphiq, types::message::Message};

pub fn dashboard_view(morphiq: &Morphiq) -> Container<'_, Message> {
    let content = Column::new()
        .push(header_view(morphiq))
        .push(text("Dashboard view").size(42))
        .push(
            button(text("Goto login").size(16).align_x(Alignment::Center))
                .width(Length::Fixed(450.0))
                .on_press(Message::ChangeRunningPage(super::RunningView::Login)),
        );

    container(content).width(Length::Fill).height(Length::Fill)
}
