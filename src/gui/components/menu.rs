use iced::{
    widget::{
        container, horizontal_rule, svg::Handle, text, vertical_space, Column, Container, Row, Svg,
    },
    Alignment, Length,
};

use crate::{
    gui::{morphiq::SVG_FULLLOGO_BYTES, types::message::Message},
    utils::icons::Icon,
};

pub fn sidebar_menu() -> Container<'static, Message> {
    let profile = Row::new().push(
        Icon::User
            .to_text()
            .size(32)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    );
    let logo_menu = Row::new().push(
        Svg::new(Handle::from_memory(SVG_FULLLOGO_BYTES))
            .height(Length::Fixed(50.0))
            .width(Length::Fill),
    );
    let content = Column::new()
        .push(logo_menu)
        .push(horizontal_rule(2.0))
        .push(vertical_space())
        .push(horizontal_rule(2.0))
        .push(profile)
        .spacing(15.0)
        .padding(10.0);

    container(content).width(310.0)
}
