use iced::{
    widget::{container, horizontal_rule, text, vertical_rule, Column, Container, Row},
    Alignment, Background, Border, Color, Padding, Rotation, Shadow, Theme,
};
use palette::num::PartialCmp;

use crate::{
    gui::{
        morphiq::Morphiq,
        styles::{container::ContainerStyle, style_constant::Colors},
        types::message::Message,
        views::dashboard_view,
    },
    utils::icons::Icon,
};

pub fn employee_tracker<'a>(_morphiq: &'a Morphiq) -> Container<'a, Message> {
    container(tracker())
        .padding(Padding::from(10))
        .style(ContainerStyle::Rounded.appearance())
}

fn tracker<'a>() -> Container<'a, Message> {
    let board_2 = Column::new()
        .push(
            container(Icon::Users.to_text().size(42).align_x(Alignment::Start))
                .align_x(Alignment::Start)
                .style(tracker_container_style(&Colors::MayaBlue))
                .padding(Padding::from(10)),
        )
        .push(
            Row::new()
                .push(text(10).size(24))
                .push(
                    container(
                        Row::new()
                            .push(Icon::Chart.to_text().size(18).align_y(Alignment::Center))
                            .push(text("60%").size(18)),
                    )
                    .padding(Padding::from([2.5, 5.0]))
                    .style(tracker_container_style(&Colors::DarkPastelGreen)),
                )
                .spacing(25),
        )
        .push(text("Resign Employee").size(24).align_x(Alignment::Start))
        .spacing(5)
        .padding(Padding::from(10));

    let content_1 = Row::new()
        .push(board(Icon::User, "New Employee", 10, 25, &Colors::Xanthous))
        .push(vertical_rule(2.0))
        .push(board(
            Icon::Users,
            "Resign Employee",
            4,
            25,
            &Colors::MayaBlue,
        ));

    container(content_1)
}

fn board<'a>(
    icon: Icon,
    title: &'a str,
    partial: u64,
    total: u64,
    icon_color: &'a Colors,
) -> Container<'a, Message> {
    let percentage = (partial / total) * 100;
    let chart_percentage = if std::cmp::PartialOrd::lt(&percentage.clone(), &50) {
        Icon::TrendingDown
            .to_text()
            .size(18)
            .align_y(Alignment::Center)
    } else {
        Icon::TrendingUp
            .to_text()
            .size(18)
            .align_y(Alignment::Center)
    };
    let tracker_color = if std::cmp::PartialOrd::lt(&percentage.clone(), &50) {
        Colors::Red
    } else {
        Colors::DarkPastelGreen
    };

    let board = Column::new()
        .push(
            container(icon.to_text().size(42).align_x(Alignment::Start))
                .align_x(Alignment::Start)
                .style(tracker_container_style(icon_color))
                .padding(Padding::from(10)),
        )
        .push(
            Row::new()
                .push(text(total).size(24))
                .push(
                    container(
                        Row::new()
                            .push(chart_percentage)
                            .push(text(format!("{percentage}%")).size(18)),
                    )
                    .padding(Padding::from([2.5, 5.0]))
                    .style(tracker_container_style(&tracker_color)),
                )
                .spacing(25),
        )
        .push(text(title).size(24).align_x(Alignment::Start))
        .spacing(5)
        .padding(Padding::from(10));

    container(board)
}

fn tracker_container_style(color: &Colors) -> impl Fn(&Theme) -> container::Style {
    move |_t| container::Style {
        text_color: Some(color.get()),
        background: Some(Background::Color(Color {
            a: color.get().a - 0.70,
            ..color.get()
        })),
        border: Border {
            radius: 12.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}
