use iced::{Text, button, text_input, Length, Checkbox, Space, Button, Column, TextInput, Row};

use crate::app::{Context, Message};
use crate::style::*;
use crate::screen::Screen;

#[derive(Default, Debug, Clone)]
pub struct State {
    back_button: button::State,
    file_input: text_input::State
}

pub fn show<'a>(context: &Context, state: &'a mut State) -> iced::Element<'a, Message> {
    let State { back_button, file_input } = state;

    Column::new()
        .padding([50, 70])
        .spacing(20)
        .push(Text::new("Настройки установки").style_heading())
        .push(Checkbox::new(
            context.create_shortcut,
            "Создать ярлык на рабочем столе",
            |checked| Message::UpdateCreateShortcut(checked))
            .style(CheckboxStyle))
        .push(Checkbox::new(context.install_for_all_users,
            "Установить для всех пользователей",
            |checked| Message::UpdateInstallForAllUsers(checked))
            .style(CheckboxStyle)) // TODO 3rd checkbox
        .push(Space::new(Length::Fill, Length::Fill))
        .push(Row::new()
            .push(Column::new()
                .push(Text::new("Путь установки:").style_heading2())
                .push(TextInput::new(file_input,
                    "", // TODO placeholder
                    &context.installation_path,
                    |path| Message::UpdateInstallationPath(path))
                    .width(Length::Units(270))
                    .style(TextInputStyle)))
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(Button::new(back_button, Text::new("Назад"))
                .padding([9, 33])
                .on_press(Message::UpdateScreen(Screen::greeting()))
                .style(AccentButtonStyle::Default)))
        .into()
}
