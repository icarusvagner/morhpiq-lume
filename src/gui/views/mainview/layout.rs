use iced::{
    widget::{container, scrollable, Column, Row, Scrollable},
    Element, Length,
};

use crate::gui::{
    components::{
        header::{HeaderMenu, HeaderMenuMessage},
        sidebar::SidebarMenu,
    },
    styles::{container::ContainerType, scrollbar::ScrollbarType, types::style_type::StyleType},
    types::message::Message,
    views::mainview::{
        attendance::AttendanceView, dashboard::DashboardView, documents::DocumentsView,
        edit_profile::EditProfileView, employee::EmployeeView, leaves::LeavesView,
        payroll::PayrollView, settings::SettingsView, MainViews,
    },
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum LayoutMessage {
    Header(HeaderMenuMessage),
}

#[derive(Debug, Clone, Default, Copy)]
pub struct MainLayout;

impl MainLayout {
    pub fn view<'a>(view_content: &'a MainViews) -> Element<'a, Message, StyleType> {
        let body_view_content: Element<'_, Message, StyleType> = Element::from(
            Scrollable::new(Self::to_view(view_content)).direction(
                scrollable::Direction::Vertical(
                    scrollable::Scrollbar::new()
                        .width(0.2)
                        .margin(0.0)
                        .scroller_width(4.0)
                        .anchor(scrollable::Anchor::Start),
                ),
            ),
        );

        let content = Row::new().push(SidebarMenu::view()).push(body_view_content);

        let body_content = Column::new().push(HeaderMenu::view()).push(content);

        container(body_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn to_view<'a>(view_content: &MainViews) -> Element<'a, Message, StyleType> {
        container(match view_content {
            MainViews::Dashboard => DashboardView::view(),
            MainViews::Employee => EmployeeView::view(),
            MainViews::Attendance => AttendanceView::view(),
            MainViews::Payroll => PayrollView::view(),
            MainViews::Leaves => LeavesView::view(),
            MainViews::Documents => DocumentsView::view(),
            MainViews::Settings => SettingsView::view(),
            MainViews::EditProfile => EditProfileView::view(),
        })
        .padding(12)
        .into()
    }
}
