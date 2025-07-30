use std::borrow::Cow;

use iced::{Font, Pixels, Settings, Task};

use crate::gui::{
    morphiq::{Morphiq, FONT_FAMILY_NAME},
    styles::style_constant::{
        FONT_SIZE_BODY, ICONS_BYTES, ORBITRON_BLACK_BYTES, ORBITRON_BOLD_BYTES,
        ORBITRON_EXTRABOLD_BYTES, ORBITRON_MEDIUM_BYTES, ORBITRON_REGULAR_BYTES,
        ORBITRON_SEMIBOLD_BYTES, RALEWAY_BLACK_BYTES, RALEWAY_BLACK_ITALIC_BYTES,
        RALEWAY_BOLD_BYTES, RALEWAY_BOLD_ITALIC_BYTES, RALEWAY_EXTRA_BOLD_BYTES,
        RALEWAY_EXTRA_BOLD_ITALIC_BYTES, RALEWAY_EXTRA_LIGHT_BYTES,
        RALEWAY_EXTRA_LIGHT_ITALIC_BYTES, RALEWAY_ITALIC_BYTES, RALEWAY_LIGHT_BYTES,
        RALEWAY_LIGHT_ITALIC_BYTES, RALEWAY_MEDIUM_BYTES, RALEWAY_MEDIUM_ITALIC_BYTES,
        RALEWAY_REGULAR_BYTES, RALEWAY_SEMI_BOLD_BYTES, RALEWAY_SEMI_BOLD_ITALIC_BYTES,
        RALEWAY_THIN_BYTES, RALEWAY_THIN_ITALIC_BYTES,
    },
};

mod gui;
mod utils;

const MORPHIQ_LOWERCASE: &str = "morphiq lume";
const MORPHIQ_TITLECASE: &str = "Morphiq Lume";
const WINDOW_ICON: &[u8] = include_bytes!("../assets/logos/icons/raw/icon.png");

pub fn main() -> iced::Result {
    iced::application(MORPHIQ_TITLECASE, Morphiq::update, Morphiq::view)
        .settings(Settings {
            id: Some(String::from(MORPHIQ_LOWERCASE)),
            fonts: [
                Cow::Borrowed(ICONS_BYTES),
                Cow::Borrowed(ORBITRON_REGULAR_BYTES),
                Cow::Borrowed(ORBITRON_MEDIUM_BYTES),
                Cow::Borrowed(ORBITRON_SEMIBOLD_BYTES),
                Cow::Borrowed(ORBITRON_BLACK_BYTES),
                Cow::Borrowed(ORBITRON_BOLD_BYTES),
                Cow::Borrowed(ORBITRON_EXTRABOLD_BYTES),
                Cow::Borrowed(RALEWAY_REGULAR_BYTES),
                Cow::Borrowed(RALEWAY_BLACK_BYTES),
                Cow::Borrowed(RALEWAY_BLACK_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_BOLD_BYTES),
                Cow::Borrowed(RALEWAY_BOLD_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_EXTRA_BOLD_BYTES),
                Cow::Borrowed(RALEWAY_EXTRA_BOLD_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_EXTRA_LIGHT_BYTES),
                Cow::Borrowed(RALEWAY_EXTRA_LIGHT_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_LIGHT_BYTES),
                Cow::Borrowed(RALEWAY_LIGHT_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_MEDIUM_BYTES),
                Cow::Borrowed(RALEWAY_MEDIUM_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_SEMI_BOLD_BYTES),
                Cow::Borrowed(RALEWAY_SEMI_BOLD_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_THIN_BYTES),
                Cow::Borrowed(RALEWAY_THIN_ITALIC_BYTES),
            ]
            .to_vec(),
            default_font: Font::with_name(FONT_FAMILY_NAME),
            default_text_size: Pixels(FONT_SIZE_BODY),
            antialiasing: true,
        })
        .theme(Morphiq::theme)
        .centered()
        .run_with(move || (Morphiq::new(), Task::none()))
}
