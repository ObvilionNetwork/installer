use iced::{button, Vector, Color, Text};

use crate::assets;

const MAIN_COLOR: Color = Color { r: 73.0 / 255.0, g: 159.0 / 255.0, b: 251.0 / 255.0, a: 1.0 };
const GRAY_COLOR: Color = Color { r: 183.0 / 255.0, g: 183.0 / 255.0, b: 183.0 / 255.0, a: 1.0 };
const LIGHT_COLOR: Color = Color { r: 200.0 / 255.0, g: 200.0 / 255.0, b: 200.0 / 255.0, a: 1.0 };

pub struct InstallationButtonStyle;

impl button::StyleSheet for InstallationButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::default(),
            background: Color { a: 0.3, ..MAIN_COLOR }.into(),
            border_radius: 9.0,
            border_width: 2.0,
            border_color: MAIN_COLOR,
            text_color: Color::WHITE,
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            border_color: LIGHT_COLOR,
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Color { a: 0.2, ..GRAY_COLOR }.into(),
            ..self.hovered()
        }
    }
}

pub struct SettingsButtonStyle;

impl button::StyleSheet for SettingsButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::default(),
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: Color::TRANSPARENT,
        }
    }
}

pub trait TextStyles {
    fn style_heading(self) -> Self;
    fn style_description(self) -> Self;
}

impl TextStyles for Text {
    fn style_heading(self) -> Text {
        self
            .color(Color::WHITE)
            .size(48)
    }

    fn style_description(self) -> Self {
        self
            .color(GRAY_COLOR)
            .size(24)
            .font(assets::NOTO_LIGHT)
    }
}
