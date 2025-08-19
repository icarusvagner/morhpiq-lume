pub mod dashboard_view;
pub mod employees;
pub mod home;
pub mod layouts;
pub mod login_view;

#[derive(Debug, Clone)]
pub enum RunningView {
    Login,
    Home(InsideView),
}

#[derive(Debug, Clone)]
pub enum InsideView {
    Dashboard,
    Employee,
    Attendance,
    Payroll,
    Leaves,
    Documents,
    Settings,
}
