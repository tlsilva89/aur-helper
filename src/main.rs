use iced::time;
use iced::{Element, Subscription, Task, Theme};
use std::time::Duration;

mod core;
mod pages;
mod ui_components;

use core::config::cyberpunk_theme;
use core::models::{Package, Tab};
use core::services::{package_manager, validator};

#[derive(Debug, Clone)]
pub enum Message {
    CheckDepsResult(bool, bool, bool),

    SetupLaunchTerminal,
    SetupComplete(bool),

    LoadInstalled,
    InstalledLoaded(Result<Vec<Package>, String>),
    CheckUpdates,
    UpdatesLoaded(Result<Vec<Package>, String>),

    SearchInput(String),
    SearchTick,
    SearchResults(Result<Vec<Package>, String>),
    CloseSearchDropdown,

    SwitchTab(Tab),
    InstallPackage(String),
    RemovePackage(String),
    UpdatePackage(String),
    UpdateAll,
    OperationComplete(Result<(), String>),

    DismissNotification,
}

#[derive(Debug, Default)]
pub struct App {
    screen: Screen,
}

#[derive(Debug, Default)]
enum Screen {
    #[default]
    Booting,
    Setup(SetupState),
    Dashboard(DashboardState),
}

#[derive(Debug, Default)]
pub struct SetupState {
    pub has_git: bool,
    pub has_base_devel: bool,
    pub has_paru: bool,
    pub is_running: bool,
    pub message: String,
}

#[derive(Debug, Default)]
pub struct DashboardState {
    pub active_tab: Tab,
    pub search_input: String,
    pub search_results: Vec<Package>,
    pub show_search_results: bool,
    pub installed_packages: Vec<Package>,
    pub updates: Vec<Package>,
    pub is_searching: bool,
    pub pending_search_tick: u8,
    pub operation_in_progress: Option<String>,
    pub notification: Option<(String, bool)>,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let task = Task::perform(
            async {
                let has_git = validator::check_binary("git");
                let has_base_devel = validator::check_base_devel();
                let has_paru = validator::check_binary("paru");
                (has_git, has_base_devel, has_paru)
            },
            |(git, bd, paru)| Message::CheckDepsResult(git, bd, paru),
        );
        (Self::default(), task)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CheckDepsResult(has_git, has_base_devel, has_paru) => {
                if has_paru {
                    self.screen = Screen::Dashboard(DashboardState::default());
                    return Task::batch([
                        Task::done(Message::LoadInstalled),
                        Task::done(Message::CheckUpdates),
                    ]);
                }
                self.screen = Screen::Setup(SetupState {
                    has_git,
                    has_base_devel,
                    has_paru,
                    is_running: false,
                    message: String::new(),
                });
                Task::none()
            }

            Message::SetupLaunchTerminal => {
                if let Screen::Setup(ref mut s) = self.screen {
                    s.is_running = true;
                    s.message = "Iniciando terminal de instalação...".into();
                }
                Task::perform(
                    validator::run_setup_in_terminal(),
                    |result| Message::SetupComplete(result.is_ok()),
                )
            }

            Message::SetupComplete(_) => {
                let has_paru = validator::check_binary("paru");
                if has_paru {
                    self.screen = Screen::Dashboard(DashboardState::default());
                    return Task::batch([
                        Task::done(Message::LoadInstalled),
                        Task::done(Message::CheckUpdates),
                    ]);
                }
                if let Screen::Setup(ref mut s) = self.screen {
                    s.is_running = false;
                    s.message =
                        "Instalação não concluída. Verifique os erros e tente novamente.".into();
                }
                Task::none()
            }

            Message::LoadInstalled => Task::perform(
                package_manager::get_installed_packages(),
                |r| Message::InstalledLoaded(r.map_err(|e| e.to_string())),
            ),

            Message::InstalledLoaded(result) => {
                if let Screen::Dashboard(ref mut s) = self.screen {
                    match result {
                        Ok(packages) => s.installed_packages = packages,
                        Err(e) => s.notification = Some((e, true)),
                    }
                }
                Task::none()
            }

            Message::CheckUpdates => Task::perform(
                package_manager::get_updates(),
                |r| Message::UpdatesLoaded(r.map_err(|e| e.to_string())),
            ),

            Message::UpdatesLoaded(result) => {
                if let Screen::Dashboard(ref mut s) = self.screen {
                    match result {
                        Ok(packages) => s.updates = packages,
                        Err(e) => s.notification = Some((e, true)),
                    }
                }
                Task::none()
            }

