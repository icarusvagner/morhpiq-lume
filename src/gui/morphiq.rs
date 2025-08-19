use std::sync::Arc;

use iced::{
    widget::container,
    window::{self, Id},
    Element, Length, Task, Theme,
};

use crate::gui::{
    types::{
        login::{LoginMessage, LoginView},
        message::{self, Error, Message},
    },
    views::{ layouts::MainLayout, RunningView},
};

#[derive(Clone)]
pub struct Morphiq {
    pub running_view: RunningView,
    pub theme: Theme,
    pub id: Option<window::Id>,
    pub login_view: LoginView,
    pub pass_secure: bool,
}

pub const ICON_FONT_FAMILY_NAME: &str = "Icons for Morphiq Lume";
pub const FONT_FAMILY_NAME: &str = "Outfit";
pub const SVG_FULLLOGO_BYTES: &[u8] = include_bytes!("../../assets/logos/icons/full/icon_full.svg");
pub const SVG_EMBLEMLOGO_BYTES: &[u8] = include_bytes!("../../assets/logos/icons/icon_macros.svg");

impl Morphiq {
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    pub fn new() -> Self {
        Self {
            running_view: RunningView::Login,
            theme: Theme::Light,
            id: None,
            login_view: LoginView::default(),
            pass_secure: true,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Logout => {
                self.running_view = RunningView::Login;
            }
            Message::ToggleShowPwd(is_pwd) => {
                self.pass_secure = is_pwd;
            }
            Message::ChangeRunningPage(running_view) => {
                eprintln!("{:?}", running_view);
                self.running_view = running_view;
            }
            Message::ChangeHomeView(inside_view) => {
                self.running_view = RunningView::Home(inside_view);
            }
            Message::LoginView(message) => {
                self.login_view.update(message.clone());

                if let LoginMessage::LoginSubmit = message.clone() && !self.login_view.username.is_empty() && !self.login_view.password.is_empty(){
                    self.running_view = RunningView::Home(crate::gui::views::InsideView::Dashboard);
                }
            },
        }

        Task::none()
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let _window_id = self.id.unwrap_or_else(Id::unique);
        let content = match &self.running_view {
            RunningView::Login => LoginView::view(&self.login_view, self),
            RunningView::Home(inside_view) => MainLayout::view(self, inside_view),
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
