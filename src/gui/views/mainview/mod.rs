use crate::gui::views::mainview::layout::LayoutMessage;

mod attendance;
mod dashboard;
mod documents;
mod edit_profile;
mod employee;
pub mod layout;
mod leaves;
mod panes;
mod payroll;
mod settings;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum MainViews {
    Dashboard,
    Employee,
    Attendance,
    Payroll,
    Leaves,
    Documents,
    Settings,
    EditProfile,
}

impl MainViews {
    pub const ALL: [MainViews; 8] = [
        MainViews::Dashboard,
        MainViews::Employee,
        MainViews::Attendance,
        MainViews::Payroll,
        MainViews::Leaves,
        MainViews::Documents,
        MainViews::Settings,
        MainViews::EditProfile,
    ];
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum MainViewMessages {
    Layout(LayoutMessage),
    Main,
}
