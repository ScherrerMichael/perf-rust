//! Gui driver
use iced::{
    executor, pane_grid,
    widget::{Column, Container, Text},
    Align, Application, Clipboard, Command, Element, Length, Settings,
};

mod events;
mod messages;
mod state;
mod style;
mod widgets;

use events::perf::PerfEvent;
use messages::{main::Message, task::TaskMessage};
use state::{
    main::State,
    pane::{Content, Context},
    save_load::SavedState,
};
use widgets::{panes, task};

/// Run the Gui Launcher
pub fn run_gui() -> iced::Result {
    Gui::run(Settings::default())
}

/// Main States for all Gui elements
enum Gui {
    Loading,
    Loaded(State),
}

/// Provide methods for Gui renderer
impl Application for Gui {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    /// Initialize state of Gui parent element
    fn new(_flags: ()) -> (Gui, Command<Self::Message>) {
        (
            Gui::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }
    /// Set title for Gui parent element
    fn title(&self) -> String {
        String::from("Ruperf")
    }
    /// Update Gui based on recieved Message flags
    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match self {
            // Update Loading consumed for Gui
            // then changed to loaded based on
            // Loading function
            Gui::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        //tasks are loaded here
                        *self = Gui::Loaded(State {
                            tasks: state.tasks,
                            ..State::default()
                        });
                    }
                    // When load file is not found
                    // set state to default
                    Message::Loaded(Err(_)) => {
                        *self = Gui::Loaded(State::default());
                    }

                    _ => {}
                }

