#[derive(Clone, Debug)]
pub enum LoginMessage {
    LoginSubmit,
    InputFieldChange(String, String),
}

#[derive(Debug, Clone, Default)]
pub struct LoginField {
    pub username: String,
    pub password: String,
}
