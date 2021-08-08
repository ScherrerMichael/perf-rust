pub mod panes {

    use iced::{
        executor, pane_grid,
        widget::{
            Button, Checkbox, Column, Container, PaneGrid, PickList, Rule, Scrollable, Space, Text,
            TextInput,
        },
        Align, Application, Clipboard, Command, Element, Length, Settings,
    };

    use crate::gui::events::perf::PerfEvent;
    use crate::gui::messages::main::Message;
    use crate::gui::state::pane;
    use crate::gui::state::pane::Context;
    use crate::gui::state::pane::PaneType;
    use crate::gui::style;

    // pub fn panes(panes_state: Content)
    pub fn new<'a>(panes_state: &mut pane_grid::State<pane::Content>) -> PaneGrid<Message> {
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
                                        list.into(),
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
