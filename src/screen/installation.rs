use iced::{Text, button, Column, Button, Container, Length, Alignment};

use crate::app::{Context, Message};
use crate::style::*;
use crate::screen::Screen;

#[derive(Default, Debug, Clone)]
pub struct State {
    cancel_button: button::State
}

pub fn show<'a>(_context: &Context, state: &'a mut State) -> iced::Element<'a, Message> {
    let State { cancel_button } = state;

    Container::new(Column::new()
            .align_items(Alignment::Center)
            .push(Text::new("Installation...").style_heading())
            .push(Button::new(cancel_button, Text::new("Отменить"))
                .padding([9, 33])
                .on_press(Message::UpdateScreen(Screen::finish()))
                .style(AccentButtonStyle::Destructive)))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}
