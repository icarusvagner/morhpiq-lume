#![allow(clippy::module_name_repetitions)]

use iced::{widget::container::Style, Background, Border, Color, Shadow, Theme, Vector};

use crate::gui::styles::style_constant::{Colors, BORDER_ROUNDED_RADIUS, BORDER_WIDTH};

#[derive(Default)]
pub enum ContainerStyle {
    #[default]
    Standard,
    Rounded,
    Outline,
    Ghost,
    Shadowed,
    ShadowOutline,
}

impl ContainerStyle {
    fn appearance(&self, theme: &Theme) -> Style {
        Style {
            text_color: Some(Colors::Night.get()),
            background: Some(match self {
                ContainerStyle::Standard => Background::Color(Colors::AntiFlashWhite.get()),
                ContainerStyle::Ghost => Background::Color(Colors::Ghost.get()),
                _ => Background::Color(Color::WHITE),
            }),
            border: Border {
                radius: match self {
                    ContainerStyle::Outline | ContainerStyle::ShadowOutline => 5.0.into(),
                    ContainerStyle::Rounded => BORDER_ROUNDED_RADIUS.into(),
                    _ => 0.0.into(),
                },
                width: match self {
                    ContainerStyle::Rounded | ContainerStyle::Outline => BORDER_WIDTH * 2.0,
                    _ => 0.0,
                },
                color: Color::default(),
            },
            shadow: match self {
                ContainerStyle::Shadowed | ContainerStyle::ShadowOutline => Shadow {
                    color: Colors::Night.get(),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 50.0,
                },
                _ => Shadow::default(),
            },
        }
    }
}