            Message::SearchInput(query) => {
                if let Screen::Dashboard(ref mut s) = self.screen {
                    s.search_input = query.clone();
                    if query.is_empty() {
                        s.show_search_results = false;
                        s.search_results.clear();
                        s.pending_search_tick = 0;
                    } else {
                        s.pending_search_tick = 3;
                    }
                }
                Task::none()
            }

            Message::SearchTick => {
                if let Screen::Dashboard(ref mut s) = self.screen {
                    if s.pending_search_tick > 0 {
                        s.pending_search_tick -= 1;
                        if s.pending_search_tick == 0 && !s.search_input.is_empty() {
                            let query = s.search_input.clone();
                            s.is_searching = true;
                            return Task::perform(
                                package_manager::search_packages(query),
                                |r| Message::SearchResults(r.map_err(|e| e.to_string())),
                            );
                        }
                    }
                }
                Task::none()
            }

            Message::SearchResults(result) => {
                if let Screen::Dashboard(ref mut s) = self.screen {
                    s.is_searching = false;
                    match result {
                        Ok(packages) => {
                            s.search_results = packages;
                            s.show_search_results = !s.search_results.is_empty();
                        }
                        Err(e) => s.notification = Some((format!("Erro na busca: {e}"), true)),
                    }
                }
                Task::none()
            }

            Message::CloseSearchDropdown => {
                if let Screen::Dashboard(ref mut s) = self.screen {
                    s.show_search_results = false;
                }
                Task::none()
            }

            Message::SwitchTab(tab) => {
                if let Screen::Dashboard(ref mut s) = self.screen {
                    s.active_tab = tab;
                    s.show_search_results = false;
                }
                Task::none()
            }

            Message::InstallPackage(name) => {
                if let Screen::Dashboard(ref mut s) = self.screen {
                    s.operation_in_progress = Some(name.clone());
                }
                Task::perform(
                    package_manager::install_package(name),
                    |r| Message::OperationComplete(r.map_err(|e| e.to_string())),
                )
            }

            Message::RemovePackage(name) => {
                if let Screen::Dashboard(ref mut s) = self.screen {
                    s.operation_in_progress = Some(name.clone());
                }
                Task::perform(
                    package_manager::remove_package(name),
                    |r| Message::OperationComplete(r.map_err(|e| e.to_string())),
                )
            }

            Message::UpdatePackage(name) => {
                if let Screen::Dashboard(ref mut s) = self.screen {
                    s.operation_in_progress = Some(name.clone());
                }
                Task::perform(
                    package_manager::install_package(name),
                    |r| Message::OperationComplete(r.map_err(|e| e.to_string())),
                )
            }

            Message::UpdateAll => {
                if let Screen::Dashboard(ref mut s) = self.screen {
                    s.operation_in_progress = Some("all".into());
                }
                Task::perform(
                    package_manager::update_all(),
                    |r| Message::OperationComplete(r.map_err(|e| e.to_string())),
                )
            }

            Message::OperationComplete(result) => {
                if let Screen::Dashboard(ref mut s) = self.screen {
                    s.operation_in_progress = None;
                    match result {
                        Ok(()) => s.notification = Some(("Operação concluída com sucesso!".into(), false)),
                        Err(e) => s.notification = Some((format!("Erro: {e}"), true)),
                    }
                }
                Task::batch([
                    Task::done(Message::LoadInstalled),
                    Task::done(Message::CheckUpdates),
                ])
            }

            Message::DismissNotification => {
                if let Screen::Dashboard(ref mut s) = self.screen {
                    s.notification = None;
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match &self.screen {
            Screen::Booting => pages::check_env::view_booting(),
            Screen::Setup(state) => pages::check_env::view(state),
            Screen::Dashboard(state) => pages::dashboard::view(state),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match &self.screen {
            Screen::Dashboard(s) if s.pending_search_tick > 0 || s.is_searching => {
                time::every(Duration::from_millis(100)).map(|_| Message::SearchTick)
            }
            _ => Subscription::none(),
        }
    }

    fn theme(&self) -> Theme {
        cyberpunk_theme()
    }
}

fn main() -> iced::Result {
    iced::application("AUR Helper", App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .window(iced::window::Settings {
            size: iced::Size::new(1100.0, 720.0),
            min_size: Some(iced::Size::new(860.0, 560.0)),
            ..Default::default()
        })
        .run_with(App::new)
}
