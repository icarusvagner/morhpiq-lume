use iced::{
    font::{Family, Stretch, Style, Weight},
    Color, Font,
};

use crate::gui::morphiq::ICON_FONT_FAMILY_NAME;

// dat theme colors
pub enum Colors {
    Ghost,
    UTOrange,
    SelectiveYellow,
    PrussianBlue,
    BlueGreen,
    SkyBlue,
    AntiFlashWhite,
    Night,
    Silver,
}

impl Colors {
    pub fn get(&self) -> Color {
        match self {
            Colors::Ghost => Color {
                r: 121.0 / 255.0,
                g: 121.0 / 255.0,
                b: 121.0 / 255.0,
                a: 10.0 / 100.0,
            },
            Colors::Silver => Color {
                r: 201.0 / 255.0,
                g: 201.0 / 255.0,
                b: 201.0 / 255.0,
                a: 1.0,
            },
            Colors::UTOrange => UT_ORANGE,
            Colors::SelectiveYellow => SELECTIVE_YELLOW,
            Colors::PrussianBlue => PRUSSIAN_BLUE,
            Colors::BlueGreen => BLUE_GREEN,
            Colors::SkyBlue => SKY_BLUE,
            Colors::AntiFlashWhite => ANTI_FLASH_WHITE,
            Colors::Night => NIGHT,
        }
    }
}

const UT_ORANGE: Color = Color {
    r: 251.0 / 255.0,
    g: 133.0 / 255.0,
    b: 0. / 255.00,
    a: 1.0,
};

const SELECTIVE_YELLOW: Color = Color {
    r: 255.0,
    g: 183.0 / 255.0,
    b: 3.0 / 255.0,
    a: 1.0,
};

const PRUSSIAN_BLUE: Color = Color {
    r: 2.0 / 255.0,
    g: 48.0 / 255.0,
    b: 71.0 / 255.0,
    a: 1.0,
};

const BLUE_GREEN: Color = Color {
    r: 33. / 255.00,
    g: 158.0 / 255.0,
    b: 188.0 / 255.0,
    a: 1.0,
};

const SKY_BLUE: Color = Color {
    r: 142.0 / 255.0,
    g: 202.0 / 255.0,
    b: 230.0 / 255.0,
    a: 1.0,
};

const ANTI_FLASH_WHITE: Color = Color {
    r: 238.0 / 255.0,
    g: 238.0 / 255.0,
    b: 238.0 / 255.0,
    a: 1.0,
};

const NIGHT: Color = Color {
    r: 21.0 / 255.0,
    g: 21.0 / 255.0,
    b: 21.0 / 255.0,
    a: 1.0,
};

/// fonts
// Outfit Font (Bytes)
pub const OUTFIT_EXTRALIGHT_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Outfit/Outfit-ExtraLight.ttf");
pub const OUTFIT_LIGHT_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Outfit/Outfit-Light.ttf");
pub const OUTFIT_THIN_BYTES: &[u8] = include_bytes!("../../../assets/fonts/Outfit/Outfit-Thin.ttf");
pub const OUTFIT_REGULAR_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Outfit/Outfit-Regular.ttf");
pub const OUTFIT_MEDIUM_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Outfit/Outfit-Medium.ttf");
pub const OUTFIT_SEMIBOLD_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Outfit/Outfit-SemiBold.ttf");
pub const OUTFIT_BOLD_BYTES: &[u8] = include_bytes!("../../../assets/fonts/Outfit/Outfit-Bold.ttf");
pub const OUTFIT_EXTRABOLD_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Outfit/Outfit-ExtraBold.ttf");
pub const OUTFIT_BLACK_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Outfit/Outfit-Black.ttf");

