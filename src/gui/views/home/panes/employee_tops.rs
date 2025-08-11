use iced::{
    widget::{container, horizontal_rule, Column, Container, Text},
    Length,
};

use crate::gui::{
    components::charts::bars::stacked::{
        horizontal::horizontal_stacked_bar,
        vertical::{BarSegment, StackedBar},
    },
    morphiq::Morphiq,
    styles::{
        container::ContainerStyle,
        style_constant::{Colors, OUTFIT_BOLD, OUTFIT_MEDIUM},
    },
    types::message::Message,
};

pub fn employee_tops(_morphiq: &Morphiq) -> Container<'_, Message> {
    let data = [
        StackedBar {
            label: "Lance Phillip".to_string(),
            segments: vec![
                BarSegment {
                    value: 40.0,
                    color: Colors::MayaBlue.get(),
                },
                BarSegment {
                    value: 25.0,
                    color: Colors::UTOrange.get(),
                },
                BarSegment {
                    value: 15.0,
                    color: Colors::SelectiveYellow.get(),
                },
            ],
        },
        StackedBar {
            label: "Christian".to_string(),
            segments: vec![
                BarSegment {
                    value: 30.0,
                    color: Colors::MayaBlue.get(),
                },
                BarSegment {
                    value: 23.0,
                    color: Colors::UTOrange.get(),
                },
                BarSegment {
                    value: 10.0,
                    color: Colors::SelectiveYellow.get(),
                },
            ],
        },
        StackedBar {
            label: "Bert".to_string(),
            segments: vec![
                BarSegment {
                    value: 23.0,
                    color: Colors::MayaBlue.get(),
                },
                BarSegment {
                    value: 13.0,
                    color: Colors::UTOrange.get(),
                },
                BarSegment {
                    value: 8.0,
                    color: Colors::SelectiveYellow.get(),
                },
            ],
        },
    ]
    .to_vec();

    let content = Column::new()
        .push(Text::new("Top 3 Employees by Performance").size(24))
        .push(horizontal_rule(2.0))
        .push(horizontal_stacked_bar(data, OUTFIT_MEDIUM))
        .spacing(15);

    container(content)
        .padding(15.0)
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(400.0))
        .style(ContainerStyle::Rounded.appearance())
}
