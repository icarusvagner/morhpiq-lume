use iced::{
    font::{Family, Stretch, Style, Weight},
    Color, Font,
};

use crate::gui::morphiq::ICON_FONT_FAMILY_NAME;

// dat theme colors
const UT_ORANGE: Color = Color {
    r: 251.0,
    g: 133.0,
    b: 0.0,
    a: 1.0,
};

const SELECTIVE_YELLOW: Color = Color {
    r: 255.0,
    g: 183.0,
    b: 3.0,
    a: 1.0,
};

const PRUSSIAN_BLUE: Color = Color {
    r: 2.0,
    g: 48.0,
    b: 71.0,
    a: 1.0,
};

const BLUE_GREEN: Color = Color {
    r: 33.0,
    g: 158.0,
    b: 188.0,
    a: 1.0,
};

const SKY_BLUE: Color = Color {
    r: 142.0,
    g: 202.0,
    b: 230.0,
    a: 1.0,
};

/// fonts
// Orbitron Font
pub const ORBITRON_REGULAR_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Orbitron/Orbitron-Regular.ttf");
pub const ORBITRON_REGULAR: Font = Font {
    family: Family::Name(ICON_FONT_FAMILY_NAME),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

pub const ORBITRON_MEDIUM_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Orbitron/Orbitron-Medium.ttf");
pub const ORBITRON_MEDIUM: Font = Font {
    family: Family::Name(ICON_FONT_FAMILY_NAME),
    weight: Weight::Medium,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

pub const ORBITRON_SEMIBOLD_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Orbitron/Orbitron-SemiBold.ttf");
pub const ORBITRON_SEMIBOLD: Font = Font {
    family: Family::Name(ICON_FONT_FAMILY_NAME),
    weight: Weight::Semibold,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

pub const ORBITRON_BOLD_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Orbitron/Orbitron-Bold.ttf");
pub const ORBITRON_BOLD: Font = Font {
    family: Family::Name(ICON_FONT_FAMILY_NAME),
    weight: Weight::Bold,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

pub const ORBITRON_EXTRABOLD_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Orbitron/Orbitron-ExtraBold.ttf");
pub const ORBITRON_EXTRABOLD: Font = Font {
    family: Family::Name(ICON_FONT_FAMILY_NAME),
    weight: Weight::ExtraBold,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

pub const ORBITRON_BLACK_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Orbitron/Orbitron-Black.ttf");
pub const ORBITRON_BLACK: Font = Font {
    family: Family::Name(ICON_FONT_FAMILY_NAME),
    weight: Weight::Black,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

// Raleway Font
pub const RALEWAY_REGULAR_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-Regular.ttf");
pub const RALEWAY_REGULAR: Font = Font {
    family: Family::Name(ICON_FONT_FAMILY_NAME),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

// font to display icons
pub const ICONS_BYTES: &[u8] = include_bytes!("../../../assets/fonts/icons/font/fontello.ttf");
pub const ICONS: Font = Font::with_name(ICON_FONT_FAMILY_NAME);

// font sizes

pub const FONT_SIZE_FOOTER: f32 = 14.3;
pub const FONT_SIZE_BODY: f32 = 16.8;
pub const FONT_SIZE_SUBTITLE: f32 = 18.3;
pub const FONT_SIZE_TITLE: f32 = 19.9;

// border styles
pub const BORDER_WIDTH: f32 = 2.0;
pub const CHARTS_LINE_BORDER: u32 = 1;
pub const BORDER_ROUNDED_RADIUS: f32 = 15.0;
pub const BORDER_BUTTON_RADIUS: f32 = 180.0;

// red colors for alerts
pub const RED_ALERT_COLOR_NIGHTLY: Color = Color {
    r: 1.0,
    g: 0.4,
    b: 0.4,
    a: 1.0,
};
pub const RED_ALERT_COLOR_DAILY: Color = Color {
    r: 0.701_960_8,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
