use iced::{
    widget::{Container, Row, Text},
    Alignment, Length, Padding,
};

use crate::gui::{
    components::types::status::Statuses, styles::style_constant::OUTFIT_SEMIBOLD,
    types::message::Message,
};

pub mod employee_status;
pub mod employee_tops;
pub mod employee_tracker;

pub fn display_status<'a>(status: &'a Statuses) -> Container<'a, Message> {
    let content = Row::new()
        .push(
            Text::new(status.to_string())
                .size(18)
                .font(OUTFIT_SEMIBOLD)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .align_y(Alignment::Center);

    Container::new(content)
        .padding(Padding::from([5, 15]))
        .width(Length::Fixed(150.0))
        .style(status.rounded_container_style())
}
