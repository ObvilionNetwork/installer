use iced::{button, Color, Text, Vector};

use crate::assets;
use crate::colors::*;

pub enum AccentButtonStyle {
    Default,
    Destructive,
}

impl button::StyleSheet for AccentButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::default(),
            background: match self {
                Self::Default => ACCENT_TRANSPARENT.into(),
                Self::Destructive => GRAY_TRANSPARENT.into(),
            },
            border_radius: 9.0,
            border_width: 2.0,
            border_color: match self {
                Self::Default => ACCENT_COLOR,
                Self::Destructive => LIGHT_TRANSPARENT,
            },
            text_color: Color::WHITE,
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            border_color: match self {
                Self::Default => LIGHT_COLOR,
                Self::Destructive => DANGER_COLOR,
            },
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: match self {
                Self::Default => GRAY_TRANSPARENT.into(),
                Self::Destructive => DANGER_TRANSPARENT.into(),
            },
            ..self.hovered()
        }
    }
}

pub struct ImageButtonStyle;

impl button::StyleSheet for ImageButtonStyle {
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
        self.color(Color::WHITE).size(48)
    }

    fn style_description(self) -> Self {
        self.color(GRAY_COLOR).size(24).font(assets::NOTO_LIGHT)
    }
}
