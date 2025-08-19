use std::f32::consts;

use iced::{
    advanced::mouse,
    alignment::{Horizontal, Vertical},
    widget::{
        canvas::{self, path::Arc, Frame, Text},
        Canvas,
    },
    Color, Length, Radians, Rectangle, Renderer, Theme,
};

use crate::gui::styles::style_constant::{Colors, OUTFIT_MEDIUM};

pub struct DonutChart {
    pub male: u128,
    pub female: u128,
    pub other: u128,
}

impl DonutChart {
    fn new(male: u128, female: u128, other: u128) -> Self {
        Self {
            male,
            female,
            other,
        }
    }

    fn total(&self) -> u128 {
        self.male + self.female + self.other
    }

    fn title(&self) -> String {
        self.total().to_string()
    }

    fn angles(&self) -> [(Radians, Radians); 3] {
        #[allow(clippy::cast_precision_loss)]
        let mut values = [self.male as f32, self.female as f32, self.other as f32];
        let total: f32 = values.iter().sum();
        let min_val = 2.0 * total / 100.0;
        let mut diff = 0.0;

        for value in &mut values {
            if *value > 0.0 && *value < min_val {
                diff += min_val - *value;
                *value = min_val;
            }
        }

        if diff > 0.0 {
            let _ = values
                .iter_mut()
                .max_by(|a, b| a.total_cmp(b))
                .map(|max| *max -= diff);
        }

        let mut start_angle = Radians(-consts::FRAC_PI_2);
        values.map(|value| {
            let start = start_angle;
            let end = start + Radians(consts::TAU) * value / total;
            start_angle = end;
            (start, end)
        })
    }
}

impl<Message> canvas::Program<Message, Theme> for DonutChart {
    type State = ();

    fn draw(
        &self,
        (): &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = frame.center();
        let radius = (frame.width().min(frame.height()) / 2.0) * 0.9;

        let colors = [
            Colors::DarkPastelGreen.get(),
            Colors::Pumpkin.get(),
            Colors::Silver.get(),
        ];

        for ((start_angle, end_angle), color) in self.angles().into_iter().zip(colors) {
            let path = canvas::Path::new(|builder| {
                builder.arc(Arc {
                    center,
                    radius,
                    start_angle,
                    end_angle,
                });
                builder.line_to(center);
                builder.close();
            });

            frame.fill(&path, color);
        }

        let inner_circle = canvas::Path::circle(center, radius - 15.0);
        frame.fill(&inner_circle, Color::WHITE);
        frame.fill_text(Text {
            content: format!("{} Total Genders", self.title().clone()),
            position: center,
            vertical_alignment: Vertical::Center,
            horizontal_alignment: Horizontal::Center,
            color: Colors::Night.get(),
            size: 20.3.into(),
            font: OUTFIT_MEDIUM,
            ..Default::default()
        });

        vec![frame.into_geometry()]
    }
}

pub fn donut_chart<Message>(
    male: u128,
    female: u128,
    other: u128,
) -> Canvas<DonutChart, Message, Theme> {
    iced::widget::canvas(DonutChart::new(male, female, other))
        .width(Length::Fill)
        .height(Length::Fill)
}
