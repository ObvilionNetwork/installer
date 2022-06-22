use iced::window::{self, Icon};
use iced::{Sandbox, Settings};

mod app;
use app::App;

mod assets;

mod colors;

mod style;

pub fn main() -> iced::Result {
    App::run(Settings {
        default_font: Some(assets::NOTO_REGULAR_BYTES),
        antialiasing: true,
        window: window::Settings {
            size: (820, 450),
            min_size: Some((665, 356)),
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
