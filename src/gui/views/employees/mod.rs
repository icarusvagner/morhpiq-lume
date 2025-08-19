use iced::{
    widget::{container, Column, Container, Text},
    Padding,
};

use crate::gui::{morphiq::Morphiq, types::message::Message};

pub fn employees_view<'a>(morphiq: &'a Morphiq) -> Container<'a, Message> {
    let content = Column::new().push(Text::new("This is the Employees view").size(42));

    container(content).padding(Padding::from(10.0))
}
