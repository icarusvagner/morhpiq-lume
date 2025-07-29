use crate::gui::views::RunningView;

#[derive(Clone, Debug)]
pub enum Message {
    ChangeRunningPage(RunningView),
}
