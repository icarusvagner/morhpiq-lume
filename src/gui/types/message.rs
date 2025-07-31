use crate::gui::{types::login::LoginMessage, views::RunningView};

#[derive(Clone, Debug)]
pub enum Message {
    ChangeRunningPage(RunningView),
    LoginMessage(LoginMessage),
    ToggleShowPwd(bool),
}
