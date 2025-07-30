use iced::{widget::Text, Theme};

use crate::gui::styles::style_constant::ICONS;

pub enum Icon {
    Cancel,
    UserTimes,
    UserPlus,
    Users,
    User,
    Calendar,
    Search,
    Logout,
    Clipboard,
    Cog,
    CogAlt,
    UpOpen,
    RightOpen,
    LeftOpen,
    DownOpen,
    Plus,
    ThLarge,
    SortAltDown,
    SortAltUp,
    Language,
    Language1,
}

impl Icon {
    pub fn codepoint(&self) -> char {
        match self {
            Icon::Cancel => 'a',
            Icon::UserTimes => 'b',
            Icon::UserPlus => 'c',
            Icon::Users => 'd',
            Icon::User => 'e',
            Icon::Calendar => 'f',
            Icon::Search => 'g',
            Icon::Logout => 'h',
            Icon::Clipboard => 'i',
            Icon::Cog => 'j',
            Icon::CogAlt => 'k',
            Icon::UpOpen => 'l',
            Icon::RightOpen => 'm',
            Icon::LeftOpen => 'n',
            Icon::DownOpen => 'o',
            Icon::Plus => 'p',
            Icon::ThLarge => 'q',
            Icon::SortAltDown => 'r',
            Icon::SortAltUp => 's',
            Icon::Language => 't',
            Icon::Language1 => 'u',
        }
    }

    pub fn to_text<'a>(&self) -> Text<'a, Theme> {
        Text::new(self.codepoint().to_string()).font(ICONS)
    }
}
