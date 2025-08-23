use crate::gui::views::mainview::MainViews;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RunningView {
    LoginView,
    MainView(MainViews),
}

impl RunningView {
    pub const ALL: [RunningView; 2] = [
        RunningView::LoginView,
        RunningView::MainView(MainViews::Dashboard),
    ];
}
