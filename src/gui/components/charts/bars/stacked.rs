use iced::{
    widget::canvas::{Cache, Frame, Geometry},
    Element, Length, Size,
};
use plotters::prelude::*;
use plotters::style::Color;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend, Renderer};

use crate::gui::{components::charts::bars::CategoryData, types::message::Message};

pub struct StackedBarChart {
    width: Length,
    height: Length,
    cache: Cache,
    data: Vec<CategoryData>,
}

impl StackedBarChart {
    pub fn new(data: Vec<CategoryData>) -> Self {
        Self {
            height: Length::Fill,
            width: Length::Fill,
            cache: Cache::new(),
            data,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        ChartWidget::new(self)
            .width(self.width)
            .height(self.height)
            .into()
    }
}

impl Chart<Message> for StackedBarChart {
    type State = ();

    fn draw<R: Renderer, F: Fn(&mut Frame)>(
        &self,
        renderer: &R,
        bounds: Size,
        draw_fn: F,
    ) -> Geometry {
        renderer.draw_cache(&self.cache, bounds, draw_fn)
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut chart: ChartBuilder<DB>) {
        let categories: Vec<&str> = self.data.iter().map(|c| c.label.as_str()).collect();
        let category_count: f32 = self.data.len() as f32;

        let max_value: f32 = self
            .data
            .iter()
            .map(|c| c.segments.iter().map(|(v, _)| *v).sum::<f32>())
            .fold(0.0, f32::max)
            .max(1.0);

        let mut chart = chart
            .margin(10)
            .x_label_area_size(40)
            .set_left_and_bottom_label_area_size(35)
            .build_cartesian_2d(0.0..category_count, 0.0..max_value)
            .expect("Failed to build chart");

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_axis()
            .disable_y_mesh()
            .x_labels(categories.len())
            .x_label_offset(150)
            .x_label_formatter(&|i| {
                let idx = i.round() as usize;
                categories.get(idx).cloned().unwrap_or("").to_string()
            })
            .x_label_style(
                ("Orbitron", 14)
                    .into_font()
                    .color(&plotters::style::colors::WHITE),
            )
            .x_labels(10)
            .draw()
            .unwrap();

        // Bar width and spacing
        let bar_width = 0.7;

        // Draw each bar segment using Rectangle
        for (i, category) in self.data.iter().enumerate() {
            let x_center = i as f32 + 0.15;
            let x_left = x_center - bar_width / 2.0;
            let x_right = x_center + bar_width / 2.0;

            let mut y_offset: f32 = 0.0;

            for (value, color) in &category.segments {
                let rect = Rectangle::new(
                    [(x_left, y_offset), (x_right, y_offset + *value)],
                    color.filled(),
                );

                chart
                    .draw_series(std::iter::once(rect))
                    .expect("Failed to draw segment");

                y_offset += *value;
            }
        }
    }
}
