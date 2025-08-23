use iced::window;

use crate::gui::{
    styles::types::style_type::StyleType,
    views::{
        loginview::LoginViewMessage,
        mainview::{MainViewMessages, MainViews},
        types::{running_view::RunningView, settings_view::SettingsView},
    },
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum Message {
    /// Run task to initialize app
    StartApp(Option<window::Id>),
    /// Sent by the backend to get the data to display
    TickDashboard,
    /// Change application Style
    Style(StyleType),
    /// Deserialize a style from a path
    LoadStyle(String),
    /// Opens the specific settings view
    OpenSettings(SettingsView),
    /// Open the last opened settings view
    OpenLastSettings,
    /// Hides the current settings view
    CloseSettings,
    /// Set notification volume
    ChangeVolume(u8),
    /// Save the configuration of the app and quit
    Quit,
    /// Recieves login view messages
    LoginViewMessage(LoginViewMessage),
    /// Change the current running view
    ChangeRunningView(RunningView),
    /// Change the main view
    ChangeMainView(MainViews),
    /// Recieves home view messages
    MainViewMessage(MainViewMessages),
}
