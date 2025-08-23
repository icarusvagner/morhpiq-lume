use iced::{
    widget::{container},
    window, Element, Length, Task,
};

use crate::{
    configs::configs::Configs,
    gui::{
        styles::{container::ContainerType, types::{
            custom_palette::{CustomPalette, ExtraStyles},
            palette::Palette,
            style_type::StyleType,
        }},
        types::message::Message,
        views::{
            loginview::{LoginView, LoginViewMessage}, mainview::layout::MainLayout, types::running_view::RunningView
        },
    },
};

pub const ICON_FONT_FAMILY_NAME: &str = "Icons for Morphiq Lume";
pub const FONT_FAMILY_NAME: &str = "Outfit";
pub const SVG_FULLLOGO_BYTES: &[u8] = include_bytes!("../../assets/logos/icons/full/icon_full.svg");
pub const SVG_EMBLEMLOGO_BYTES: &[u8] = include_bytes!("../../assets/logos/icons/icon_macros.svg");

#[derive(Debug, Clone)]
pub struct Morphiq {
    /// Application's configurations: settings and more to come
    pub configs: Configs,
    /// Defines the current running view
    pub running_view: RunningView,
    /// Window ID
    pub id: Option<window::Id>,
    /// Login view events
    pub login_view: LoginView,
}

impl Morphiq {
    pub fn new(configs: Configs) -> Self {
        Self {
            configs,
            running_view: RunningView::LoginView,
            id: None,
            login_view: LoginView::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StartApp(id) => {
                self.id = id;

                // Performs some task
            }
            Message::TickDashboard => {}
            Message::Style(style) => {
                self.configs.settings.style = style;
            }
            Message::LoadStyle(path) => {
                self.configs.settings.style_path.clone_from(&path);
                if let Ok(palette) = Palette::from_file(&path) {
                    let style = StyleType::Custom(ExtraStyles::CustomToml(
                        CustomPalette::from_palette(palette),
                    ));

                    self.configs.settings.style = style;
                }
            }
            Message::OpenSettings(_) => {}
            Message::OpenLastSettings => {}
            Message::CloseSettings => {}
            Message::ChangeVolume(_) => {}
            Message::Quit => {
                let _ = self.configs.clone().store();
                return window::close(self.id.unwrap_or_else(window::Id::unique));
            }
            Message::LoginViewMessage(mssg) => {
                self.login_view.update(mssg.clone());

                if let LoginViewMessage::LoginSubmitted = mssg.clone() && !self.login_view.username.is_empty() && !self.login_view.password.is_empty()
                {
                    self.running_view =
                        RunningView::MainView(super::views::mainview::MainViews::Dashboard);
                } else if let LoginViewMessage::Logout = mssg.clone() {
                    self.running_view = RunningView::LoginView;
                }
            }
            Message::ChangeRunningView(running_view) => {
                self.running_view = running_view;
            }
            Message::MainViewMessage(mssg) => {}
            Message::ChangeMainView(main_view) => {
            self.running_view = RunningView::MainView(main_view);
        }
        }
        Task::none()
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message, StyleType> {
        let content = match &self.running_view {
            RunningView::LoginView => LoginView::view(&self.login_view, self),
            RunningView::MainView(main_view) => MainLayout::view(main_view)
        };

        container(content)
            .class(ContainerType::Base300)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    pub fn theme(&self) -> StyleType {
        self.configs.settings.style
    }
}