                Command::none()
            }

            // When Gui is loaded prepare to recieve message
            // callbacks from children widgets
            Gui::Loaded(state) => {
                let mut saved = false;

                let mut data_state = state.panes_state.get_mut(&state.data_pane).unwrap();

                match message {
                    Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                        if state.horz_split == split {
                            // println!("horizontal split");
                        } else {
                            // println!("vertical split");
                        }

                        state.panes_state.resize(&split, ratio);
                        // println!("split: {:?}, ratio: {}",split, ratio);
                    }

                    Message::NewAppPressed => {
                        data_state.context = Context::NewEvent;
                        println!("new app pressed");
                    }

                    Message::TaskMessage(i, TaskMessage::Run) => {
                        run_program(&state.tasks[i], data_state).expect("error");
                    }

                    Message::CommandSelected(PerfEvent::Stat) => {
                        data_state.selected_command = PerfEvent::Stat;
                        println!("stat selected")
                    }
                    Message::CommandSelected(PerfEvent::Record) => {
                        data_state.selected_command = PerfEvent::Record;
                        println!("record selected")
                    }
                    Message::CommandSelected(PerfEvent::Report) => {
                        data_state.selected_command = PerfEvent::Report;
                        println!("report selected")
                    }
                    Message::CommandSelected(PerfEvent::Annotate) => {
                        data_state.selected_command = PerfEvent::Annotate;
                        println!("annotate selected")
                    }
                    Message::CommandSelected(PerfEvent::Top) => {
                        data_state.selected_command = PerfEvent::Top;
                        println!("top selected")
                    }
                    Message::CommandSelected(PerfEvent::Bench) => {
                        data_state.selected_command = PerfEvent::Bench;
                        println!("bench selected")
                    }
                    Message::CommandSelected(PerfEvent::Test) => {
                        data_state.selected_command = PerfEvent::Test;
                        println!("test selected")
                    }

                    // Stat Options
                    Message::CyclesToggled(value) => {
                        data_state.launch_options.cycles = value;
                    }

                    Message::InstructionsToggled(value) => {
                        data_state.launch_options.instructions = value;
                    }

                    // Test Options
                    Message::JsonToggled(value) => {
                        data_state.launch_options.json = value;
                    }

                    Message::ListToggled(value) => {
                        data_state.launch_options.list = value;
                    }

                    Message::VerboseToggled(value) => {
                        data_state.launch_options.verbose = value;
                    }

                    Message::InputChanged(value) => {
                        data_state.input_value = value;
                    }

                    Message::LaunchCommand => {
                        match data_state.selected_command {
                            PerfEvent::Stat => {
                                let task = task::Task::new(
                                    Some(PerfEvent::Stat),
                                    Some(data_state.get_options().to_string()),
                                    Some(data_state.input_value.to_string()),
                                );

                                match task {
                                    Ok(t) => {
                                        run_program(&t, data_state).expect("error");
                                        state.tasks.push(t);
                                    }
                                    Err(s) => {
                                        println!("Error: {}", s);
                                    }
                                }
                            }
                            PerfEvent::Record => {
                                //TODO: Add program here
                                data_state.data = format!("Record output:");
                            }
                            PerfEvent::Report => {
                                //TODO: Add program here
                                data_state.data = format!("Report output:");
                            }
                            PerfEvent::Annotate => {
                                //TODO: Add program here
                                data_state.data = format!("Annotate output:");
                            }
                            PerfEvent::Top => {
                                //TODO: Add program here
                                data_state.data = format!("Top output:");
                            }
                            PerfEvent::Bench => {
                                //TODO: Add program here
                                data_state.data = format!("Bench output:");
                            }
                            PerfEvent::Test => {
                                let task = task::Task::new(
                                    Some(PerfEvent::Test),
                                    Some(data_state.get_options().to_string()),
                                    None,
                                );

                                match task {
                                    Ok(t) => {
                                        run_program(&t, data_state).expect("error");
                                        state.tasks.push(t);
                                    }
                                    Err(s) => {
                                        println!("Error: {}", s);
                                    }
                                }
                            }
                        }

                        // Switch data panel to main view,
                        // and PerfEvent output
                        data_state.context = Context::Main;
                    }

                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;
                    }

                    _ => {}
                }

                if !saved {
                    state.dirty = true;
                }

                if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            tasks: state.tasks.clone(),
                        }
                        .save(),
                        Message::Saved,
                    )
                } else {
                    Command::none()
                }
            }
        }
    }
    /// Display Graphics to screen
    fn view(&mut self) -> Element<Self::Message> {
        match self {
            Gui::Loading => loading_message(),
            Gui::Loaded(State {
                panes_state,
                tasks,
                task_pane,
                ..
            }) => {
                let task_pane = panes_state.get_mut(&task_pane).unwrap();

                task_pane.tasks = tasks.to_vec();

                //panes in the main application
                let panes = panes::new(panes_state);

                // Collect all panes and add them to main Gui element
                let content = Column::new()
                    .spacing(5)
                    .padding(5)
                    .width(Length::Fill)
                    .align_items(Align::Center)
                    .push(Text::new("test"))
                    .push(panes);

                // Display all widget elements
                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(style::widget::Container {})
                    .into()
            }
        }
    }
}

/// Message to display while Gui is loading
fn loading_message<'a>() -> Element<'a, Message> {
    Container::new(Text::new("Loading...").size(50))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
}

#[derive(Debug, Clone)]
/// Error type for save function
pub enum ProgramError {
    PerfEvent,
    Program(String),
}

fn run_program(task: &task::Task, data_state: &mut Content) -> Result<(), ProgramError> {
    use std::process::Command;
    use std::str;

    //create another process, in this case run another perf-rust
    //with command: test
    let run_command = &task.command;

    // run_command.push_str("stat");
    // run_command.push_str(data_state.get_options().as_str());
    // run_command.push_str(" ");
    // run_command.push_str(data_state.input_value.as_str());

    // run_command.push_str("test");
    // run_command.push_str(data_state.get_options().as_str());

    println!("splitted: {:?}", run_command);

    let output = Command::new("./ruperf")
        .args(task.command.split(" "))
        .output()
        .expect("failed to execute process");

    // Create buffer variable
    let buf = &output.stdout;

    // Convert &vec[u8] into string
    let s = match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // save task data
    // saveState.tasks.push(task);

    //output to data pane
    data_state.data = s.to_string();

    println!("output: {}", s);

    Ok(())
}
