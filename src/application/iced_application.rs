use iced::{
    widget::{button, checkbox, column, text, Column},
    Task,
};

use iced_aw;

use crate::{auto_start::auto_start_agent::initialize_auto_start, sorting::filesort::run_sorting};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ApplicationState {
    #[default]
    Main,
    LoadingApp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Message {
    Init,
    StartSorting,
    ToggleAutoStart(bool),
    ToggleTimedSort(bool),
}

#[derive(Default)]
pub struct RustyFilesortApplication {
    application_state: ApplicationState,
    is_sorting: bool,
    auto_start: bool,
    timed_sorting: bool,
    chosen_theme: iced::Theme,
}

impl RustyFilesortApplication {
    pub fn view(&self) -> Column<Message> {
        match self.application_state {
            ApplicationState::Main => {
                let on_press = if self.is_sorting {
                    None
                } else {
                    Some(Message::StartSorting)
                };
                let auto_start =
                    checkbox("Automatically start the App with Windows", self.auto_start)
                        .on_toggle(Message::ToggleAutoStart);
                let timed_sort_available = Some(Message::ToggleTimedSort);
                let timed_sorting = checkbox(
                    "Automatically Sort Download Dir on specific time",
                    self.timed_sorting,
                )
                .on_toggle_maybe(timed_sort_available);
                return column![
                    button("Start sorting").on_press_maybe(on_press),
                    auto_start,
                    timed_sorting
                ];
            }
            ApplicationState::LoadingApp => {
                column![text!("Loading")]
            }
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::StartSorting => {
                tracing::info!("starting filesort");
                self.is_sorting = true;
                match run_sorting() {
                    Ok(_) => {
                        tracing::info!("Sorted successfully");
                    }
                    Err(error) => {
                        tracing::error!("Something went wrong while sorting {}", error);
                    }
                }
                self.is_sorting = false
            }
            Message::ToggleAutoStart(auto_start) => {
                self.auto_start = !self.auto_start;
                let auto_start_agent = initialize_auto_start();
                if auto_start_agent.is_some() {
                    if auto_start {
                        auto_start_agent.unwrap().enable().unwrap(); //TODO: NEED TO REMOVE UNWRAPS
                    } else {
                        auto_start_agent.unwrap().disable().unwrap(); //TODO: NEED TO REMOVE UNWRAPS
                    }
                }
            }

            Message::Init => {
                self.application_state = ApplicationState::Main;
            }
            Message::ToggleTimedSort(_) => {}
        }
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::TokyoNight
    }

    pub fn new() -> (RustyFilesortApplication, Task<Message>) {
        let auto_start_agent = initialize_auto_start();
        let mut auto_start = false;
        if auto_start_agent.is_some() {
            auto_start = auto_start_agent.unwrap().is_enabled().unwrap(); // TODO: NEED TO REMOVE UNWRAPS
                                                                          // TODO: NEED TO ADD ERROR DISPLAY LOGIC
        }
        (
            RustyFilesortApplication {
                application_state: ApplicationState::LoadingApp,
                is_sorting: false,
                auto_start: auto_start,
                timed_sorting: false, //TODO: SAVE SETTINGS
                chosen_theme: iced::Theme::Nightfly,
            },
            Task::done(Message::Init),
        )
    }
}
