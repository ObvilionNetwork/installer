use iced::{Sandbox, Settings};
use iced::window::{self, Icon};

mod screen;
mod app;
mod assets;
mod colors;
mod style;
mod installer;

use crate::app::App;

pub fn main() -> iced::Result {
    App::run(Settings {
        default_font: Some(assets::NOTO_REGULAR_BYTES),
        antialiasing: true,
        window: window::Settings {
            size: (820, 455),
            min_size: None,
            icon: Some(Icon::from_rgba(
                assets::ICON_RGB.to_vec(),
                assets::ICON_SIZE.0,
                assets::ICON_SIZE.1,
            ).unwrap()),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
