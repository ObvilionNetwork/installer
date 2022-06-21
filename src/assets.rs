use iced::Font;

pub static LOGO_PNG: &[u8] = include_bytes!("../assets/logo.png");

pub static ICON_RGB: &[u8] = include_bytes!("../assets/icon.rgba");
pub static ICON_SIZE: (u32, u32) = (64, 64);

pub static SETTINGS_SVG: &[u8] = include_bytes!("../assets/settings.svg");

pub static NOTO_REGULAR_BYTES: &[u8] = include_bytes!("../assets/noto-sans.ttf");
pub static NOTO_LIGHT: Font = Font::External { name: "Noto Sans Light", bytes: include_bytes!("../assets/noto-sans-light.ttf") };
