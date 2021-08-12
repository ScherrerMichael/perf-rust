pub mod panes {

    use iced::{
        pane_grid, Element,
        widget::{
            Button, Checkbox, Column, Container, PaneGrid, PickList, Rule, Scrollable, Space, Text,
            TextInput,
        },
        Align, Length,
    };

    use crate::gui::events::perf::PerfEvent;
    use crate::gui::messages::main::Message;
    use crate::gui::state::pane;
    use crate::gui::state::pane::Context;
    use crate::gui::state::pane::PaneType;
    use crate::gui::style;
    use super::task::Task;

    // pub fn panes(panes_state: Content)
    pub fn new<'a>(panes_state: &'a mut pane_grid::State<pane::Content>) -> PaneGrid<'a, Message> {
        let panes = PaneGrid::new(panes_state, |pane, content| {
            let title = Text::new("");

            // Title of pane
            let title_bar = pane_grid::TitleBar::new(title).padding(10);

            // Initialize event list of elements
            let event_list = PickList::new(
                &mut content.event_list,
                &PerfEvent::ALL[..],
                Some(content.selected_command),
                Message::CommandSelected,
            );

            // fn loading_message<'a>() -> Element<'a, Message> {
             let task_list: Element<_> = if content.tasks.iter().count() > 0 {
                 println!("inside tasks ceation!");
                        content.tasks
                        .iter_mut()
                        .enumerate()
                        .fold(Column::new().spacing(20), |column, (i, task)| {
                            column.push(task.view().map(move |message| {
                                Message::TaskMessage(i, message)
                            }))
                        })
                        .into()
                } else {
                    println!("no tasks found :(, number found: {}", content.tasks.iter().count());
                    Column::new().into()
                };

            // Initialize scrollable list of elements
            let scrollable_list = Scrollable::new(&mut content.scroll)
                .height(Length::Fill)
                .width(Length::Fill)
                .align_items(Align::Start)
                .spacing(10);

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
                        .push( scrollable_list.push(Column::with_children(vec![
                            
                            Button::new(&mut content.create_button, Text::new("new"))
                                .style(style::widget::Button {})
                                .on_press(Message::NewAppPressed)
                                .width(Length::FillPortion(100)).into(),
                                task_list,
                        ]))
                ))
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(5),

                // data_pane will switch visual context based on outside events:
                // Main: view data of running event (default)
                // NewEvent: generate menu for creating events
                PaneType::Main => match content.context {
                    Context::Main => Container::new(
                        scrollable_list
                            .push(Text::new(&content.data).color(style::widget::TEXT_COLOR)),
                    ),

                    Context::NewEvent => Container::new(
                        Column::new()
                            .spacing(5)
                            .padding(5)
                            .width(Length::Fill)
                            .align_items(Align::Center)
                            .push(
                                scrollable_list.push(
                                    Column::with_children(vec![
                                        Text::new("Select a program to run")
                                            .color(style::widget::TEXT_COLOR)
                                            .into(),
                                        event_list.into(),
                                        Rule::horizontal(100).into(),
                                        // Space::new(Length::Fill, Length::from(100)).into(),
                                        {
                                            match content.selected_command {
                                                PerfEvent::Stat => Column::with_children(vec![
                                                    Text::new("Program to run:")
                                                        .color(style::widget::TEXT_COLOR)
                                                        .into(),
                                                    input.into(),
                                                    Rule::horizontal(100).into(),
                                                ])
                                                .into(),

                                                _ => Container::new(Column::with_children(vec![]))
                                                    .into(),
                                            }
                                        },
                                        Text::new("Options:")
                                            .color(style::widget::TEXT_COLOR)
                                            .into(),
                                        {
                                            //these are the options for each individual event selected:
                                            match content.selected_command {
                                                PerfEvent::Stat => {
                                                    Container::new(Column::with_children(vec![
                                                        Checkbox::new(
                                                            content.launch_options.cycles,
                                                            "Cycles",
                                                            Message::CyclesToggled,
                                                        )
                                                        .into(),
                                                        Space::new(Length::Fill, Length::from(10))
                                                            .into(),
                                                        Checkbox::new(
                                                            content.launch_options.instructions,
                                                            "Instructions",
                                                            Message::InstructionsToggled,
                                                        )
                                                        .into(),
                                                    ]))
                                                    .into()
                                                }
                                                PerfEvent::Test => {
                                                    Container::new(Column::with_children(vec![
                                                        Checkbox::new(
                                                            content.launch_options.json,
                                                            "Json",
                                                            Message::JsonToggled,
                                                        )
                                                        .into(),
                                                        Space::new(Length::Fill, Length::from(10))
                                                            .into(),
                                                        Checkbox::new(
                                                            content.launch_options.list,
                                                            "List",
                                                            Message::ListToggled,
                                                        )
                                                        .into(),
                                                        Space::new(Length::Fill, Length::from(10))
                                                            .into(),
                                                        Checkbox::new(
                                                            content.launch_options.verbose,
                                                            "Verbose",
                                                            Message::VerboseToggled,
                                                        )
                                                        .into(),
                                                    ]))
                                                    .into()
                                                }

                                                _ => Container::new(Column::with_children(vec![]))
                                                    .into(),
                                            }
                                        },
                                        Rule::horizontal(100).into(),
                                        // Space::new(Length::Fill, Length::from(100)).into(),
                                        Button::new(
                                            &mut content.launch_button,
                                            Text::new("Launch"),
                                        )
                                        .on_press(Message::LaunchCommand)
                                        .style(style::widget::Button {})
                                        .into(),
                                    ])
                                    .padding(20),
                                ),
                            ),
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
        });

        panes
    }
}

