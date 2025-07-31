use iced::{
    widget::{button, container, horizontal_space, text, Column, Container, Row},
    Alignment, Length, Padding,
};

use crate::{
    gui::{
        components::types::button::{button_style, ButtonType},
        morphiq::Morphiq,
        types::message::Message,
    },
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

    let row = Row::new()
        .push(greeting)
        .push(horizontal_space())
        .push(
            button(
                Row::new()
                    .push(horizontal_space())
                    .push(
                        Icon::UserPlus
                            .to_text()
                            .size(20)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center),
                    )
                    .push(
                        text("Attendance")
                            .size(20)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center),
                    )
                    .push(horizontal_space())
                    .spacing(2)
                    .align_y(Alignment::Center)
                    .width(Length::Fill),
            )
            .style(button_style(&ButtonType::Ghost))
            .width(250)
            .height(35)
            .on_press(Message::ChangeRunningPage(
                crate::gui::views::RunningView::Login,
            )),
        )
        .push(
            button(
                Row::new()
                    .push(horizontal_space())
                    .push(
                        Icon::Plus
                            .to_text()
                            .size(20)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center),
                    )
                    .push(
                        text("Add Employee")
                            .size(20)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center),
                    )
                    .push(horizontal_space())
                    .spacing(2)
                    .align_y(Alignment::Center)
                    .width(Length::Fill),
            )
            .style(button_style(&ButtonType::Ghost))
            .width(250)
            .height(35)
            .on_press(Message::ChangeRunningPage(
                crate::gui::views::RunningView::Login,
            )),
        )
        .padding(Padding::from(10.0))
        .spacing(20);

    container(row).width(Length::Fill)
}
