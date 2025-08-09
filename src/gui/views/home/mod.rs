mod panes;

use iced::{
    widget::{container, Column, Container, Row},
    Padding,
};

use crate::gui::{
    morphiq::Morphiq,
    types::message::Message,
    views::home::panes::{
        employee_status::employee_status, employee_tops::employee_tops,
        employee_tracker::employee_tracker,
    },
};

pub fn dashboard_view<'a>(_morphiq: &'a Morphiq) -> Container<'a, Message> {
    let content_1 = Row::new()
        .push(employee_tracker(_morphiq))
        .push(employee_tops(_morphiq))
        .spacing(15);
    let content_2 = Row::new().push(employee_status(_morphiq)).spacing(15);
    let content = Column::new().push(content_1).push(content_2).spacing(15);
    container(content).padding(Padding::from(10.0))
}

#[derive(Debug, Clone)]
pub struct DropdownState {
    pub employee_tracker: Option<EmployeeTrackerChoice>,
    pub attendance_tracker: Option<AttendanceTrackerChoice>,
}

#[derive(Debug, Clone)]
pub struct AttendanceTrackerChoice {
    pub choice: AttendanceTrackerDropdown,
    pub expand: bool,
}

#[derive(Debug, Clone)]
pub struct EmployeeTrackerChoice {
    pub choice: EmployeeTrackerDropdown,
    pub expand: bool,
}

#[derive(Debug, Clone)]
pub enum DropdownChoice {
    EmployeeTracker(EmployeeTrackerDropdown),
    AttendanceTracker(AttendanceTrackerDropdown),
}

#[derive(Default, Clone, Debug)]
pub enum EmployeeTrackerDropdown {
    #[default]
    ThisWeek,
    LastWeek,
    Month,
    Year,
}

pub const EMPLOYEE_TRACKER_CHOICES: [EmployeeTrackerDropdown; 4] = [
    EmployeeTrackerDropdown::ThisWeek,
    EmployeeTrackerDropdown::LastWeek,
    EmployeeTrackerDropdown::Month,
    EmployeeTrackerDropdown::Year,
];

#[derive(Default, Clone, Debug)]
pub enum AttendanceTrackerDropdown {
    #[default]
    Present,
    Leave,
    Absent,
}

pub const ATTENDANCE_CHOICES: [AttendanceTrackerDropdown; 3] = [
    AttendanceTrackerDropdown::Present,
    AttendanceTrackerDropdown::Leave,
    AttendanceTrackerDropdown::Absent,
];

#[derive(Clone, Debug)]
pub enum TrackerStateDD {
    Dismiss,
    Expand,
}

#[derive(Clone, Debug)]
pub enum AttendanceDD {
    Dismiss,
    Expand,
}
