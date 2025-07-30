use iced::{widget::Text, Theme};

use crate::gui::styles::style_constant::ICONS;

pub enum Icon {
    UserDelete,
    UserAdd,
    Users,
    Calendar,
    Search,
    Logout,
    Clipboard,
    Cog,
    DownOpen,
    RightOpen,
    LeftOpen1,
    UpOpen,
    Plus,
    ThLarge,
    SortAltDown,
    SortAltUp,
    Language,
    Filter,
    DocTextInv,
    Cancel,
}

impl Icon {
    pub fn codepoint(&self) -> char {
        match self {
            Icon::UserDelete => char::from_u32(59393).unwrap(),
            Icon::UserAdd => char::from_u32(59392).unwrap(),
            Icon::Users => char::from_u32(59394).unwrap(),
            Icon::Calendar => char::from_u32(59395).unwrap(),
            Icon::Search => char::from_u32(59396).unwrap(),
            Icon::Logout => char::from_u32(59397).unwrap(),
            Icon::Clipboard => char::from_u32(59398).unwrap(),
            Icon::Cog => char::from_u32(59399).unwrap(),
            Icon::DownOpen => char::from_u32(59401).unwrap(),
            Icon::RightOpen => char::from_u32(59403).unwrap(),
            Icon::LeftOpen1 => char::from_u32(59402).unwrap(),
            Icon::UpOpen => char::from_u32(59400).unwrap(),
            Icon::Plus => char::from_u32(59404).unwrap(),
            Icon::ThLarge => char::from_u32(59405).unwrap(),
            Icon::SortAltDown => char::from_u32(61793).unwrap(),
            Icon::SortAltUp => char::from_u32(61792).unwrap(),
            Icon::Language => char::from_u32(61867).unwrap(),
            Icon::Filter => char::from_u32(61616).unwrap(),
            Icon::DocTextInv => char::from_u32(61788).unwrap(),
            Icon::Cancel => char::from_u32(59406).unwrap(),
        }
    }

    pub fn to_text<'a>(&self) -> Text<'a, Theme> {
        Text::new(self.codepoint().to_string()).font(ICONS)
    }
}