pub mod task {
    use serde::{Deserialize, Serialize};
    use crate::gui::state::task::TaskState;
    use crate::gui::messages::task::TaskMessage;
    use crate::gui::events::perf;
    use iced::{Element, widget::{button, Text}};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    // Currently running or previously ran events
    pub struct Task {
        event: perf::PerfEvent,
        program: String,
        options: String,
        pub command: String,
        
        #[serde(skip)]
        state: TaskState,
    }

    impl Task{
        pub fn new(event: Option<perf::PerfEvent>, options: Option<String>, program: Option<String>)
         -> Result<Task, &'static str> {
            let mut command = String::new();
            let mut task_event = perf::PerfEvent::default();
            let mut task_options = String::new();
            let mut task_program = String::new();
            match event {
                Some(res) => {
                    command.push_str(res.as_str());
                    task_event = res;
                }
                None => {
                    return Err("No event.")
                }
            }
            match options {
                Some(res) => {
                    command.push_str(res.as_str());
                    if task_event == perf::PerfEvent::Stat{
                    command.push_str(" ");
                    }
                    task_options = res;
                }
                None => {
                    return Err("No options.")
                }
            }
            match program {
                Some(res) => {
                    command.push_str(res.as_str());
                    task_program = res.to_string();
                }
                None => {}
            }

            Ok(
                Task{
                    event: task_event,
                    options: task_options.to_string(),
                    program: task_program.to_string(),
                    command: command,

                    state: TaskState::Idle {
                        edit_button: button::State::new(),
                    }
                }
            )
        }

        fn update(&mut self, message: TaskMessage) {
            match message {
                TaskMessage::Edit => {
                    println!("editing task")
                }
                TaskMessage::Delete => {
                    println!("deleting task")
                }
                TaskMessage::Run => {
                    println!("running task")
                }

            }
        }

        pub fn view(&mut self) -> Element<TaskMessage> {
            match &mut self.state{
                TaskState::Idle {edit_button} => {
                    Text::new("testing..").into()
                }
            }
        }
    }

    impl Default for Task{
        fn default() -> Self {
            Task{
                event: perf::PerfEvent::default(),
                program: String::default(),
                options: String::default(),
                command: String::default(),
                state: TaskState::Idle {
                    edit_button: button::State::new(),
                }
            }
        }
    }

    /// Provide Tasks as String data types
    impl std::fmt::Display for Task {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"({}), {}", self.event, self.program)
        }
    }
}
