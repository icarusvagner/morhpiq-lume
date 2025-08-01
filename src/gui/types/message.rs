use crate::gui::{
    types::login::LoginMessage,
    views::{InsideView, RunningView},
};

#[derive(Clone, Debug)]
pub enum Message {
    ChangeRunningPage(RunningView),
    ChangeHomeView(InsideView),
    LoginMessage(LoginMessage),
    ToggleShowPwd(bool),
    Logout,
}
