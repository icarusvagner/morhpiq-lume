use iced::widget::button;

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
            ButtonType::Nothing => None,
            ButtonType::Ghost => Some(iced::Background::Color(Colors::Ghost.get())),
            ButtonType::Primary => Some(iced::Background::Color(Colors::UTOrange.get())),
            ButtonType::Secondary => Some(iced::Background::Color(Colors::SelectiveYellow.get())),
            ButtonType::Tertiary => Some(iced::Background::Color(Colors::PrussianBlue.get())),
            ButtonType::Accent1 => Some(iced::Background::Color(Colors::BlueGreen.get())),
            ButtonType::Accent2 => Some(iced::Background::Color(Colors::SkyBlue.get())),
            ButtonType::Outline => None,
        },
        text_color: if btn_type.eq(&ButtonType::Outline) || btn_type.eq(&ButtonType::Ghost) {
            Colors::Night.get()
        } else {
            Colors::AntiFlashWhite.get()
        },
        border: if btn_type.eq(&ButtonType::Outline) {
            iced::Border {
                color: Colors::Night.get(),
                width: 1.0,
                radius: iced::border::Radius::new(5.0),
            }
        } else {
            iced::Border::default()
        },
        shadow: iced::Shadow::default(),
    }
}
