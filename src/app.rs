use iced::{button, svg, Button, Color, Column, Container, Element, Image, Length, Row, Sandbox, Text, Space};

use crate::style::{InstallationButtonStyle, SettingsButtonStyle, TextStyles};

pub struct App {
    screen: Screen,
    exit: bool,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            screen: Screen::Greeting {
                install_button: button::State::default(),
                settings_button: button::State::default(),
            },
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
        println!("{message:?}");
    }

    fn view(&mut self) -> Element<Message> {
        let content = match &mut self.screen {
            Screen::Greeting { install_button, settings_button } => Row::new()
                .push(Container::new(Image::new("assets/logo.png")
                        .width(Length::Units(166))
                        .height(Length::Units(166)))
                    .width(Length::Units(166 + 70))
                    .height(Length::Fill)
                    .center_y())
                .push(Container::new(Column::new()
                        .push(Text::new("Установка среды запуска Obvilion Network").style_heading())
                        .push(Space::new(Length::Units(0), Length::Units(20)))
                        .push(Text::new("Этот мастер установки поможет Вам установить ПО для правильной работы Obvilion Launcher").style_description())
                        .push(Space::new(Length::Units(0), Length::Units(35)))
                        .push(Row::new()
                            .push(Button::new(install_button, Text::new("Установить"))
                                .style(InstallationButtonStyle)
                                .on_press(Message::BeginInstall))
                            .push(Button::new(settings_button, svg::Svg::new(svg::Handle::from_path("assets/settings.svg")))
                                .on_press(Message::OpenSettings)
                                .style(SettingsButtonStyle))))
                    .height(Length::Fill)
                    .center_y()),
            _ => todo!(),
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
