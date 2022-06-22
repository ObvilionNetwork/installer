use iced::{button, checkbox, text_input, Color, Text, Vector};

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
            text_color: CHECKBOX_TEXT,
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
            text_color: Color::TRANSPARENT
        }
    }
}

pub struct CheckboxStyle;

impl checkbox::StyleSheet for CheckboxStyle {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: CHECKBOX.into(),
            checkmark_color: Color::BLACK,
            border_radius: 4.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: Color::WHITE.into(),
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        self.active(is_checked)
    }
}

pub struct TextInputStyle;

impl text_input::StyleSheet for TextInputStyle {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Color::TRANSPARENT.into(),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: TEXT_INPUT_BORDER
        }
    }

    fn focused(&self) -> text_input::Style {
        self.hovered()
    }

    fn placeholder_color(&self) -> Color {
        GRAY_COLOR
    }

    fn value_color(&self) -> Color {
        TEXT_INPUT_COLOR
    }

    fn selection_color(&self) -> Color {
        GRAY_TRANSPARENT
    }

    fn hovered(&self) -> text_input::Style {
        text_input::Style {
            border_color: LIGHT_COLOR,
            ..self.active()
        }
    }
}

pub trait TextStyles {
    fn style_heading(self) -> Self;
    fn style_description(self) -> Self;
    fn style_description_highlight(self) -> Self;
    fn style_heading2(self) -> Self;
}

impl TextStyles for Text {
    fn style_heading(self) -> Text {
        self.color(Color::WHITE).size(48)
    }

    fn style_description(self) -> Self {
        self.color(GRAY_COLOR).size(24).font(assets::NOTO_LIGHT)
    }

    fn style_description_highlight(self) -> Self {
        self.color(Color::WHITE).size(24).font(assets::NOTO_LIGHT)
    }

    fn style_heading2(self) -> Self {
        self.color(HEADING2_COLOR).size(20)
    }
}
