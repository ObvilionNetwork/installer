use iced::Color;

pub const ACCENT_COLOR: Color = Color {
    r: 73.0 / 255.0,
    g: 159.0 / 255.0,
    b: 251.0 / 255.0,
    a: 1.0,
};
pub const ACCENT_TRANSPARENT: Color = Color {
    a: 0.3,
    ..ACCENT_COLOR
};

pub const DANGER_COLOR: Color = Color {
    r: 255.0,
    g: 61.0 / 255.0,
    b: 61.0 / 255.0,
    a: 1.0,
};
pub const DANGER_TRANSPARENT: Color = Color {
    a: 0.3,
    ..DANGER_COLOR
};

pub const GRAY_COLOR: Color = Color {
    r: 183.0 / 255.0,
    g: 183.0 / 255.0,
    b: 183.0 / 255.0,
    a: 1.0,
};
pub const GRAY_TRANSPARENT: Color = Color { a: 0.2, ..GRAY_COLOR };

pub const LIGHT_COLOR: Color = Color {
    r: 200.0 / 255.0,
    g: 200.0 / 255.0,
    b: 200.0 / 255.0,
    a: 1.0,
};
pub const LIGHT_TRANSPARENT: Color = Color {
    a: 0.5,
    ..LIGHT_COLOR
};

pub const CHECKBOX: Color = Color {
    a: 0.88,
    ..Color::WHITE
};
pub const CHECKBOX_TEXT: Color = Color { a: 0.9, ..CHECKBOX };

pub const HEADING2_COLOR: Color = Color {
    a: 0.95,
    ..Color::WHITE
};

pub const TEXT_INPUT_COLOR: Color = Color {
    r: 238.0 / 255.0,
    g: 238.0 / 255.0,
    b: 238.0 / 255.0,
    a: 1.0,
};
pub const TEXT_INPUT_BORDER: Color = Color {
    a: 0.2,
    ..Color::WHITE
};
