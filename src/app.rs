use iced::image::Handle as ImageHandle;
use iced::svg::Handle as SvgHandle;
use iced::{
    button, Alignment, Button, Color, Column, Container, Element, Image, Length, Row, Sandbox, Space, Svg,
    Text,
};

use crate::assets;
use crate::style::{AccentButtonStyle, ImageButtonStyle, TextStyles};

pub struct App {
    screen: Screen,
    exit: bool,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            screen: Screen::greeting(),
            exit: false,
        }
    }

    fn title(&self) -> String {
        "Obvilion Network installer".to_string()
    }

    fn background_color(&self) -> Color {
        Color::from_rgb8(18, 18, 21)
    }

    fn should_exit(&self) -> bool {
        self.exit
    }

    fn update(&mut self, message: Message) {
        self.screen = match message {
            Message::BeginInstall => Screen::Installation,
            Message::CancelInstall => Screen::greeting(),
            Message::OpenSettings => Screen::Settings,
            Message::CloseSettings => Screen::greeting(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content: Element<Message> = match &mut self.screen {
            Screen::Greeting { install_button, settings_button } => Row::new()
                .align_items(Alignment::Center)
                .spacing(70)
                .push(Image::new(ImageHandle::from_memory(assets::LOGO_PNG.to_vec()))
                    .width(Length::Units(166))
                    .height(Length::Units(166)))
                .push(Column::new()
                    .push(Text::new("Установка среды запуска Obvilion Network").style_heading())
                    .push(Space::new(Length::Units(0), Length::Units(20)))
                    .push(Text::new("Этот мастер установки поможет Вам установить ПО для правильной работы Obvilion Launcher").style_description())
                    .push(Space::new(Length::Units(0), Length::Units(35)))
                    .push(Row::new()
                        .align_items(Alignment::Center)
                        .push(Button::new(install_button, Text::new("Установить"))
                            .style(AccentButtonStyle::Default)
                            .padding([9, 33])
                            .on_press(Message::BeginInstall))
                        .push(Space::new(Length::Units(5), Length::Units(0)))
                        .push(Button::new(settings_button, Svg::new(SvgHandle::from_memory(assets::SETTINGS_SVG)))
                            .on_press(Message::OpenSettings)
                            .style(ImageButtonStyle)))).into(),
            Screen::Settings => {
                Text::new("Settings").style_heading().into()
            },
            Screen::Installation => {
                Text::new("Installation...").style_heading().into()
            }
            Screen::Finish => {
                Text::new("Installation completed").style_heading().into()
            }
        };

        Container::new(content)
            .padding([0, 70])
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenSettings,
    CloseSettings,
    BeginInstall,
    CancelInstall,
}

enum Screen {
    Greeting {
        install_button: button::State,
        settings_button: button::State,
    },
    Settings,
    Installation,
    Finish,
}

impl Screen {
    fn greeting() -> Self {
        Self::Greeting {
            install_button: button::State::default(),
            settings_button: button::State::default()
        }
    }

    fn settings() -> Self {
        Self::Settings
    }

    fn installation() -> Self {
        Self::Installation
    }

    fn finish() -> Self {
        Self::Finish
    }
}
