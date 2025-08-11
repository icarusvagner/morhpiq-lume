use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        container, horizontal_rule, horizontal_space, scrollable, text, Button, Column, Container,
        Row, Scrollable, Text,
    },
    Alignment, Background, Element, Length, Theme,
};

use crate::{
    gui::{
        components::{
            charts::donut::donut_chart,
            types::{
                button::{button_style, ButtonType},
                status::Statuses,
            },
        },
        morphiq::Morphiq,
        styles::{
            container::ContainerStyle,
            style_constant::{Colors, OUTFIT_BOLD, OUTFIT_MEDIUM, RALEWAY_ITALIC, RALEWAY_REGULAR},
        },
        types::message::Message,
        views::home::panes::display_status,
    },
    utils::icons::Icon,
};

pub fn employee_status<'a>(_morphiq: &Morphiq) -> Container<'a, Message> {
    let genders_label = Column::new()
        .push(
            Row::new()
                .push(
                    Container::new(Text::new(""))
                        .width(20.0)
                        .height(20.0)
                        .style(GenderLabelDot::Primary.to_style()),
                )
                .push(
                    Text::new("Male")
                        .size(24)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .font(OUTFIT_MEDIUM),
                )
                .spacing(5)
                .align_y(Alignment::Center),
        )
        .push(
            Row::new()
                .push(
                    Container::new(Text::new(""))
                        .width(20.0)
                        .height(20.0)
                        .style(GenderLabelDot::Secondary.to_style()),
                )
                .push(
                    Text::new("Female")
                        .size(24)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .font(OUTFIT_MEDIUM),
                )
                .spacing(5)
                .align_y(Alignment::Center),
        )
        .push(
            Row::new()
                .push(
                    Container::new(Text::new(""))
                        .width(20.0)
                        .height(20.0)
                        .style(GenderLabelDot::Tertiary.to_style()),
                )
                .push(
                    Text::new("Other")
                        .size(24)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .font(OUTFIT_MEDIUM),
                )
                .spacing(5)
                .align_y(Alignment::Center),
        )
        .spacing(15);

    let gender_count = Container::new(
        Column::new()
            .push(
                Row::new()
                    .push(
                        Text::new("Genders")
                            .size(24)
                            .font(OUTFIT_BOLD)
                            .align_x(Horizontal::Left)
                            .align_y(Vertical::Center),
                    )
                    .push(horizontal_space())
                    .push(
                        Container::new(
                            Button::new(
                                Row::new()
                                    .push(Text::new("Filter").size(18).font(RALEWAY_REGULAR))
                                    .push(Icon::Calendar.to_text().size(18))
                                    .spacing(10),
                            )
                            .style(button_style(&ButtonType::Nothing)),
                        )
                        .style(ContainerStyle::Rounded.appearance()),
                    )
                    .align_y(Alignment::Center),
            )
            .push(horizontal_rule(2.0))
            .push(
                Row::new()
                    .push(donut_chart(25, 10, 5))
                    .push(genders_label)
                    .spacing(5)
                    .align_y(Alignment::Center),
            )
            .spacing(15),
    )
    .width(Length::Fixed(500.0))
    .padding(15)
    .style(ContainerStyle::Rounded.appearance());

    let col = Column::new()
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            &Statuses::Active,
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            &Statuses::Inactive,
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            &Statuses::Onboarding,
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            &Statuses::Leave,
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            &Statuses::Active,
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            &Statuses::Inactive,
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            &Statuses::Onboarding,
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            &Statuses::Leave,
        ))
        .padding(15)
        .spacing(20);

    let scrollable_content: Element<Message> = Element::from(
        Scrollable::new(col)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(0.0)
                    .margin(0.0)
                    .scroller_width(1.0)
                    .anchor(scrollable::Anchor::Start),
            ))
            .width(Length::Fill)
            .height(Length::Fill),
    );

    let scroll_content =
        Container::new(scrollable_content).style(ContainerStyle::Rounded.appearance());

    let content = Row::new()
        .push(scroll_content)
        .push(gender_count)
        .spacing(15);

    Container::new(content).height(Length::Fixed(400.0))
}

fn employee_component<'a>(
    fullname: &'a str,
    email: &'a str,
    department: &'a str,
    id_no: u32,
    status: &'a Statuses,
) -> Container<'a, Message> {
    let content = Row::new()
        .push(
            Column::new()
                .push(
                    Text::new(fullname)
                        .size(20)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center),
                )
                .push(
                    Text::new(email)
                        .size(18)
                        .font(RALEWAY_ITALIC)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center),
                )
                .spacing(5),
        )
        .push(horizontal_space())
        .push(
            Text::new(id_no)
                .size(20)
                .width(50)
                .align_x(Horizontal::Right)
                .align_y(Vertical::Center),
        )
        .push(horizontal_space())
        .push(display_status(status))
        .spacing(15);

    Container::new(content).width(Length::Fill)
}

#[derive(Debug, Clone)]
enum GenderLabelDot {
    Primary,
    Secondary,
    Tertiary,
}

impl GenderLabelDot {
    fn to_style(&self) -> impl Fn(&Theme) -> container::Style {
        move |_t| container::Style {
            background: match self {
                GenderLabelDot::Primary => Some(Background::Color(Colors::DarkPastelGreen.get())),
                GenderLabelDot::Secondary => Some(Background::Color(Colors::Citrine.get())),
                GenderLabelDot::Tertiary => Some(Background::Color(Colors::Silver.get())),
            },
            border: iced::Border {
                radius: 50.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
