
use iced::widget::{container, text, Container};

use crate::gui::{morphiq::Morphiq, types::message::Message};

pub fn employee_status(_morphiq: &Morphiq) -> Container<'_, Message> {
    container(text("Employee status").size(32))
}
