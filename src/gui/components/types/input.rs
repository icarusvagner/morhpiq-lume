use iced::{widget::text_input, Background, Border, Color, Theme};

use crate::gui::styles::{
    style_constant::Colors,
    types::{gradient_type::GradientType, palette::mix_colors},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputType {
    Base,
    Primary,
    Secondary,
    Tertiary,
    Accent,
    Ghost,
    Outline,
}

impl Default for InputType {
    fn default() -> Self {
        Self::Base
    }
}

impl InputType {
    pub fn input_style(&self) -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
        move |_t, _e| text_input::Style {
            background: match self {
                InputType::Primary => Background::Color(Colors::UTOrange.get()),
                InputType::Secondary => Background::Color(Colors::SelectiveYellow.get()),
                InputType::Tertiary => Background::Color(Colors::PrussianBlue.get()),
                InputType::Accent => Background::Gradient(GradientType::Mild.set()),
                _ => Background::Color(Color::default()),
            },
            border: Border {
                color: match self {
                    InputType::Primary => Colors::UTOrange.get(),
                    InputType::Secondary => Colors::SelectiveYellow.get(),
                    InputType::Tertiary => Colors::PrussianBlue.get(),
                    InputType::Accent => {
                        mix_colors(Colors::UTOrange.get(), Colors::SelectiveYellow.get())
                    }
                    InputType::Ghost => match _e {
                        text_input::Status::Focused => Colors::Night.get(),
                        _ => Color::TRANSPARENT,
                    },
                    InputType::Outline => Colors::Night.get(),
                    _ => Color::TRANSPARENT,
                },
                width: 1.0,
                radius: 1.0.into(),
            },
            icon: Color::default(),
            placeholder: match self {
                InputType::Ghost => Colors::AntiFlashWhite.get(),
                InputType::Base | InputType::Tertiary | InputType::Outline => Colors::Silver.get(),
                _ => Color::default(),
            },
            value: match self {
                InputType::Ghost => Colors::AntiFlashWhite.get(),
                InputType::Base | InputType::Outline | InputType::Tertiary => Colors::Night.get(),
                _ => Colors::Night.get(),
            },
            selection: Colors::UTOrange.get(),
        }
    }
}
