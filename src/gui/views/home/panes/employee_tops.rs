

use iced::widget::{container, text, Container};

use crate::gui::{morphiq::Morphiq, types::message::Message};

pub fn employee_tops(_morphiq: &Morphiq) -> Container<'_, Message> {
    container(text("Employee tops").size(32))
}
