use crate::gui::{
    types::login::LoginMessage,
    views::{InsideView, RunningView},
};

#[derive(Clone, Debug)]
pub enum Message {
    ChangeRunningPage(RunningView),
    ChangeHomeView(InsideView),
    LoginView(LoginMessage),
    ToggleShowPwd(bool),
    Logout,
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("DB Error: {0}")]
    DatabaseConnectionError(String),
    #[error("Failed to load env: {0}")]
    FailedToLoadEnv(String),
}
