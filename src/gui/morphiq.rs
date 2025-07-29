use iced::{widget::container, window, Alignment, Element, Length, Task, Theme};

use crate::gui::{
    types::message::Message,
    views::{dashboard_view::dashboard_view, login_view::login_view, RunningView},
};

pub struct Morphiq {
    pub running_view: RunningView,
    pub theme: Theme,
    pub id: Option<window::Id>,
}

impl Morphiq {
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    pub fn new() -> Self {
        Self {
            running_view: RunningView::Login,
            theme: Theme::Light,
            id: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeRunningPage(running_view) => {
                self.running_view = running_view;
            }
        }
        Task::none()
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let content = match self.running_view {
            RunningView::Login => login_view(self),
            RunningView::Dashboard => dashboard_view(self),
        };

        container(content)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
