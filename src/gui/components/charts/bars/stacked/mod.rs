pub mod horizontal;
pub mod vertical;

#[derive(Debug, Clone)]
pub struct BarSegment {
    pub value: f64,
    pub color: iced::Color,
}

#[derive(Debug, Clone)]
pub struct StackedBar {
    pub label: String,
    pub segments: Vec<BarSegment>,
}
