#[derive(Clone, Debug)]
pub enum LoginMessage {
    ClearPressed,
    LoginSubmit,
    UsernameChange(String),
    PasswordChange(String),
}

#[derive(Debug, Clone, Default)]
pub struct LoginView {
    pub username: String,
    pub password: String,
}

impl LoginView {
    pub fn update(&mut self, message: LoginMessage) {
        match message {
            LoginMessage::ClearPressed => {
                self.username = String::new();
                self.password = String::new();
            }
            LoginMessage::UsernameChange(value) => {
                if !value.is_empty() {
                    self.username = value;
                }
            }
            LoginMessage::PasswordChange(value) => {
                if !value.is_empty() {
                    self.password = value;
                }
            }
            LoginMessage::LoginSubmit => {}
        }
    }
}
