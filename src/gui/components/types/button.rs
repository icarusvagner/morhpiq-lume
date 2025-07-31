use iced::{widget::button, Color};

use crate::gui::styles::style_constant::Colors;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonType {
    Ghost,
    Primary,
    Secondary,
    Tertiary,
    Accent1,
    Accent2,
    Outline,
    Nothing,
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Primary
    }
}

pub fn button_style(
    btn_type: &ButtonType,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_t, _e| button::Style {
        background: match btn_type {
            ButtonType::Ghost => Some(iced::Background::Color(Colors::Ghost.get())),
            ButtonType::Primary => Some(iced::Background::Color(Colors::UTOrange.get())),
            ButtonType::Secondary => Some(iced::Background::Color(Colors::SelectiveYellow.get())),
            ButtonType::Tertiary => Some(iced::Background::Color(Colors::PrussianBlue.get())),
            ButtonType::Accent1 => Some(iced::Background::Color(Colors::BlueGreen.get())),
            ButtonType::Accent2 => Some(iced::Background::Color(Colors::SkyBlue.get())),
            _ => None,
        },
        text_color: match btn_type {
            ButtonType::Nothing | ButtonType::Outline | ButtonType::Ghost => Colors::Night.get(),
            _ => Color::WHITE,
        },
        border: match btn_type {
            ButtonType::Outline => iced::Border {
                color: Colors::Night.get(),
                width: 1.0,
                radius: iced::border::Radius::new(5.0),
            },
            _ => iced::Border::default(),
        },
        shadow: iced::Shadow::default(),
    }
}
