use iced::image::Handle as ImageHandle;
use iced::{Text, button, Container, Row, Alignment, Space, Length, Button, Column, Image, Checkbox};

use crate::app::{Context, Message};
use crate::assets;
use crate::style::*;

#[derive(Default, Debug, Clone)]
pub struct State {
    finish_button: button::State,
}

pub fn show<'a>(context: &Context, state: &'a mut State) -> iced::Element<'a, Message> {
    let State { finish_button } = state;

    Container::new(Row::new()
            .align_items(Alignment::Center)
            .spacing(70)
            .push(Image::new(ImageHandle::from_memory(assets::LOGO_PNG.to_vec()))
                .width(Length::Units(166))
                .height(Length::Units(166)))
            .push(Column::new()
                .push(Text::new("Установка успешно завершена").style_heading())
                .push(Space::new(Length::Shrink, Length::Units(20)))
                .push(Text::new("Лаунчер был установлен по данному пути:").style_description())
                .push(Text::new(&context.installation_path).style_description_highlight())
                .push(Space::new(Length::Shrink, Length::Units(35)))
                .push(Row::new()
                    .align_items(Alignment::Center)
                    .spacing(40)
                    .push(Button::new(finish_button, Text::new("Завершить"))
                        .style(AccentButtonStyle::Default)
                        .padding([9, 33])
                        .on_press(Message::Exit))
                    .push(Checkbox::new(context.start_launcher,
                        "Запустить лаунчер",
                        |checked| Message::UpdateStartLauncher(checked))
                        .style(CheckboxStyle)))))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding([0, 70])
        .into()
}
