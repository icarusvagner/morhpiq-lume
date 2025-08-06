use iced::{
    widget::{button, container, text, vertical_rule, Column, Container, Row},
    Alignment, Length,
};

use crate::gui::{
    components::{header::header_view, menu::sidebar_menu},
    morphiq::Morphiq,
    types::message::Message,
    views::{home::dashboard_view, InsideView},
};

pub struct MainLayout<'a> {
    morphiq: Option<&'a Morphiq>,
    inside_view: Option<&'a InsideView>,
}

pub fn main_layout<'a>(
    morphiq: &'a Morphiq,
    inside_view: &'a InsideView,
) -> Container<'a, Message> {
    let view_layout = Column::new()
        .push(header_view(morphiq))
        .push(to_view(morphiq, inside_view));

    let content = Row::new()
        .push(sidebar_menu(morphiq))
        .push(vertical_rule(2.0))
        .push(view_layout)
        .spacing(10);

    container(content).width(Length::Fill).height(Length::Fill)
}

pub fn to_view<'a>(morphiq: &'a Morphiq, inside_view: &'a InsideView) -> Container<'a, Message> {
    match inside_view {
        InsideView::Dashboard => dashboard_view(morphiq),
        _ => container(
            Column::new()
                .push(text("Dashboard view").size(42))
                .push(
                    button(text("Goto login").size(16).align_x(Alignment::Center))
                        .width(Length::Fixed(450.0))
                        .on_press(Message::ChangeRunningPage(super::RunningView::Login)),
                )
                .width(Length::Fill),
        ),
    }
}
