use iced::{Settings, Sandbox, window};

mod app;
use app::App;

mod style;

pub fn main() -> iced::Result {
    App::run(Settings {
        default_font: Some(include_bytes!("../assets/noto-sans.ttf")),
        antialiasing: true,
        window: window::Settings {
            size: (820, 450),
            min_size: None, // TODO: add
            icon: None, // TODO: add
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
