use iced::{
    advanced::mouse,
    widget::{
        canvas::{self, Frame, Path},
        Canvas,
    },
    Font, Length, Point, Rectangle, Renderer, Size, Theme,
};

use crate::gui::{components::charts::bars::stacked::StackedBar, styles::style_constant::Colors};

pub struct HorizontalStackedBarChart {
    data: Vec<StackedBar>,
    font: Font,
    title: String,
}

impl HorizontalStackedBarChart {
    pub fn new(data: Vec<StackedBar>, font: Font, title: String) -> Self {
        Self { data, font, title }
    }

    fn title(&self) -> String {
        self.title.clone()
    }
}

impl<Message, Theme> canvas::Program<Message, Theme> for HorizontalStackedBarChart {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let padding = 20.0;
        let label_width = 1.0;
        let bar_gap = 20.0;
        let bar_height = 30.0;

        let chart_top = padding + 40.0;
        let chart_area_width = bounds.width - label_width - 2.0 * padding;

        let max_total = self
            .data
            .iter()
            .map(|bar| bar.segments.iter().map(|s| s.value).sum::<f64>())
            .fold(0.0, f64::max);

        // --- Draw title ---
        // frame.fill_text(canvas::Text {
        //     content: self.title(),
        //     position: Point::new(bounds.width / 2.0, padding),
        //     horizontal_alignment: iced::alignment::Horizontal::Right,
        //     vertical_alignment: iced::alignment::Vertical::Top,
        //     font: self.font,
        //     size: 28.into(),
        //     color: Colors::Night.get(),
        //     ..Default::default()
        // });

        for (i, bar) in self.data.iter().enumerate() {
            let y = chart_top + i as f32 * (bar_height + bar_gap);
            let mut x = padding + label_width;

            // --- Draw label
            frame.fill_text(canvas::Text {
                content: bar.label.clone(),
                position: Point::new(padding + label_width - 10.0, bar_height / 2.0),
                size: 16.into(),
                font: self.font,
                horizontal_alignment: iced::alignment::Horizontal::Right,
                vertical_alignment: iced::alignment::Vertical::Center,
                color: Colors::Night.get(),
                ..Default::default()
            });

            // --- Draw segment with rounded edges
            for segment in &bar.segments {
                let segment_width = (segment.value / max_total) as f32 * chart_area_width;

                let rect = if segment_width > 0.0 {
                    Path::rectangle(Point::new(x, y), Size::new(segment_width, bar_height))
                } else {
                    continue;
                };

                frame.fill(&rect, segment.color);

                frame.fill_text(canvas::Text {
                    content: String::from((segment_width as f64 / max_total) * 100.0),
                    position: Point::new(padding + label_width - 2.0, segment_width / 2.0),
                });

                x += segment_width;
            }
        }

        vec![frame.into_geometry()]
    }
}

pub fn horizontal_stacked_bar<Message>(
    data: Vec<StackedBar>,
    font: Font,
    title: String,
) -> Canvas<HorizontalStackedBarChart, Message, Theme, Renderer> {
    iced::widget::canvas(HorizontalStackedBarChart::new(data, font, title))
        .width(Length::Fill)
        .height(Length::Fill)
}
