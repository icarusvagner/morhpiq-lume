use iced::{
    widget::container,
    window::{self, Id},
    Element, Length, Task, Theme,
};

use crate::gui::{
    types::{
        login::{LoginField, LoginMessage},
        message::Message,
    },
    views::{layouts::main_layout, login_view::login_view, InsideView, RunningView},
};

pub struct Morphiq {
    pub running_view: RunningView,
    pub theme: Theme,
    pub id: Option<window::Id>,
    pub login_field: LoginField,
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
            login_field: LoginField::default(),
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
                self.running_view = running_view;
            }
            Message::ChangeHomeView(inside_view) => {
                self.running_view = RunningView::Home(inside_view);
            }
            Message::LoginMessage(login) => match login {
                LoginMessage::InputFieldChange(username, password) => {
                    self.login_field = LoginField { username, password }
                }
                LoginMessage::LoginSubmit => {
                    if !self.login_field.username.is_empty()
                        && !self.login_field.password.is_empty()
                    {
                        self.running_view = RunningView::Home(InsideView::Dashboard);
                    }
                }
            },
        }

        Task::none()
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let _window_id = self.id.unwrap_or_else(Id::unique);
        let content = match &self.running_view {
            RunningView::Login => login_view(self),
            RunningView::Home(inside_view) => main_layout(self, inside_view),
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
