use iced::{
    widget::{button, container, text, Column, Container, Row},
    Alignment, Length,
};

use crate::gui::{
    components::{header::header_view, menu::sidebar_menu},
    morphiq::Morphiq,
    types::message::Message,
};

pub fn dashboard_view(morphiq: &Morphiq) -> Container<'_, Message> {
    let content = Row::new()
        .push(sidebar_menu())
        .push(
            Column::new()
                .push(header_view(morphiq))
                .push(text("Dashboard view").size(42))
                .push(
                    button(text("Goto login").size(16).align_x(Alignment::Center))
                        .width(Length::Fixed(450.0))
                        .on_press(Message::ChangeRunningPage(super::RunningView::Login)),
                ),
        )
        .spacing(10);

    container(content).width(Length::Fill).height(Length::Fill)
}
