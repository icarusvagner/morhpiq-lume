use iced::{
    widget::{Column, Container, Text},
    Padding,
};

use crate::gui::{
    components::types::status::Statuses, styles::style_constant::OUTFIT_SEMIBOLD,
    types::message::Message,
};

pub mod employee_status;
pub mod employee_tops;
pub mod employee_tracker;

pub fn display_status<'a>(status: &'a Statuses) -> Container<'a, Message> {
    let content = Column::new().push(Text::new(status.to_string()).size(18).font(OUTFIT_SEMIBOLD));

    Container::new(content)
        .padding(Padding::from([5, 15]))
        .style(status.rounded_container_style())
}
