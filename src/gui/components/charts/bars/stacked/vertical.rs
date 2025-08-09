use iced::{
    advanced::mouse,
    widget::{
        canvas::{self, Frame, Geometry, Path},
        Canvas,
    },
    Color, Font, Length, Point, Rectangle, Renderer, Size, Theme,
};

#[derive(Debug, Clone)]
pub struct BarSegment {
    pub value: f64,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct StackedBar {
    pub label: String,
    pub segments: Vec<BarSegment>,
}

#[derive(Debug, Clone)]
pub struct DonutData {
    pub label: String,
    pub segments: BarSegment,
}

pub struct VerticalStackedBarChart {
    data: Vec<StackedBar>,
    font: Font,
}

impl VerticalStackedBarChart {
    pub fn new(data: Vec<StackedBar>, font: Font) -> Self {
        Self { data, font }
    }
}

impl<Message, Theme> canvas::Program<Message, Theme> for VerticalStackedBarChart {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(_renderer, bounds.size());

        let padding = 20.0;
        let label_height = 30.0;
        let title_height = 40.0;

        let chart_area = Size {
            width: bounds.width - 2.0 * padding,
            height: bounds.height - padding - label_height - title_height,
        };

        let max_total = self
            .data
            .iter()
            .map(|bar| bar.segments.iter().map(|seg| seg.value).sum::<f64>())
            .fold(0.0, f64::max);

        let bar_count = self.data.len();
        let bar_width = chart_area.width / (bar_count as f32 * 1.5); // include spacing
        let bar_spacing = bar_width / 2.0;

        // --- Draw bars ---
        for (i, bar) in self.data.iter().enumerate() {
            let x = padding + i as f32 * (bar_width + bar_spacing);
            let mut y = bounds.height - label_height - padding; // Start drawing from bottom up

            for segment in &bar.segments {
                let height_ratio = segment.value / max_total;
                let height = chart_area.height * height_ratio as f32;

                let rect = Path::rectangle(Point::new(x, y - height), Size::new(bar_width, height));
                frame.fill(&rect, segment.color);

                y -= height;
            }

            // --- Draw centered label ---
            frame.fill_text(canvas::Text {
                content: bar.label.clone(),
                position: Point::new(x + bar_width / 2.0, bounds.height - label_height + 5.0),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Top,
                font: self.font,
                size: 18.into(),
                color: Color::from_rgb8(100, 100, 100),
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }
}

pub fn vertical_stacked_bar<Message>(
    data: Vec<StackedBar>,
    font: Font,
) -> Canvas<VerticalStackedBarChart, Message, Theme, Renderer> {
    iced::widget::canvas(VerticalStackedBarChart::new(data, font))
        .width(Length::Fill)
        .height(Length::Fixed(400.0 * 2.0))
}
