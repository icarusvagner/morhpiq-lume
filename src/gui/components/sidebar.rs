use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        button, container, horizontal_rule, svg::Handle, text, tooltip, vertical_space, Column,
        Row, Svg,
    },
    Alignment, Element, Length, Padding,
};

use crate::{
    gui::{
        morphiq::SVG_EMBLEMLOGO_BYTES,
        styles::{
            button::ButtonType, container::ContainerType, rule::RuleType, text::TextType,
            types::style_type::StyleType,
        },
        types::message::Message,
        views::{mainview::MainViews, types::running_view::RunningView},
    },
    utils::types::icon::Icon,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct SidebarMenu;

impl SidebarMenu {
    pub const ALL: [(&str, Icon, Message); 6] = [
        (
            "Dashboard",
            Icon::House,
            Message::ChangeRunningView(RunningView::MainView(MainViews::Dashboard)),
        ),
        (
            "Employees",
            Icon::UsersRound,
            Message::ChangeRunningView(RunningView::MainView(MainViews::Employee)),
        ),
        (
            "Attendance",
            Icon::CalendarDays,
            Message::ChangeRunningView(RunningView::MainView(MainViews::Attendance)),
        ),
        (
            "Payroll",
            Icon::PhPeso,
            Message::ChangeRunningView(RunningView::MainView(MainViews::Payroll)),
        ),
        (
            "Leaves",
            Icon::ArrowBendDoubleUpRight,
            Message::ChangeRunningView(RunningView::MainView(MainViews::Leaves)),
        ),
        (
            "Documents",
            Icon::Newspaper,
            Message::ChangeRunningView(RunningView::MainView(MainViews::Documents)),
        ),
    ];

    pub fn view<'a>() -> Element<'a, Message, StyleType> {
        let tooltip_btns_menu = Column::with_children(Self::ALL.map(|v| {
            tooltip(
                button(
                    v.1.to_text()
                        .size(32)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .class(TextType::Base100),
                )
                .class(ButtonType::Ghost)
                .on_press(v.2),
                container(text(v.0).size(18).class(TextType::Base100))
                    .padding(Padding::from([2.5, 5.0]))
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .class(ContainerType::Ghost),
                tooltip::Position::Right,
            )
            .class(ContainerType::Neutral)
            .into()
        }));

        let content = Column::new()
            .push(tooltip_btns_menu)
            .push(vertical_space())
            .push(horizontal_rule(2.0).class(RuleType::Base200))
            .push(
                tooltip(
                    button(
                        Icon::Power
                            .to_text()
                            .size(32)
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                            .class(TextType::Base100),
                    )
                    .on_press(Message::Quit)
                    .class(ButtonType::Ghost),
                    container(text("Quit").size(18).class(TextType::Base100))
                        .padding(Padding::from([2.5, 5.0]))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .class(ContainerType::Ghost),
                    tooltip::Position::Right,
                )
                .class(ContainerType::Neutral),
            )
            .spacing(15)
            .width(Length::Fill)
            .padding(15)
            .align_x(Horizontal::Center);

        container(content)
            .width(80.0)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Start)
            .class(ContainerType::InfoNoBorder)
            .into()
    }
}
