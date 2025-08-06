pub mod stacked;

use plotters::style::{RGBColor, BLUE, YELLOW};

#[derive(Clone, Debug)]
pub struct CategoryData {
    pub label: String,
    pub segments: Vec<(f32, RGBColor)>,
}

pub fn demo_data() -> Vec<CategoryData> {
    [
        CategoryData {
            label: "Mon".into(),
            segments: vec![(3.0, YELLOW), (22.0, BLUE)],
        },
        CategoryData {
            label: "Tue".into(),
            segments: vec![(2.0, YELLOW), (23.0, BLUE)],
        },
        CategoryData {
            label: "Wed".into(),
            segments: vec![(0.0, YELLOW), (25.0, BLUE)],
        },
        CategoryData {
            label: "Thu".into(),
            segments: vec![(0.0, YELLOW), (25.0, BLUE)],
        },
        CategoryData {
            label: "Fri".into(),
            segments: vec![(1.0, YELLOW), (24.0, BLUE)],
        },
    ]
    .to_vec()
}
