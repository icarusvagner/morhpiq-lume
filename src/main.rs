use iced::Task;

use crate::gui::morphiq::Morphiq;

mod gui;
mod utils;

const MORPHIQ_TITLECASE: &str = "Morphiq Lume";

pub fn main() -> iced::Result {
    iced::application(MORPHIQ_TITLECASE, Morphiq::update, Morphiq::view)
        .theme(Morphiq::theme)
        .centered()
        .run_with(move || (Morphiq::new(), Task::none()))
}