// Outfit Font (Font objects)
pub const OUTFIT_EXTRALIGHT: Font = Font {
    family: Family::Name("Outfit"),
    weight: Weight::ExtraLight,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const OUTFIT_LIGHT: Font = Font {
    family: Family::Name("Outfit"),
    weight: Weight::Light,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const OUTFIT_THIN: Font = Font {
    family: Family::Name("Outfit"),
    weight: Weight::Thin,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const OUTFIT_REGULAR: Font = Font {
    family: Family::Name("Outfit"),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const OUTFIT_MEDIUM: Font = Font {
    family: Family::Name("Outfit"),
    weight: Weight::Medium,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const OUTFIT_SEMIBOLD: Font = Font {
    family: Family::Name("Outfit"),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const OUTFIT_BOLD: Font = Font {
    family: Family::Name("Outfit"),
    weight: Weight::Bold,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const OUTFIT_EXTRABOLD: Font = Font {
    family: Family::Name("Outfit"),
    weight: Weight::ExtraBold,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const OUTFIT_BLACK: Font = Font {
    family: Family::Name("Outfit"),
    weight: Weight::Black,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

// Raleway Font (Bytes)
pub const RALEWAY_REGULAR_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-Regular.ttf");
pub const RALEWAY_BLACK_ITALIC_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-BlackItalic.ttf");
pub const RALEWAY_BLACK_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-Black.ttf");
pub const RALEWAY_BOLD_ITALIC_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-BoldItalic.ttf");
pub const RALEWAY_BOLD_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-Bold.ttf");
pub const RALEWAY_EXTRA_BOLD_ITALIC_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-ExtraBoldItalic.ttf");
pub const RALEWAY_EXTRA_BOLD_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-ExtraBold.ttf");
pub const RALEWAY_EXTRA_LIGHT_ITALIC_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-ExtraLightItalic.ttf");
pub const RALEWAY_EXTRA_LIGHT_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-ExtraLight.ttf");
pub const RALEWAY_ITALIC_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-Italic.ttf");
pub const RALEWAY_LIGHT_ITALIC_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-LightItalic.ttf");
pub const RALEWAY_LIGHT_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-Light.ttf");
pub const RALEWAY_MEDIUM_ITALIC_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-MediumItalic.ttf");
pub const RALEWAY_MEDIUM_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-Medium.ttf");
pub const RALEWAY_SEMI_BOLD_ITALIC_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-SemiBoldItalic.ttf");
pub const RALEWAY_SEMI_BOLD_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-SemiBold.ttf");
pub const RALEWAY_THIN_ITALIC_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-ThinItalic.ttf");
pub const RALEWAY_THIN_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-Thin.ttf");

// Raleway Font (Font objects)
pub const RALEWAY_REGULAR: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const RALEWAY_BLACK: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Black,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const RALEWAY_BLACK_ITALIC: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Black,
    stretch: Stretch::Normal,
    style: Style::Italic,
};
pub const RALEWAY_BOLD: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Bold,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const RALEWAY_BOLD_ITALIC: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Bold,
    stretch: Stretch::Normal,
    style: Style::Italic,
};
pub const RALEWAY_EXTRA_BOLD: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::ExtraBold,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const RALEWAY_EXTRA_BOLD_ITALIC: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::ExtraBold,
    stretch: Stretch::Normal,
    style: Style::Italic,
};
pub const RALEWAY_EXTRA_LIGHT: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::ExtraLight,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const RALEWAY_EXTRA_LIGHT_ITALIC: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::ExtraLight,
    stretch: Stretch::Normal,
    style: Style::Italic,
};
pub const RALEWAY_LIGHT: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Light,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const RALEWAY_LIGHT_ITALIC: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Light,
    stretch: Stretch::Normal,
    style: Style::Italic,
};
pub const RALEWAY_MEDIUM: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Medium,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const RALEWAY_MEDIUM_ITALIC: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Medium,
    stretch: Stretch::Normal,
    style: Style::Italic,
};
pub const RALEWAY_ITALIC: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Italic,
};
pub const RALEWAY_SEMI_BOLD: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Semibold,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const RALEWAY_SEMI_BOLD_ITALIC: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Semibold,
    stretch: Stretch::Normal,
    style: Style::Italic,
};
pub const RALEWAY_THIN: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Thin,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
pub const RALEWAY_THIN_ITALIC: Font = Font {
    family: Family::Name("Raleway"),
    weight: Weight::Thin,
    stretch: Stretch::Normal,
    style: Style::Italic,
};

// font to display icons
pub const ICONS_BYTES: &[u8] = include_bytes!("../../../assets/fonts/icons/font/icons.ttf");
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
