mod panes;

use iced::{
    widget::{container, Column, Container, Row},
    Padding,
};

use crate::gui::{
    morphiq::Morphiq,
    types::message::Message,
    views::home::panes::{
        employee_status::employee_status, employee_tops::employee_tops,
        employee_tracker::employee_tracker,
    },
};

pub fn dashboard_view<'a>(_morphiq: &'a Morphiq) -> Container<'a, Message> {
    let content_1 = Row::new()
        .push(employee_tracker(_morphiq))
        .push(employee_tops(_morphiq))
        .spacing(15);
    let content_2 = Row::new().push(employee_status(_morphiq)).spacing(15);
    let content = Column::new().push(content_1).push(content_2).spacing(15);
    container(content).padding(Padding::from(10.0))
}
