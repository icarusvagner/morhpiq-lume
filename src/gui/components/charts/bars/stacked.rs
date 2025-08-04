use iced::{
    widget::{
        canvas::{Frame, Geometry, Program},
        Canvas,
    },
    Element, Length, Point, Rectangle, Renderer, Size, Theme,
};

use crate::gui::types::message::Message;

#[derive(Debug, Clone)]
pub struct Segment {
    pub height: f32,        // value (proportional)
    pub color: iced::Color, // segment color
}

#[derive(Debug, Clone)]
pub struct Bar {
    pub segments: Vec<Segment>, // stacked segments
    pub label: Option<String>,  // optional label below bar
}

#[derive(Debug, Clone)]
pub struct StackedBarChart {
    bars: Vec<Bar>,
    max_value: f32,
    width: f32,
    height: f32,
}

impl StackedBarChart {
    pub fn new(bars: Vec<Bar>) -> Self {
        let max_value = bars
            .iter()
            .map(|bar| bar.segments.iter().map(|s| s.height).sum::<f32>())
            .fold(0.0, f32::max);

        Self {
            bars,
            max_value,
            width: 400.0,
            height: 300.0,
        }
    }

    pub fn chart_view<'a>(&'a self) -> Element<'a, Message> {
        Canvas::new(self.clone())
            .width(Length::Fixed(self.width))
            .height(Length::Fixed(self.height))
            .into()
    }
}

impl<Message> Program<Message, Theme, Renderer> for StackedBarChart {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        _bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, Size::new(self.width, self.height));
        let bar_width = (self.width - 40.0) / self.bars.len() as f32;
        let spacing = 8.0;

        for (i, bar) in self.bars.iter().enumerate() {
            let x = 20.0 + i as f32 * (bar_width + spacing);
            let mut y = self.height - 20.0;

            for segment in &bar.segments {
                let height = (segment.height / self.max_value) * (self.height - 40.0);
                let rect = Rectangle {
                    x,
                    y: y - height,
                    width: bar_width,
                    height,
                };

                frame.fill_rectangle(
                    Point::new(rect.x, rect.y),
                    Size::new(rect.width, rect.height),
                    segment.color,
                );

                y -= height;
            }
        }

        vec![frame.into_geometry()]
    }
}
