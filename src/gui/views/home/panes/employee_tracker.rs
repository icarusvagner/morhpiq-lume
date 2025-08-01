use iced::{
    widget::{button, container, horizontal_rule, text, vertical_rule, Column, Container, Row},
    Alignment, Background, Border, Color, Length, Padding, Shadow, Theme,
};

use crate::{
    gui::{
        components::types::button::{button_style, ButtonType},
        morphiq::Morphiq,
        styles::{container::ContainerStyle, style_constant::Colors},
        types::message::Message,
    },
    utils::icons::Icon,
};

pub fn employee_tracker<'a>(_morphiq: &'a Morphiq) -> Container<'a, Message> {
    let content = Row::new()
        .push(tracker())
        .push(vertical_rule(2.0))
        .push(bar_chart_tracker())
        .spacing(15);

    container(content)
        .height(Length::Fixed(400.0))
        .style(ContainerStyle::Rounded.appearance())
}

fn bar_chart_tracker<'a>() -> Container<'a, Message> {
    let header = Row::new().push(text("Employee's Tracker").size(24)).push(
        container(
            button(
                Row::new()
                    .push(text("This Week").size(18))
                    .push(Icon::DownOpen.to_text().size(18))
                    .spacing(15)
                    .align_y(Alignment::Center),
            )
            .style(button_style(&ButtonType::Nothing)),
        )
        .style(ContainerStyle::Rounded.appearance()),
    );

    container(header).padding(Padding::from(15))
}

fn tracker<'a>() -> Container<'a, Message> {
    let content_1 = Row::new()
        .push(board(
            Icon::User,
            "New Employee",
            10.0,
            25.0,
            &Colors::Xanthous,
        ))
        .push(vertical_rule(2.0))
        .push(board(
            Icon::Users,
            "Resign Employee",
            4.0,
            25.0,
            &Colors::MayaBlue,
        ))
        .align_y(Alignment::Center);

    let content_2 = Row::new()
        .push(board(
            Icon::Calendar,
            "Employees",
            24.0,
            26.0,
            &Colors::SkyBlue,
        ))
        .push(vertical_rule(2.0))
        .push(board(
            Icon::IdCardO,
            "Employee's Application",
            10.0,
            15.0,
            &Colors::Red,
        ))
        .align_y(Alignment::Center);

    let content = Column::new()
        .push(content_1)
        .push(horizontal_rule(2.0))
        .push(content_2)
        .spacing(5);

    container(content)
        .padding(Padding::from(15))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
}

fn board<'a>(
    icon: Icon,
    title: &'a str,
    partial: f64,
    total: f64,
    icon_color: &'a Colors,
) -> Container<'a, Message> {
    let percentage = (partial / total) * 100.0;

    let chart_percentage = if std::cmp::PartialOrd::lt(&percentage.clone(), &50.0) {
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
    let tracker_color = if std::cmp::PartialOrd::lt(&percentage.clone(), &50.0) {
        &Colors::Red
    } else {
        &Colors::DarkPastelGreen
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
                .push(text(format!("{partial} / {total}")).size(24))
                .push(
                    container(
                        Row::new()
                            .push(chart_percentage)
                            .push(text(format!("{percentage:.0}%")).size(18))
                            .spacing(5),
                    )
                    .padding(Padding::from([2.5, 5.0]))
                    .style(tracker_container_style(tracker_color)),
                )
                .spacing(25),
        )
        .push(text(title).size(24).align_x(Alignment::Start))
        .spacing(5)
        .padding(Padding::from(18));

    container(board)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
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
