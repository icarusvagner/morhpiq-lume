use iced::{
    widget::{
        button, container, scrollable, text, vertical_rule, Column, Container, Row, Scrollable,
    },
    Alignment, Element, Length,
};

use crate::gui::{
    components::{header::header_view, menu::sidebar_menu},
    morphiq::Morphiq,
    types::message::Message,
    views::{employees::employees_view, home::dashboard_view, InsideView},
};

pub struct MainLayout;

impl MainLayout {
    pub fn view<'a>(morphiq: &'a Morphiq, inside_view: &'a InsideView) -> Element<'a, Message> {
        let to_view_content: Element<Message> = Element::from(
            Scrollable::new(Self::to_view(morphiq, inside_view))
                .direction(scrollable::Direction::Vertical(
                    scrollable::Scrollbar::new()
                        .width(0.0)
                        .margin(0.0)
                        .scroller_width(4.0)
                        .anchor(scrollable::Anchor::Start),
                ))
                .width(Length::Fill)
                .height(Length::Fill),
        );
        let view_layout = Column::new()
            .push(header_view(morphiq))
            .push(to_view_content);

        let content = Row::new()
            .push(sidebar_menu(morphiq))
            .push(vertical_rule(2.0))
            .push(view_layout)
            .spacing(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn to_view<'a>(morphiq: &'a Morphiq, inside_view: &'a InsideView) -> Container<'a, Message> {
        match inside_view {
            InsideView::Dashboard => dashboard_view(morphiq),
            InsideView::Employee => employees_view(morphiq),
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
}
