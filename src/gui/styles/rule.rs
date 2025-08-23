//! Rule Style

#[allow(clippy::module_name_repetitions)]
use iced::widget::rule::{Catalog, FillMode, Style};

use crate::gui::styles::types::style_type::StyleType;

#[derive(Default)]
pub enum RuleType {
    #[default]
    Straight,
    Dashed,
}

impl RuleType {
    fn appearance(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();

        Style {
            color: colors.base_content,
            width: match self {
                RuleType::Straight => 1,
                RuleType::Dashed => 3,
            },
            radius: 0.0.into(),
            fill_mode: FillMode::Full,
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = RuleType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class.appearance(self)
    }
}
