use iced::{
    widget::{button, container, text, text_input, Column, Container},
    Alignment, Length, Padding, Theme,
};

use crate::gui::{
    components::header::header_view,
    morphiq::Morphiq,
    types::{login::LoginMessage, message::Message},
};

pub fn login_view(morphiq: &Morphiq) -> Container<'_, Message> {
    let username_input = Column::new()
        .push(text("Username *").size(18))
        .push(
            text_input("Enter your username *", &morphiq.login_field.username)
                .on_input(|val| {
                    Message::LoginMessage(LoginMessage::InputFieldChange(
                        val,
                        morphiq.login_field.password.clone(),
                    ))
                })
                .padding(Padding::from(10))
                .line_height(text::LineHeight::Relative(1.75)),
        )
        .width(Length::Fixed(450.0))
        .spacing(15);
    let password_input = Column::new()
        .push(text("Password *").size(18))
        .push(
            text_input("Enter your password", &morphiq.login_field.password)
                .on_input(|val| {
                    Message::LoginMessage(LoginMessage::InputFieldChange(
                        morphiq.login_field.username.clone(),
                        val,
                    ))
                })
                .secure(true)
                .padding(Padding::from(10))
                .line_height(text::LineHeight::Relative(1.75)),
        )
        .width(Length::Fixed(450.0))
        .spacing(15);

    let content = Column::new()
        .push(header_view(morphiq))
        .push(username_input)
        .push(password_input)
        .push(
            button(
                text("Submit")
                    .size(18)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
            )
            .width(Length::Fixed(450.0))
            .height(45.0)
            .on_press(Message::LoginMessage(LoginMessage::LoginSubmit)),
        )
        .spacing(20)
        .align_x(Alignment::Center);

    container(content)
        .width(Length::Fixed(450.0))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(login_continer_style())
}

fn login_continer_style() -> impl Fn(&Theme) -> container::Style {
    |_| container::Style {
        shadow: iced::Shadow {
            color: iced::Color::BLACK,
            offset: iced::Vector::new(2.0, 4.0),
            blur_radius: 50.0,
        },
        background: None,
        text_color: None,
        border: iced::Border::default(),
    }
}
