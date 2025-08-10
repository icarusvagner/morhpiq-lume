use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        container, horizontal_space, scrollable, text, Column, Container, Row, Scrollable, Text,
    },
    Element, Length,
};

use crate::gui::{
    morphiq::Morphiq, styles::style_constant::RALEWAY_ITALIC, types::message::Message,
};

pub fn employee_status<'a>(_morphiq: &Morphiq) -> Container<'a, Message> {
    let col = Column::new()
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            "Active",
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            "Active",
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            "Active",
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            "Active",
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            "Active",
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            "Active",
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            "Active",
        ))
        .push(employee_component(
            "Descartin, Lance Phillip",
            "dev.castlebyte@gmail.com",
            "Programmer",
            1001,
            "Active",
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

    Container::new(scrollable_content)
        .height(Length::Fixed(400.0))
        .width(Length::Fill)
}

fn employee_component<'a>(
    fullname: &'a str,
    email: &'a str,
    department: &'a str,
    id_no: u32,
    status: &'a str,
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
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center),
        )
        .push(horizontal_space())
        .push(
            Text::new(status)
                .size(20)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center),
        )
        .spacing(15);

    Container::new(content).width(Length::Fill)
}
