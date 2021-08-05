use crate::gui::PerfEvent;
/// States of all panes within the pane grid
// every pane state must be held here
use iced::{button, pick_list, scrollable, text_input};
use serde::{Deserialize, Serialize};

use crate::gui::*;

pub struct Content {
    pub input_value: String,
    pub input: text_input::State,
    pub selected_command: PerfEvent,
    pub scroll: scrollable::State,
    pub pick_list: pick_list::State<PerfEvent>,
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
            selected_command: PerfEvent::default(),
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

impl Options {
    pub fn get_options(&self) -> String {
        let mut res = String::new();

        if self.cycles == true {
            res.push_str(" --event cycles");
        }
        if self.instructions == true {
            res.push_str(" --event instructions");
        }
        res.push(' ');

        res
    }
}
