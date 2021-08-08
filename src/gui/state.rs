pub mod main {
    use super::pane::*;
    use iced::pane_grid::{Pane, Split, self};

    /// State for Gui
    pub struct State {
        pub tasks: Vec<Task>,
        pub panes_state: pane_grid::State<Content>,
        pub panes_created: usize,
        pub data_pane: Pane,
        pub log_pane: Pane,
        pub task_pane: Pane,
        pub vert_split: Split,
        pub horz_split: Split,
    }

    /// Default state for Gui
    impl Default for State {
        fn default() -> Self {
            // First pane and first state is created here:
            // task Pane, panes_state
            let (mut panes_state, task_pane) = pane_grid::State::new(Content::new(PaneType::Task));

            // Second pane and first split is created here:
            // data_pane, vert_split
            let (data_pane, vert_split) = panes_state
                .split(
                    pane_grid::Axis::Vertical,
                    &task_pane,
                    Content::new(PaneType::Main),
                )
                .unwrap();

            // Third plane and second split is created here:
            // log_pane, horz_split
            let (log_pane, horz_split) = panes_state
                .split(
                    pane_grid::Axis::Horizontal,
                    &data_pane,
                    Content::new(PaneType::Log),
                )
                .unwrap();

            panes_state.resize(&vert_split, 0.17);
            panes_state.resize(&horz_split, 0.88);

            let tasks = Vec::new();

            State {
                tasks,
                panes_state,
                panes_created: 3,
                data_pane,
                task_pane,
                log_pane,
                vert_split,
                horz_split,
            }
        }
    }
}

pub mod pane {

    use crate::gui::events::*;

    /// States of all panes within the pane grid
    // every pane state must be held here
    use iced::{button, pick_list, scrollable, text_input};
    use serde::{Deserialize, Serialize};

    pub struct Content {
        pub input_value: String,
        pub input: text_input::State,
        pub selected_command: perf::PerfEvent,
        pub scroll: scrollable::State,
        pub pick_list: pick_list::State<perf::PerfEvent>,
        pub data: String,
        pub log: String,
        pub application: String,
        pub pane_type: PaneType,
        pub create_button: button::State,
        pub launch_button: button::State,
        pub context: Context,
        pub launch_options: Options,
    }

    /// Initialize pane states to default values
    impl Content {
        pub fn new(pane_type: PaneType) -> Self {
            Content {
                input_value: String::new(),
                input: text_input::State::new(),
                selected_command: perf::PerfEvent::default(),
                scroll: scrollable::State::new(),
                pick_list: pick_list::State::default(),
                pane_type,
                data: String::new(),
                log: String::new(),
                create_button: button::State::new(),
                launch_button: button::State::new(),
                application: String::new(),
                context: Context::Main,
                launch_options: Options::default(),
            }
        }

        pub fn get_options(&self) -> String {
            let mut res = String::new();

            match self.selected_command {
                perf::PerfEvent::Stat => {
                    if self.launch_options.cycles == true {
                        res.push_str(" --event cycles");
                    }
                    if self.launch_options.instructions == true {
                        res.push_str(" --event instructions");
                    }
                }

                perf::PerfEvent::Test => {
                    if self.launch_options.json == true {
                        res.push_str(" --json");
                    } else if self.launch_options.list == true {
                        res.push_str(" --list");
                    } else if self.launch_options.verbose == true {
                        res.push_str(" --verbose");
                    }
                }

                _ => {
                    //nothing for now
                }
            }

            res
        }
    }

    /// Main pane Contexts
    pub enum Context {
        Main,
        NewEvent,
    }

    /// Pane Type
    pub enum PaneType {
        Task,
        Main,
        Log,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    // Currently running or previously ran events
    pub struct Task {
        name: String,
        application: String,
        options: Vec<String>,
    }

    pub struct Options {
        pub cycles: bool,
        pub instructions: bool,
        pub json: bool,
        pub list: bool,
        pub verbose: bool,
    }

    impl Default for Options {
        fn default() -> Self {
            Options {
                cycles: false,
                instructions: false,
                json: false,
                list: false,
                verbose: false,
            }
        }
    }
}

pub mod save_load {
    use super::pane::Task;
    use serde::{Deserialize, Serialize};

    //customized from iced todo example.
    // source: https://github.com/hecrj/iced/blob/0.3/examples/todos/src/main.rs

    //Persistance
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SavedState {
        pub tasks: Vec<Task>,
    }

    #[derive(Debug, Clone)]
    /// Error type for load function
    pub enum LoadError {
        FileError,
        FormatError,
    }

    #[derive(Debug, Clone)]
    /// Error type for save function
    pub enum SaveError {
        FileError,
        WriteError,
        FormatError,
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Saved state for Gui
    impl SavedState {
        fn path() -> std::path::PathBuf {
            let mut path = if let Some(project_dirs) =
                directories_next::ProjectDirs::from("rs", "ruperf", "Tasks")
            {
                project_dirs.data_dir().into()
            } else {
                std::env::current_dir().unwrap_or(std::path::PathBuf::new())
            };

            path.push("tasks.json");

            path
        }

        pub async fn load() -> Result<SavedState, LoadError> {
            use async_std::prelude::*;

            let mut contents = String::new();

            let mut file = async_std::fs::File::open(Self::path())
                .await
                .map_err(|_| LoadError::FileError)?;

            file.read_to_string(&mut contents)
                .await
                .map_err(|_| LoadError::FileError)?;

            serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)
        }

        pub async fn save(self) -> Result<(), SaveError> {
            use async_std::prelude::*;

            let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::FormatError)?;

            let path = Self::path();

            if let Some(dir) = path.parent() {
                async_std::fs::create_dir_all(dir)
                    .await
                    .map_err(|_| SaveError::FileError)?;
            }

            {
                let mut file = async_std::fs::File::create(path)
                    .await
                    .map_err(|_| SaveError::FileError)?;

                file.write_all(json.as_bytes())
                    .await
                    .map_err(|_| SaveError::WriteError)?;
            }

            // This is a simple way to save at most once every couple seconds
            async_std::task::sleep(std::time::Duration::from_secs(2)).await;

            Ok(())
        }
    }

    #[cfg(target_arch = "wasm32")]
    // Saved state for Gui (wasm32)
    impl SavedState {
        fn storage() -> Option<web_sys::Storage> {
            let window = web_sys::window()?;

            window.local_storage().ok()?
        }

        pub async fn load() -> Result<SavedState, LoadError> {
            let storage = Self::storage().ok_or(LoadError::FileError)?;

            let contents = storage
                .get_item("state")
                .map_err(|_| LoadError::FileError)?
                .ok_or(LoadError::FileError)?;

            serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)
        }

        async fn save(self) -> Result<(), SaveError> {
            let storage = Self::storage().ok_or(SaveError::FileError)?;

            let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::FormatError)?;

            storage
                .set_item("state", &json)
                .map_err(|_| SaveError::WriteError)?;

            let _ = wasm_timer::Delay::new(std::time::Duration::from_secs(2)).await;

            Ok(())
        }
    }
}
