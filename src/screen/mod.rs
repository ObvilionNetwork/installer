use crate::app::{Message, Context};

pub mod greeting;
pub mod settings;
pub mod installation;
pub mod finish;

#[derive(Debug, Clone)]
pub enum Screen {
    Greeting(greeting::State),
    Settings(settings::State),
    Installation(installation::State),
    Finish(finish::State)
}

impl Screen {
    pub fn show(&mut self, context: &Context) -> iced::Element<Message> {
        match self {
            Self::Greeting(state) => greeting::show(context, state),
            Self::Settings(state) => settings::show(context, state),
            Self::Installation(state) => installation::show(context, state),
            Self::Finish(state) => finish::show(context, state),
        }
    }

    pub fn greeting() -> Self {
        Self::Greeting(greeting::State::default())
    }

    pub fn settings() -> Self {
        Self::Settings(settings::State::default())
    }

    pub fn installation() -> Self {
        Self::Installation(installation::State::default())
    }

    pub fn finish() -> Self {
        Self::Finish(finish::State::default())
    }
}
