use std::fmt::Display;

use iced::{
    widget::{button, container},
    Background, Border, Color, Theme,
};

use crate::gui::styles::style_constant::Colors;

pub enum Statuses {
    Active,
    Inactive,
    Onboarding,
    Leave,
}

impl Display for Statuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statuses::Active => write!(f, "Active"),
            Statuses::Inactive => write!(f, "Inactive"),
            Statuses::Onboarding => write!(f, "Onboarding"),
            Statuses::Leave => write!(f, "Leave"),
        }
    }
}

impl Statuses {
    pub fn button_style(&self) -> impl Fn(&Theme, button::Status) -> button::Style {
        move |_t, _e| button::Style {
            background: match self {
                Statuses::Active => Some(Background::Color(Color {
                    a: 0.20,
                    ..Colors::DarkPastelGreen.get()
                })),
                Statuses::Inactive => Some(Background::Color(Color {
                    a: 0.20,
                    ..Colors::Red.get()
                })),
                Statuses::Onboarding => Some(Background::Color(Color {
                    a: 0.20,
                    ..Colors::Pumpkin.get()
                })),
                Statuses::Leave => Some(Background::Color(Color {
                    a: 0.20,
                    ..Colors::Xanthous.get()
                })),
            },
            text_color: match self {
                Statuses::Active => Colors::DarkPastelGreen.get(),
                Statuses::Inactive => Colors::Red.get(),
                Statuses::Onboarding => Colors::Pumpkin.get(),
                Statuses::Leave => Colors::Xanthous.get(),
            },
            ..Default::default()
        }
    }

    pub fn solid_container_style(&self) -> impl Fn(&Theme) -> container::Style {
        move |_t| container::Style {
            background: match self {
                Statuses::Active => Some(Background::Color(Colors::DarkPastelGreen.get())),
                Statuses::Inactive => Some(Background::Color(Colors::Red.get())),
                Statuses::Onboarding => Some(Background::Color(Colors::Pumpkin.get())),
                Statuses::Leave => Some(Background::Color(Colors::Xanthous.get())),
            },
            ..Default::default()
        }
    }

    pub fn container_style(&self) -> impl Fn(&Theme) -> container::Style {
        move |_t| container::Style {
            background: match self {
                Statuses::Active => Some(Background::Color(Color {
                    a: 0.20,
                    ..Colors::DarkPastelGreen.get()
                })),
                Statuses::Inactive => Some(Background::Color(Color {
                    a: 0.20,
                    ..Colors::Red.get()
                })),
                Statuses::Onboarding => Some(Background::Color(Color {
                    a: 0.20,
                    ..Colors::Pumpkin.get()
                })),
                Statuses::Leave => Some(Background::Color(Color {
                    a: 0.20,
                    ..Colors::Xanthous.get()
                })),
            },
            text_color: match self {
                Statuses::Active => Some(Colors::DarkPastelGreen.get()),
                Statuses::Inactive => Some(Colors::Red.get()),
                Statuses::Onboarding => Some(Colors::Pumpkin.get()),
                Statuses::Leave => Some(Colors::Xanthous.get()),
            },
            ..Default::default()
        }
    }

    pub fn rounded_container_style(&self) -> impl Fn(&Theme) -> container::Style {
        move |_t| container::Style {
            background: match self {
                Statuses::Active => Some(Background::Color(Color {
                    a: 0.20,
                    ..Colors::DarkPastelGreen.get()
                })),
                Statuses::Inactive => Some(Background::Color(Color {
                    a: 0.20,
                    ..Colors::Red.get()
                })),
                Statuses::Onboarding => Some(Background::Color(Color {
                    a: 0.20,
                    ..Colors::Pumpkin.get()
                })),
                Statuses::Leave => Some(Background::Color(Color {
                    a: 0.20,
                    ..Colors::Xanthous.get()
                })),
            },
            text_color: match self {
                Statuses::Active => Some(Colors::DarkPastelGreen.get()),
                Statuses::Inactive => Some(Colors::Red.get()),
                Statuses::Onboarding => Some(Colors::Pumpkin.get()),
                Statuses::Leave => Some(Colors::Xanthous.get()),
            },
            border: Border {
                radius: 100.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
