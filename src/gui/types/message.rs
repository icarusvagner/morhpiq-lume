use crate::gui::{
    morphiq::Db,
    types::login::LoginMessage,
    views::{home::DropdownChoice, InsideView, RunningView},
};

#[derive(Clone, Debug)]
pub enum Message {
    ChangeRunningPage(RunningView),
    ChangeHomeView(InsideView),
    LoginMessage(LoginMessage),
    ToggleShowPwd(bool),
    Logout,
    Loaded(Result<Db>),
    DropDown(DropdownChoice),
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("DB Error: {0}")]
    DatabaseConnectionError(String),
    #[error("Failed to load env: {0}")]
    FailedToLoadEnv(String),
}
