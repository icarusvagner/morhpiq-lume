use crate::{
    gui::{
        morphiq::{Morphiq, SVG_FULLLOGO_BYTES},
        styles::{
            button::ButtonType, container::ContainerType, text::TextType,
            text_input::TextInputType, types::style_type::StyleType,
        },
        types::message::Message,
    },
    utils::types::icon::Icon,
};
use iced::{
    widget::{button, container, svg::Handle, text, text_input, Column, Row, Svg},
    Alignment, Element, Length, Padding,
};

#[derive(Debug, Clone, Default)]
pub struct LoginView {
    pub username: String,
    pub password: String,
    pub toggle_show: bool,
}

#[derive(Debug, Clone)]
pub enum LoginViewMessage {
    ChangeUsername(String),
    ChangePassword(String),
    ToggleShowPassword,
    LoginSubmitted,
    Logout,
}

impl LoginView {
    pub fn update(&mut self, message: LoginViewMessage) {
        match message {
            LoginViewMessage::ChangeUsername(username) => self.username = username,
            LoginViewMessage::ChangePassword(password) => self.password = password,
            LoginViewMessage::ToggleShowPassword => self.toggle_show = !self.toggle_show,
            LoginViewMessage::LoginSubmitted => {}
            LoginViewMessage::Logout => {
                self.username = String::new();
                self.password = String::new();
            }
        }
    }

    pub fn view(&self, morphiq: &Morphiq) -> Element<'_, Message, StyleType> {
        let what_eye = if !self.toggle_show {
            Icon::EyeOff
                .to_text()
                .size(20)
                .class(TextType::Neutral)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
        } else {
            Icon::Eye
                .to_text()
                .size(20)
                .class(TextType::Neutral)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
        };

        let logo_full = Row::new()
            .push(
                Svg::new(Handle::from_memory(SVG_FULLLOGO_BYTES))
                    .height(Length::Fixed(110.0))
                    .width(Length::Fill),
            )
            .width(Length::Fixed(450.0))
            .align_y(Alignment::Center);

        let username_input = Column::new()
            .push(text("Username *").size(18).class(TextType::Neutral))
            .push(
                container(
                    text_input("Enter your username *", &self.username)
                        .on_input(|val| {
                            Message::LoginViewMessage(LoginViewMessage::ChangeUsername(val))
                        })
                        .line_height(text::LineHeight::Relative(1.75))
                        .width(Length::Fixed(450.0))
                        .class(TextInputType::Ghost),
                )
                .padding(Padding::from(10))
                .class(ContainerType::Bordered),
            )
            .width(Length::Fixed(450.0))
            .spacing(15);

        let password_input = Column::new()
            .push(text("Password *").size(18).class(TextType::Neutral))
            .push(
                container(
                    Row::new()
                        .push(
                            text_input("Enter your password", &self.password)
                                .on_input(|val| {
                                    Message::LoginViewMessage(LoginViewMessage::ChangePassword(val))
                                })
                                .width(Length::Fill)
                                .secure(!self.toggle_show)
                                .line_height(text::LineHeight::Relative(1.75))
                                .class(TextInputType::Ghost),
                        )
                        .push(
                            button(what_eye)
                                .on_press(Message::LoginViewMessage(
                                    LoginViewMessage::ToggleShowPassword,
                                ))
                                .class(ButtonType::Ghost),
                        )
                        .align_y(Alignment::Center)
                        .width(Length::Fill),
                )
                .padding(Padding::from(10))
                .width(Length::Fill)
                .class(ContainerType::Bordered),
            )
            .width(Length::Fixed(450.0))
            .spacing(15);

        let content = container(
            Column::new()
                .push(logo_full)
                .push(username_input)
                .push(password_input)
                .push(
                    button(
                        text("Submit")
                            .size(20)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .class(TextType::Base100),
                    )
                    .class(ButtonType::Primary)
                    .width(Length::Fixed(450.0))
                    .height(45.0)
                    .on_press(Message::LoginViewMessage(LoginViewMessage::LoginSubmitted)),
                )
                .spacing(20)
                .align_x(Alignment::Center),
        )
        .class(ContainerType::Base100)
        .padding(Padding::from(20))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

        container(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
