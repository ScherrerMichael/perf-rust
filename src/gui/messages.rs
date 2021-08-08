pub mod main {
    use crate::gui::events::perf::PerfEvent;
    use crate::gui::state::*;
    use iced::pane_grid;
    /// Messages to be sent to the parent widget from
    /// other child widgets, and consumed on update
    #[derive(Debug, Clone)]
    pub enum Message {
        Loaded(Result<save_load::SavedState, save_load::LoadError>),
        Saved(Result<(), save_load::SaveError>),
        InputChanged(String),
        NewAppPressed,
        Resized(pane_grid::ResizeEvent),
        CommandSelected(PerfEvent),
        CyclesToggled(bool),
        InstructionsToggled(bool),
        JsonToggled(bool),
        ListToggled(bool),
        VerboseToggled(bool),
        LaunchCommand,
    }
}
