use iced::{
    widget::{button, container, svg::Handle, text, text_input, Column, Row, Svg},
    Alignment, Element, Length, Padding, Theme,
};

use crate::{
    gui::{
        components::types::{
            button::{button_style, ButtonType},
            input::InputType,
        },
        morphiq::{Morphiq, SVG_FULLLOGO_BYTES},
        styles::{container::ContainerStyle, style_constant::Colors},
        types::{
            login::{LoginMessage, LoginView},
            message::Message,
        },
    },
    utils::icons::Icon,
};

impl LoginView {
    pub fn view(&self, morphiq: &Morphiq) -> Element<'_, Message> {
        let what_eye = if morphiq.pass_secure {
            Icon::EyeOff
                .to_text()
                .size(24)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
        } else {
            Icon::Eye
                .to_text()
                .size(24)
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
            .push(text("Username *").size(18))
            .push(
                text_input("Enter your username *", &self.username)
                    .on_input(|val| Message::LoginView(LoginMessage::UsernameChange(val)))
                    .width(Length::Fill)
                    .padding(Padding::from(10))
                    .line_height(text::LineHeight::Relative(1.75))
                    .style(InputType::Outline.input_style())
                    .width(Length::Fixed(450.0)),
            )
            .width(Length::Fixed(450.0))
            .spacing(15);

        let password_input = Column::new()
            .push(text("Password *").size(18))
            .push(
                container(
                    Row::new()
                        .push(
                            text_input("Enter your password", &self.password)
                                .on_input(|val| {
                                    Message::LoginView(LoginMessage::PasswordChange(val))
                                })
                                .width(Length::Fill)
                                .secure(morphiq.pass_secure)
                                .style(InputType::Base.input_style())
                                .line_height(text::LineHeight::Relative(1.75)),
                        )
                        .push(
                            button(what_eye)
                                .style(button_style(&ButtonType::Nothing))
                                .on_press(Message::ToggleShowPwd(!morphiq.pass_secure)),
                        )
                        .width(Length::Fill),
                )
                .width(Length::Fixed(450.0))
                .padding(Padding::from(5))
                .style(ContainerStyle::Outline.appearance()),
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
                            .align_y(Alignment::Center),
                    )
                    .style(button_style(&ButtonType::Primary))
                    .width(Length::Fixed(450.0))
                    .height(45.0)
                    .on_press(Message::LoginView(LoginMessage::LoginSubmit)),
                )
                .spacing(20)
                .align_x(Alignment::Center),
        )
        .style(Self::login_container_style())
        .padding(Padding::from(20))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

        container(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    fn login_container_style() -> impl Fn(&Theme) -> container::Style {
        |_| container::Style {
            shadow: iced::Shadow {
                color: Colors::Night.get(),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 30.0,
            },
            background: None,
            text_color: None,
            border: iced::Border::default(),
        }
    }
}
