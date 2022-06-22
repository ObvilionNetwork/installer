use iced::image::Handle as ImageHandle;
use iced::svg::Handle as SvgHandle;
use iced::{button, Alignment, Button, Column, Container, Image, Length, Row, Space, Svg, Text};

use crate::app::{Context, Message};
use crate::assets;
use crate::style::*;
use crate::screen::Screen;

#[derive(Default, Debug, Clone)]
pub struct State {
    install_button: button::State,
    settings_button: button::State,
}

pub fn show<'a>(_context: &Context, state: &'a mut State) -> iced::Element<'a, Message> {
    let State {
        install_button,
        settings_button,
    } = state;

    Container::new(Row::new()
            .align_items(Alignment::Center)
            .spacing(70)
            .push(Image::new(ImageHandle::from_memory(assets::LOGO_PNG.to_vec()))
                .width(Length::Units(166))
                .height(Length::Units(166)))
            .push(Column::new()
                .push(Text::new("Установка среды запуска Obvilion Network").style_heading())
                .push(Space::new(Length::Shrink, Length::Units(20)))
                .push(Text::new("Этот мастер установки поможет Вам установить ПО для правильной работы Obvilion Launcher").style_description())
                .push(Space::new(Length::Shrink, Length::Units(35)))
                .push(Row::new()
                    .align_items(Alignment::Center)
                    .spacing(5)
                    .push(Button::new(install_button, Text::new("Установить"))
                        .style(AccentButtonStyle::Default)
                        .padding([9, 33])
                        .on_press(Message::UpdateScreen(Screen::installation())))
                    .push(Button::new(settings_button, Svg::new(SvgHandle::from_memory(assets::SETTINGS_SVG)))
                        .on_press(Message::UpdateScreen(Screen::settings()))
                        .style(ImageButtonStyle)))))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding([0, 70])
        .into()
}
