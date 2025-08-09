use iced::{
    widget::{
        canvas::{self, Frame, Geometry, Path, Text},
        Canvas,
    },
    Color, Font, Length, Point, Rectangle, Renderer, Size, Theme,
};

use crate::gui::components::charts::bars::stacked::vertical::StackedBar;

pub struct HorizontalStackedBarChart {
    data: Vec<StackedBar>,
    font: Font,
    bar_height: f64,
    bar_spacing: f64,
}

impl HorizontalStackedBarChart {
    pub fn new(data: Vec<StackedBar>, font: Font) -> Self {
        Self {
            data,
            font,
            bar_height: 24.0,
            bar_spacing: 20.0,
        }
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
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let total_width = bounds.width;
        let mut y_offset: f64 = 0.0;
        let left_padding = 13.0;
        let usable_width = bounds.width - left_padding;

        let max_total: f64 = self
            .data
            .iter()
            .map(|row| row.segments.iter().map(|s| s.value).sum::<f64>())
            .fold(0.0, f64::max);

        for stacked_bar in &self.data {
            // 1. Draw label
            let label_text = Text {
                content: stacked_bar.label.clone(),
                position: Point::new(0.0, y_offset as f32),
                color: Color::from_rgb8(100, 100, 100),
                size: 16.into(),
                font: self.font,
                horizontal_alignment: iced::alignment::Horizontal::Left,
                vertical_alignment: iced::alignment::Vertical::Top,
                ..Default::default()
            };
            frame.fill_text(label_text);

            y_offset += 20.0;

            // 2. Draw each segment in the stacked bar
            let mut x_offset = 0.0;
            for segment in &stacked_bar.segments {
                let segment_width = total_width * (segment.value as f32 / max_total as f32);
                let rect = Path::rectangle(
                    Point::new(x_offset, y_offset as f32),
                    Size::new(segment_width, self.bar_height as f32),
                );
                frame.fill(&rect, segment.color);
                x_offset += segment_width;
            }

            y_offset += self.bar_height + self.bar_spacing;
        }

        // 3. Draw the X-axis scale
        let scale_y = y_offset + 20.0;
        let step_percentage = 25;
        for i in 0..=100 {
            if i % step_percentage == 0 {
                let x = left_padding + usable_width * (i as f32 / 110.0);
                let label = Text {
                    content: format!("{}%", i),
                    position: Point::new(x, scale_y as f32),
                    color: Color::from_rgb8(100, 100, 100),
                    size: 16.into(),
                    font: self.font,
                    horizontal_alignment: iced::alignment::Horizontal::Center,
                    vertical_alignment: iced::alignment::Vertical::Top,
                    ..Default::default()
                };
                frame.fill_text(label);
            }
        }

        vec![frame.into_geometry()]
    }
}

pub fn horizontal_stacked_bar<Message>(
    data: Vec<StackedBar>,
    font: Font,
) -> Canvas<HorizontalStackedBarChart, Message, Theme, Renderer> {
    iced::widget::canvas(HorizontalStackedBarChart::new(data, font))
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(400.0 * 2.0))
}
