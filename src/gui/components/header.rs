use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        button, container, horizontal_space, text, text_input, vertical_rule, Button, Column,
        Container, Row, Text,
    },
    Alignment, Element, Length,
};
use iced_aw::ContextMenu;

use crate::{
    gui::{
        styles::{
            button::ButtonType, container::ContainerType, context_menu::ContextMenuType,
            text::TextType, text_input::TextInputType, types::style_type::StyleType,
        },
        types::message::Message,
        views::{loginview::LoginViewMessage, mainview::MainViews},
    },
    utils::types::icon::Icon,
};

#[derive(Clone, Debug)]
enum ProfileChoice {
    EditProfile,
    Settings,
    SignOut,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum HeaderMenuMessage {
    EditProfile,
    Settings,
    SignOut,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct HeaderMenu;

impl HeaderMenu {
    pub fn view<'a>() -> Element<'a, Message, StyleType> {
        let search_input = Row::new()
            .push(
                text_input("Search...", "")
                    .class(TextInputType::Ghost)
                    .width(Length::Fill),
            )
            .push(Icon::Search.to_text().size(20).class(TextType::Neutral))
            .push(vertical_rule(2.0))
            .align_y(Alignment::Center)
            .width(200.0)
            .spacing(5);

        let profile_choice = Container::new(
            Button::new(
                Text::new("John Doe")
                    .size(24)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .class(TextType::Neutral),
            )
            .class(ButtonType::Ghost),
        )
        .class(ContainerType::Ghost);

        let utilities = Row::new()
            .push(
                Icon::Mail
                    .to_text()
                    .size(24)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .class(TextType::Neutral),
            )
            .push(
                Icon::Bell
                    .to_text()
                    .size(24)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .class(TextType::Neutral),
            )
            .push(
                ContextMenu::new(profile_choice, || {
                    container(
                        Column::new()
                            .push(
                                Button::new(
                                    Row::new()
                                        .push(
                                            Icon::UserRoundPlus
                                                .to_text()
                                                .size(18)
                                                .align_x(Horizontal::Center)
                                                .align_y(Vertical::Center)
                                                .class(TextType::Primary),
                                        )
                                        .push(
                                            Text::new("Edit Profile")
                                                .size(18)
                                                .class(TextType::Neutral),
                                        )
                                        .spacing(10)
                                        .align_y(Vertical::Center),
                                )
                                .width(Length::Fill)
                                .class(ButtonType::GhostHovered)
                                .on_press(Message::ChangeMainView(MainViews::EditProfile)),
                            )
                            .push(
                                Button::new(
                                    Row::new()
                                        .push(
                                            Icon::Settings
                                                .to_text()
                                                .size(18)
                                                .align_x(Horizontal::Center)
                                                .align_y(Vertical::Center)
                                                .class(TextType::Primary),
                                        )
                                        .push(
                                            Text::new("Settings").size(18).class(TextType::Neutral),
                                        )
                                        .spacing(10)
                                        .align_y(Vertical::Center),
                                )
                                .class(ButtonType::GhostHovered)
                                .width(Length::Fill)
                                .on_press(Message::ChangeMainView(MainViews::Settings)),
                            )
                            .push(
                                Button::new(
                                    Row::new()
                                        .push(
                                            Icon::LogOut
                                                .to_text()
                                                .size(18)
                                                .align_x(Horizontal::Center)
                                                .align_y(Vertical::Center)
                                                .class(TextType::Primary),
                                        )
                                        .push(
                                            Text::new("Sign Out").size(18).class(TextType::Neutral),
                                        )
                                        .spacing(10)
                                        .align_y(Vertical::Center),
                                )
                                .class(ButtonType::GhostHovered)
                                .width(Length::Fill)
                                .on_press(Message::LoginViewMessage(LoginViewMessage::Logout)),
                            )
                            .width(Length::Fill)
                            .align_x(Horizontal::Center),
                    )
                    .width(200.0)
                    .padding(10)
                    .class(ContainerType::Base200)
                    .into()
                })
                .class(ContextMenuType::Ghost),
            )
            .push(vertical_rule(2.0))
            .push(
                button(
                    Icon::MessageSquareMore
                        .to_text()
                        .size(24)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .class(TextType::Neutral),
                )
                .class(ButtonType::Ghost),
            )
            .align_y(Alignment::Center)
            .spacing(10);

        let content = Row::new()
            .push(
                Icon::Morphiq
                    .to_text()
                    .size(28)
                    .width(80.0)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .class(TextType::Neutral),
            )
            .push(search_input)
            .push(horizontal_space())
            .push(utilities)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill);

        container(content)
            .padding(5.0)
            .height(Length::Fixed(65.0))
            .width(Length::Fill)
            .class(ContainerType::Base100)
            .into()
    }
}
