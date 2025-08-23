use iced::widget::Text;

use crate::{
    gui::styles::types::style_type::StyleType,
    translations::{
        translations::{general_translation, notifications_translation, style_translation},
        types::language::Language,
    },
    utils::types::icon::Icon,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SettingsView {
    /// Settings Notifications view
    Notifications,
    /// Settings Appearance view
    Appearance,
    /// General Settings,
    General,
}

impl SettingsView {
    pub const ALL: [SettingsView; 3] = [
        SettingsView::Notifications,
        SettingsView::Appearance,
        SettingsView::General,
    ];

    pub fn get_tab_label(&self, language: Language) -> &str {
        match self {
            SettingsView::Notifications => notifications_translation(language),
            SettingsView::Appearance => style_translation(language),
            SettingsView::General => general_translation(language),
        }
    }

    pub fn icon<'a>(self) -> Text<'a, StyleType> {
        match self {
            SettingsView::Notifications => Icon::Bell,
            SettingsView::Appearance => Icon::Palette,
            SettingsView::General => Icon::SlidersHorizontal,
        }
        .to_text()
    }

    // Add translation later
}
