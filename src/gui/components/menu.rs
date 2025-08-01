use iced::{
    widget::{
        button, container, horizontal_rule, svg::Handle, text, tooltip, vertical_space, Column,
        Container, Row, Svg,
    },
    Alignment, Length, Padding,
};

use crate::{
    gui::{
        components::types::button::button_style,
        morphiq::{Morphiq, SVG_EMBLEMLOGO_BYTES},
        types::message::Message,
    },
    utils::icons::Icon,
};

pub fn sidebar_menu(_morphiq: &Morphiq) -> Container<'static, Message> {
    let logo_menu = Row::new().push(
        Svg::new(Handle::from_memory(SVG_EMBLEMLOGO_BYTES))
            .height(Length::Fixed(45.0))
            .width(Length::Fill),
    );
    let content = Column::new()
        .push(logo_menu)
        .push(horizontal_rule(2.0))
        .push(btns_menu(_morphiq))
        .width(Length::Fill)
        .spacing(15.0)
        .padding(10.0);

    container(content).width(100.0)
}

fn btns_menu(_morphiq: &Morphiq) -> Container<'static, Message> {
    container(
        Column::new()
            .push(tooltip(
                button(
                    Icon::Home
                        .to_text()
                        .size(32)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center),
                )
                .on_press(Message::ChangeHomeView(
                    crate::gui::views::InsideView::Dashboard,
                ))
                .style(button_style(&super::types::button::ButtonType::Nothing)),
                container(text("Dashboard").size(18))
                    .padding(Padding::from([2.5, 5.0]))
                    .style(container::rounded_box)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
                tooltip::Position::Right,
            ))
            .push(tooltip(
                button(
                    Icon::Users
                        .to_text()
                        .size(32)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center),
                )
                .on_press(Message::ChangeHomeView(
                    crate::gui::views::InsideView::Employee,
                ))
                .style(button_style(&super::types::button::ButtonType::Nothing)),
                container(text("Employees").size(18))
                    .padding(Padding::from([2.5, 5.0]))
                    .style(container::rounded_box)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
                tooltip::Position::Right,
            ))
            .push(tooltip(
                button(
                    Icon::Calendar
                        .to_text()
                        .size(32)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center),
                )
                .on_press(Message::ChangeHomeView(
                    crate::gui::views::InsideView::Attendance,
                ))
                .style(button_style(&super::types::button::ButtonType::Nothing)),
                container(text("Attendance").size(18))
                    .padding(Padding::from([2.5, 5.0]))
                    .style(container::rounded_box)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
                tooltip::Position::Right,
            ))
            .push(tooltip(
                button(
                    Icon::Money
                        .to_text()
                        .size(32)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center),
                )
                .on_press(Message::ChangeHomeView(
                    crate::gui::views::InsideView::Payroll,
                ))
                .style(button_style(&super::types::button::ButtonType::Nothing)),
                container(text("Payroll").size(18))
                    .padding(Padding::from([2.5, 5.0]))
                    .style(container::rounded_box)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
                tooltip::Position::Right,
            ))
            .push(tooltip(
                button(
                    Icon::Logout
                        .to_text()
                        .size(32)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center),
                )
                .on_press(Message::ChangeHomeView(
                    crate::gui::views::InsideView::Leaves,
                ))
                .style(button_style(&super::types::button::ButtonType::Nothing)),
                container(text("Leaves").size(18))
                    .padding(Padding::from([2.5, 5.0]))
                    .style(container::rounded_box)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
                tooltip::Position::Right,
            ))
            .push(tooltip(
                button(
                    Icon::DocTextInv
                        .to_text()
                        .size(32)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center),
                )
                .on_press(Message::ChangeHomeView(
                    crate::gui::views::InsideView::Documents,
                ))
                .style(button_style(&super::types::button::ButtonType::Nothing)),
                container(text("Documents").size(18))
                    .padding(Padding::from([2.5, 5.0]))
                    .style(container::rounded_box)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
                tooltip::Position::Right,
            ))
            .push(vertical_space())
            .push(horizontal_rule(2.0))
            .push(tooltip(
                button(
                    Icon::CogAlt
                        .to_text()
                        .size(32)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center),
                )
                .on_press(Message::ChangeHomeView(
                    crate::gui::views::InsideView::Settings,
                ))
                .style(button_style(&super::types::button::ButtonType::Nothing)),
                container(text("Settings").size(18))
                    .padding(Padding::from([2.5, 5.0]))
                    .style(container::rounded_box)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
                tooltip::Position::Right,
            ))
            .push(tooltip(
                button(
                    Icon::Logout1
                        .to_text()
                        .size(32)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center),
                )
                .on_press(Message::Logout)
                .style(button_style(&super::types::button::ButtonType::Nothing)),
                container(text("Logout").size(18))
                    .padding(Padding::from([2.5, 5.0]))
                    .style(container::rounded_box)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
                tooltip::Position::Right,
            ))
            .width(Length::Fill)
            .spacing(10)
            .align_x(Alignment::Center),
    )
}
