use iced::{Color, Element, Sandbox};

use crate::screen::*;
use crate::installer::default_installation_directory;

pub struct App {
    pub screen: Screen,
    pub context: Context,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let install_for_all_users = cfg!(windows);
        let mut manual_path_selection = false;
        let installation_path = match default_installation_directory(install_for_all_users) {
            Some(path) => path,
            None => {
                manual_path_selection = true;
                String::new()
            }
        };

        App {
            screen: Screen::greeting(),
            context: Context {
                exit: false,
                create_shortcut: true,
                install_for_all_users,
                manual_path_selection,
                installation_path,
                start_launcher: true
            },
        }
    }

    fn title(&self) -> String {
        "Obvilion Network installer".to_string()
    }

    fn background_color(&self) -> Color {
        Color::from_rgb8(18, 18, 21)
    }

    fn should_exit(&self) -> bool {
        self.context.exit
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::UpdateScreen(screen) => {
                self.screen = screen
            },
            Message::UpdateCreateShortcut(create_shortcut) => {
                self.context.create_shortcut = create_shortcut
            },
            Message::UpdateInstallForAllUsers(install_for_all_users) => {
                self.context.install_for_all_users = install_for_all_users
            },
            Message::UpdateManualPathSelection(manual_path_selection) => {
                self.context.manual_path_selection = manual_path_selection;
            },
            Message::UpdateInstallationPath(path) => {
                self.context.installation_path = path;
            },
            Message::UpdateStartLauncher(start_launcher) => {
                self.context.start_launcher = start_launcher;
            },
            Message::Exit => {
                if self.context.start_launcher {
                    // TODO: actually start
                    println!("Starting launcher");
                }
                self.context.exit = true;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        self.screen.show(&self.context)
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateScreen(Screen),
    UpdateCreateShortcut(bool),
    UpdateInstallForAllUsers(bool),
    UpdateManualPathSelection(bool),
    UpdateInstallationPath(String),
    UpdateStartLauncher(bool),
    Exit
}

pub struct Context {
    pub exit: bool,
    pub create_shortcut: bool,
    pub install_for_all_users: bool,
    pub manual_path_selection: bool,
    pub installation_path: String,
    pub start_launcher: bool
}
