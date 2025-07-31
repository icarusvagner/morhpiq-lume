use iced::widget::{container, text, Container};

use crate::gui::{morphiq::Morphiq, types::message::Message};

pub fn home_view(_morphiq: &Morphiq) -> Container<'_, Message> {
    container(text("Home view").size(32))
}
