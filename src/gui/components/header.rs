use iced::{
    widget::{button, container, row, text, Column, Container, Row},
    Alignment, Length, Padding,
};

use crate::{
    gui::{morphiq::Morphiq, types::message::Message},
    utils::icons::Icon,
};

pub fn header_view(morphiq: &Morphiq) -> Container<'_, Message> {
    let greeting = Column::new()
        .push(
            text("Welcome back Lance")
                .size(16)
                .line_height(text::LineHeight::Relative(1.75)),
        )
        .push(text("Home / Dashboard").size(14))
        .spacing(5)
        .width(Length::Fill);
    let btn_action = Row::new().push(
        button(
            row![
                Icon::UserAdd
                    .to_text()
                    .size(20)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
                text("Attendance")
                    .size(20)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
            ]
            .width(Length::Fill)
            .spacing(5),
        )
        .width(250)
        .height(25)
        .padding(Padding::from([20, 5])),
    );

    let row = Row::new()
        .push(greeting)
        .push(btn_action)
        .padding(Padding::from([40, 10]))
        .spacing(20);

    container(row).width(Length::Fill)
}
