pub mod dashboard_view;
pub mod login_view;

#[derive(Debug, Clone)]
pub enum RunningView {
    Login,
    Dashboard,
}
