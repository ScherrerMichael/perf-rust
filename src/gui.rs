//! Gui driver
use iced::{
    executor, pane_grid,
    widget::{
        Button, Checkbox, Column, Container, PaneGrid, PickList, Rule, Scrollable, Space, Text,
        TextInput,
    },
    Align, Application, Clipboard, Command, Element, Length, Settings,
};

mod pane_state;
mod perf_event;
mod save_state;
mod state;
mod style;

use pane_state::*;
use perf_event::*;
use save_state::*;
use state::*;

/// Run the Gui Launcher
pub fn run_gui() -> iced::Result {
    Gui::run(Settings::default())
}

/// Main States for all Gui elements
enum Gui {
    Loading,
    Loaded(State),
}

/// Messages to be sent to the parent widget from
/// other child widgets, and consumed on update
#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
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
    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match self {
            // Update Loading consumed for Gui
            // then changed to loaded based on
            // Loading function
            Gui::Loading => match message {
                Message::Loaded(Ok(state)) => {
                    *self = Gui::Loaded(State {
                        tasks: state.tasks,
                        ..State::default()
                    })
                }
                // When load file is not found
                // set state to default
                Message::Loaded(Err(_)) => {
                    *self = Gui::Loaded(State::default());
                }

                _ => {
                    println!("error")
                }
            },

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
                                //TODO: Add program here
                                run_program(PerfEvent::Stat, data_state)
                            }
                            PerfEvent::Record => {
                                //TODO: Add program here
                                run_program(PerfEvent::Record, data_state)
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
                                //TODO: Add program here
                                run_program(PerfEvent::Test, data_state)

                            }
                        }

                        // Switch data panel to main view,
                        // and PerfEvent output
                        data_state.context = Context::Main;
                    }

                    _ => {
                        println!("other")
                    }
                }
            }
        }
        Command::none()
    }
    /// Display Graphics to screen
    fn view(&mut self) -> Element<Self::Message> {
        match self {
            Gui::Loading => loading_message(),
            Gui::Loaded(State { panes_state, .. }) => {
                // Iterate entire pane grid and display each
                // with thier own content
                let panes = PaneGrid::new(panes_state, |pane, content| {
                    let title = Text::new("");

                    // Title of pane
                    let title_bar = pane_grid::TitleBar::new(title).padding(10);

                    // Initialize list of elements
                    let list = PickList::new(
                        &mut content.pick_list,
                        &PerfEvent::ALL[..],
                        Some(content.selected_command),
                        Message::CommandSelected,
                    );

                    // Initialize scrollable list of elements
                    let scrollable_list = Scrollable::new(&mut content.scroll)
                        .width(Length::Fill)
                        .align_items(Align::Start)
                        .spacing(10)
                        .push(Text::new("Select a program to run")
                            .color(style::widget::TEXT_COLOR))
                        .push(list);

                    // Initialize Input field
                    let input = TextInput::new(
                        &mut content.input,
                        "",
                        &mut content.input_value,
                        Message::InputChanged,
                    )
                    .width(Length::from(200));

                    // Pane main container dependant on the given PaneType:
                    //--------------------------------------------------------------------
                    // Task: previously ran events, or creating new event
                    // Main: main input for event creation, viewing output from ran events
                    // Log:  viewing logs for debug purposes
                    //---------------------------------------------------------------------
                    pane_grid::Content::new(match content.pane_type {
                        // Task pane
                        PaneType::Task => Container::new(
                            Column::new()
                                .spacing(5)
                                .padding(5)
                                .width(Length::Fill)
                                .align_items(Align::Start)
                                .push(
                                    Button::new(&mut content.create_button, Text::new("new"))
                                        .style(style::widget::Button {})
                                        .on_press(Message::NewAppPressed)
                                        .width(Length::FillPortion(100)),
                                ),
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(5),

                        // data_pane will switch visual context based on outside events:
                        // Main: view data of running event (default)
                        // NewEvent: generate menu for creating events
                        PaneType::Main => match content.context {
                            Context::Main => Container::new(
                                Column::new()
                                    .spacing(5)
                                    .padding(5)
                                    .width(Length::Fill)
                                    .align_items(Align::Center)
                                    .push(Text::new(&content.data)
                                    .color(style::widget::TEXT_COLOR)),
                            ),

                            Context::NewEvent => Container::new(
                                Column::new()
                                    .spacing(5)
                                    .padding(5)
                                    .width(Length::Fill)
                                    .align_items(Align::Center)
                                    .push(scrollable_list.push(Column::with_children(vec![
                                        Rule::horizontal(100).into(),
                                        // Space::new(Length::Fill, Length::from(100)).into(),
                                        {
                                            match content.selected_command
                                            {
                                                PerfEvent::Stat => {
                                                    Column::with_children(
                                                        vec![
                                                            Text::new("Program to run:")
                                                            .color(style::widget::TEXT_COLOR)
                                                            .into(),
                                                            input.into(),
                                                            Rule::horizontal(100).into(),
                                                        ]
                                                    ).into()
                                                }

                                                _=> {
                                                    Container::new(
                                                        Column::with_children(
                                                            vec![
                                                            ]
                                                        )
                                                    ).into()
                                                }
                                            }
                                        },
                                        Text::new("Options:")
                                        .color(style::widget::TEXT_COLOR)
                                        .into(),
                                        {
                                            //these are the options for each individual event selected:
                                            match content.selected_command {
                                                PerfEvent::Stat => {
                                                    Container::new(
                                                        Column::with_children(
                                                            vec![
                                                    Checkbox::new(content.launch_options.cycles, "Cycles", Message::CyclesToggled)
                                                    .into(),
                                                    Space::new(Length::Fill, Length::from(10)).into(),
                                                    Checkbox::new(content.launch_options.instructions, "Instructions", Message::InstructionsToggled).into(),
                                                            ]
                                                        )
                                                    ).into()
                                                }
                                                PerfEvent::Test => {
                                                    Container::new(
                                                        Column::with_children(
                                                            vec![
                                                    Checkbox::new(content.launch_options.json, "Json", Message::JsonToggled).into(),
                                                    Space::new(Length::Fill, Length::from(10)).into(),
                                                    Checkbox::new(content.launch_options.list, "List", Message::ListToggled).into(),
                                                    Space::new(Length::Fill, Length::from(10)).into(),
                                                    Checkbox::new(content.launch_options.verbose, "Verbose", Message::VerboseToggled).into(),
                                                            ]
                                                        )
                                                    ).into()
                                                }

                                                _ => {
                                                    Container::new(
                                                        Column::with_children(
                                                            vec![
                                                            ]
                                                        )
                                                    ).into()
                                                }
                                            }
                                        },
                                        Rule::horizontal(100).into(),
                                        // Space::new(Length::Fill, Length::from(100)).into(),
                                        Button::new(
                                            &mut content.launch_button,
                                            Text::new("Launch"),
                                        )
                                        .on_press(Message::LaunchCommand)
                                        .style(style::widget::Button{})
                                        .into(),
                                    ])
                                .padding(20))),
                            ),
                        },
                        // Log pane
                        PaneType::Log => Container::new(
                            Column::new()
                                .spacing(5)
                                .padding(5)
                                .width(Length::Fill)
                                .align_items(Align::Center)
                                .push(Text::new("Logs")),
                        ),
                    })
                    .title_bar(title_bar)
                    .style(style::widget::Pane { is_focused: false })
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .on_resize(10, Message::Resized)
                .spacing(7);

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

fn run_program(event: PerfEvent, mut data_state: &mut Content){
    use std::process::Command;
    use std::str;

    //create another process, in this case run another perf-rust
    //with command: test
    let mut run_command = String::new();

    match event{
        PerfEvent::Stat => {
            run_command.push_str("stat");
            run_command.push_str(data_state.get_options().as_str());
            run_command.push_str(" ");
            run_command.push_str(data_state.input_value.as_str());
        }
        PerfEvent::Record => {
            run_command.push_str("record");
        }
        PerfEvent::Report => {
            run_command.push_str("report");
        }
        PerfEvent::Annotate => {
            run_command.push_str("annotate");
        }
        PerfEvent::Bench => {
            run_command.push_str("bench");
        }
        PerfEvent::Top => {
            run_command.push_str("top");
        }
        PerfEvent::Test => {
            run_command.push_str("test");
            run_command.push_str(data_state.get_options().as_str());
        }
    }


    println!("splitted: {:?}", run_command);

    let output = Command::new("./ruperf")
        .args(run_command.split(' '))
        .output()
        .expect("failed to execute process");

    // Create buffer variable
    let buf = &output.stdout;

    // Convert &vec[u8] into string
    let s = match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    //output to data pane
    data_state.data = s.to_string();

    println!("output: {}", s);
}
