use iced::widget::{container, text, Container};

use crate::gui::{morphiq::Morphiq, types::message::Message};

pub fn main_layout(morphiq: &Morphiq) -> Container<'_, Message> {
    container(text("Main layout").size(32))
}
