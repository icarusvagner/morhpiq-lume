pub mod dashboard_view;
pub mod login_view;
pub mod home;
pub mod layouts;

#[derive(Debug, Clone)]
pub enum RunningView {
    Login,
    Home,
}
