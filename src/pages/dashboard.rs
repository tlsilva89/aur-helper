use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Background, Border, Color, Element, Length, Shadow};

use crate::core::config::*;
use crate::core::models::{Package, Tab};
use crate::ui_components::{badge, cyber_button, danger_button};
use crate::{DashboardState, Message};

pub fn view<'a>(state: &'a DashboardState) -> Element<'a, Message> {
    let header = build_header(state);
    let search_results = if state.show_search_results && !state.search_results.is_empty() {
        Some(build_search_dropdown(state))
    } else {
        None
    };
    let tabs = build_tabs(state);
    let content = build_content(state);
    let footer = build_footer(state);

    let mut main_col = column![header];

    if let Some(dropdown) = search_results {
        main_col = main_col.push(dropdown);
    } else {
        main_col = main_col.push(tabs);
        main_col = main_col.push(content);
    }

    if let Some((msg, is_err)) = &state.notification {
        main_col = main_col.push(build_notification(msg, *is_err));
    }

    main_col = main_col.push(footer);

    container(main_col.spacing(0))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Background::Color(BG_PRIMARY)),
            border: Border::default(),
            text_color: None,
            shadow: Shadow::default(),
        })
        .into()
}

fn build_header<'a>(state: &'a DashboardState) -> Element<'a, Message> {
    let search_bar = text_input("  Buscar pacotes no AUR...", &state.search_input)
        .on_input(Message::SearchInput)
        .padding([10, 16])
        .size(15)
        .width(Length::Fixed(480.0))
        .style(|_theme, status| text_input::Style {
            background: Background::Color(BG_SECONDARY),
            border: Border {
                color: match status {
                    text_input::Status::Focused => CYAN,
                    text_input::Status::Hovered => Color { a: 0.4, ..CYAN },
                    _ => BORDER_DIM,
                },
                width: 1.5,
                radius: 8.0_f32.into(),
            },
            icon: TEXT_SECONDARY,
            placeholder: TEXT_MUTED,
            value: TEXT_PRIMARY,
            selection: Color { a: 0.3, ..CYAN },
        });

    let search_indicator = if state.is_searching {
        text("◌").size(16).color(CYAN)
    } else {
        text("⌕").size(18).color(TEXT_SECONDARY)
    };

    let logo = row![
        text("◈").size(24).color(CYAN),
        Space::new(8, 0),
        column![
            text("AUR HELPER")
                .size(18)
                .color(TEXT_PRIMARY)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                }),
            text("Arch Linux Package Manager")
                .size(11)
                .color(TEXT_MUTED),
        ]
        .spacing(1),
    ]
    .align_y(iced::Alignment::Center)
    .spacing(0);

    let search_row = row![search_bar, Space::new(8, 0), search_indicator]
        .align_y(iced::Alignment::Center);

    let header_content = row![logo, Space::new(Length::Fill, 0), search_row]
        .align_y(iced::Alignment::Center)
        .padding([0, 4]);

    container(header_content)
        .padding([16, 24])
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Background::Color(BG_SECONDARY)),
            border: Border {
                color: BORDER_DIM,
                width: 0.0,
                radius: 0.0_f32.into(),
            },
            text_color: None,
            shadow: Shadow {
                color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.4 },
                offset: iced::Vector { x: 0.0, y: 2.0 },
                blur_radius: 8.0,
            },
        })
        .into()
}

fn build_search_dropdown<'a>(state: &'a DashboardState) -> Element<'a, Message> {
    let count = state.search_results.len();
    let header = container(
        row![
            text(format!("{count} resultado(s) para \"{}\"", state.search_input))
                .size(12)
                .color(TEXT_SECONDARY),
            Space::new(Length::Fill, 0),
            button(text("✕").size(12).color(TEXT_MUTED))
                .padding([2, 8])
                .style(|_theme, _status| button::Style {
                    background: None,
                    text_color: TEXT_MUTED,
                    border: Border::default(),
                    shadow: Shadow::default(),
                })
                .on_press(Message::CloseSearchDropdown),
        ]
        .align_y(iced::Alignment::Center),
    )
    .padding([8, 20])
    .width(Length::Fill)
    .style(|_theme| container::Style {
        background: Some(Background::Color(BG_SECONDARY)),
        border: Border {
            color: BORDER_DIM,
            width: 0.0,
            radius: 0.0_f32.into(),
        },
        text_color: None,
        shadow: Shadow::default(),
    });

    let items = state.search_results.iter().map(|pkg| search_result_row(pkg, state));

    let list = scrollable(
        column(items)
            .spacing(1)
            .padding(iced::Padding { top: 4.0, right: 16.0, bottom: 4.0, left: 16.0 }),
    )
    .height(Length::Fixed(380.0));

    column![header, list]
        .spacing(0)
        .into()
}

fn search_result_row<'a>(pkg: &'a Package, state: &'a DashboardState) -> Element<'a, Message> {
    let is_operating = state
        .operation_in_progress
        .as_deref()
        .map_or(false, |n| n == pkg.name);

    let action = if is_operating {
        container(
            text("...").size(13).color(TEXT_MUTED),
        )
        .padding([6, 12])
        .into()
    } else if pkg.is_installed {
        danger_button("Remover", Message::RemovePackage(pkg.name.clone()))
    } else {
        cyber_button("Instalar", NEON_GREEN, Message::InstallPackage(pkg.name.clone()))
    };

    let votes_str = pkg.votes.map(|v| format!("★ {v}")).unwrap_or_default();

    let desc = if pkg.description.is_empty() {
        "Sem descrição"
    } else {
        &pkg.description
    };

    let row_content = row![
        column![
            row![
                text(&pkg.name)
                    .size(14)
                    .color(CYAN)
                    .font(iced::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
                Space::new(10, 0),
                text(&pkg.version).size(12).color(TEXT_MUTED),
                Space::new(10, 0),
                if pkg.is_installed {
                    badge("instalado", NEON_GREEN)
                } else {
                    badge("aur", PURPLE)
                },
                if pkg.out_of_date {
                    badge(" obsoleto", WARNING)
                } else {
                    Space::new(0, 0).into()
                },
            ]
            .align_y(iced::Alignment::Center)
            .spacing(0),
            Space::new(0, 4),
            text(desc)
                .size(12)
                .color(TEXT_SECONDARY),
            {
                let votes_area: Element<'a, Message> = if !votes_str.is_empty() {
                    row![
                        Space::new(0, 2),
                        text(votes_str.clone()).size(11).color(TEXT_MUTED),
                    ]
                    .into()
                } else {
                    Space::new(0, 0).into()
                };
                votes_area
            },
        ]
        .spacing(0),
        Space::new(Length::Fill, 0),
        action,
    ]
    .align_y(iced::Alignment::Center);

    container(row_content)
        .padding([10, 12])
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Background::Color(BG_CARD)),
            border: Border {
                color: BORDER_DIM,
                width: 1.0,
                radius: 8.0_f32.into(),
            },
            text_color: None,
            shadow: Shadow::default(),
        })
        .into()
}

fn build_tabs<'a>(state: &'a DashboardState) -> Element<'a, Message> {
    let inst_count = state.installed_packages.len();
    let upd_count = state.updates.len();

    let tab_btn = |label: String, tab: Tab, active: bool| -> Element<'a, Message> {
        let color = if active { CYAN } else { TEXT_SECONDARY };
        button(text(label).size(14).color(color))
            .padding([10, 20])
            .style(move |_theme, status| {
                let bg = if active {
                    Color { a: 0.12, ..CYAN }
                } else if matches!(status, button::Status::Hovered) {
                    Color { a: 0.06, ..CYAN }
                } else {
                    Color::TRANSPARENT
                };
                button::Style {
                    background: Some(Background::Color(bg)),
                    text_color: color,
                    border: Border {
                        color: if active {
                            Color { a: 0.5, ..CYAN }
                        } else {
                            Color::TRANSPARENT
                        },
                        width: if active { 0.0 } else { 0.0 },
                        radius: 8.0_f32.into(),
                    },
                    shadow: Shadow::default(),
                }
            })
            .on_press(Message::SwitchTab(tab))
            .into()
    };

    let update_badge = if upd_count > 0 {
        container(
            text(upd_count.to_string())
                .size(10)
                .color(BG_PRIMARY),
        )
        .padding([1, 6])
        .style(|_theme| container::Style {
            background: Some(Background::Color(WARNING)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 10.0_f32.into(),
            },
            text_color: None,
            shadow: Shadow::default(),
        })
    } else {
        container(Space::new(0, 0))
    };

    let tabs_row = row![
        tab_btn(
            format!("Instalados ({inst_count})"),
            Tab::Installed,
            state.active_tab == Tab::Installed,
        ),
        Space::new(4, 0),
        row![
            tab_btn(
                format!("Atualizações ({upd_count})"),
                Tab::Updates,
                state.active_tab == Tab::Updates,
            ),
            update_badge,
        ]
        .align_y(iced::Alignment::Center)
        .spacing(4),
        Space::new(Length::Fill, 0),
        if state.active_tab == Tab::Updates {
            cyber_button(
                "Atualizar Tudo",
                CYAN,
                Message::UpdateAll,
            )
        } else {
            Space::new(0, 0).into()
        },
    ]
    .align_y(iced::Alignment::Center)
    .spacing(0);

    container(tabs_row)
        .padding([8, 20])
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Background::Color(BG_SECONDARY)),
            border: Border {
                color: BORDER_DIM,
                width: 1.0,
                radius: 0.0_f32.into(),
            },
            text_color: None,
            shadow: Shadow::default(),
        })
        .into()
}

fn build_content<'a>(state: &'a DashboardState) -> Element<'a, Message> {
    let packages = match state.active_tab {
        Tab::Installed => &state.installed_packages,
        Tab::Updates => &state.updates,
    };

    let body: Element<Message> = if packages.is_empty() {
        container(
            column![
                text(match state.active_tab {
                    Tab::Installed => "◈",
                    Tab::Updates => "✔",
                })
                .size(40)
                .color(TEXT_MUTED),
                Space::new(0, 12),
                text(match state.active_tab {
                    Tab::Installed => "Nenhum pacote AUR instalado",
                    Tab::Updates => "Sistema atualizado!",
                })
                .size(16)
                .color(TEXT_SECONDARY),
                Space::new(0, 6),
                text(match state.active_tab {
                    Tab::Installed => "Use a busca acima para encontrar e instalar pacotes.",
                    Tab::Updates => "Todos os pacotes AUR estão na versão mais recente.",
                })
                .size(13)
                .color(TEXT_MUTED),
            ]
            .spacing(0)
            .align_x(iced::Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .into()
    } else {
        let items = packages.iter().map(|pkg| installed_package_row(pkg, state));
        scrollable(
            column(items)
                .spacing(6)
                .padding([12, 20]),
        )
        .height(Length::Fill)
        .into()
    };

    container(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Background::Color(BG_PRIMARY)),
            border: Border::default(),
            text_color: None,
            shadow: Shadow::default(),
        })
        .into()
}

fn installed_package_row<'a>(pkg: &'a Package, state: &'a DashboardState) -> Element<'a, Message> {
    let is_operating = state
        .operation_in_progress
        .as_deref()
        .map_or(false, |n| n == pkg.name);

    let action = if is_operating {
        container(
            text("Aguarde...").size(12).color(WARNING),
        )
        .padding([6, 12])
        .into()
    } else {
        danger_button("Remover", Message::RemovePackage(pkg.name.clone()))
    };

    let update_btn: Element<Message> = if pkg.version.contains('→') {
        cyber_button(
            "Atualizar",
            CYAN,
            Message::UpdatePackage(pkg.name.clone()),
        )
    } else {
        Space::new(0, 0).into()
    };

    container(
        row![
            column![
                row![
                    text(&pkg.name)
                        .size(14)
                        .color(TEXT_PRIMARY)
                        .font(iced::Font { weight: iced::font::Weight::Semibold, ..Default::default() }),
                    Space::new(10, 0),
                    badge("aur", PURPLE),
                ]
                .align_y(iced::Alignment::Center)
                .spacing(0),
                Space::new(0, 4),
                text(&pkg.version).size(12).color(TEXT_MUTED),
            ]
            .spacing(0),
            Space::new(Length::Fill, 0),
            update_btn,
            Space::new(8, 0),
            action,
        ]
        .align_y(iced::Alignment::Center),
    )
    .padding([12, 16])
    .width(Length::Fill)
    .style(|_theme| container::Style {
        background: Some(Background::Color(BG_CARD)),
        border: Border {
            color: BORDER_DIM,
            width: 1.0,
            radius: 8.0_f32.into(),
        },
        text_color: None,
        shadow: Shadow::default(),
    })
    .into()
}

fn build_notification<'a>(msg: &'a str, is_error: bool) -> Element<'a, Message> {
    let (icon, color) = if is_error {
        ("✖", DANGER)
    } else {
        ("✔", SUCCESS)
    };

    container(
        row![
            text(icon).size(14).color(color),
            Space::new(10, 0),
            text(msg).size(13).color(TEXT_PRIMARY),
            Space::new(Length::Fill, 0),
            button(text("✕").size(12).color(TEXT_MUTED))
                .padding([2, 8])
                .style(|_theme, _status| button::Style {
                    background: None,
                    text_color: TEXT_MUTED,
                    border: Border::default(),
                    shadow: Shadow::default(),
                })
                .on_press(Message::DismissNotification),
        ]
        .align_y(iced::Alignment::Center),
    )
    .padding([10, 20])
    .width(Length::Fill)
    .style(move |_theme| container::Style {
        background: Some(Background::Color(Color { a: 0.12, ..color })),
        border: Border {
            color: Color { a: 0.4, ..color },
            width: 1.0,
            radius: 0.0_f32.into(),
        },
        text_color: None,
        shadow: Shadow::default(),
    })
    .into()
}

fn build_footer<'a>(state: &'a DashboardState) -> Element<'a, Message> {
    let inst = state.installed_packages.len();
    let upd = state.updates.len();

    let status_dot = container(Space::new(8.0, 8.0))
        .style(|_theme| container::Style {
            background: Some(Background::Color(SUCCESS)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0_f32.into(),
            },
            text_color: None,
            shadow: Shadow {
                color: Color { a: 0.6, ..SUCCESS },
                offset: iced::Vector { x: 0.0, y: 0.0 },
                blur_radius: 4.0,
            },
        });

    let refresh_btn = button(text("↻  Verificar Atualizações").size(12).color(TEXT_SECONDARY))
        .padding([4, 12])
        .style(|_theme, status| button::Style {
            background: Some(Background::Color(if matches!(status, button::Status::Hovered) {
                Color { a: 0.1, ..CYAN }
            } else {
                Color::TRANSPARENT
            })),
            text_color: TEXT_SECONDARY,
            border: Border {
                color: BORDER_DIM,
                width: 1.0,
                radius: 6.0_f32.into(),
            },
            shadow: Shadow::default(),
        })
        .on_press(Message::CheckUpdates);

    container(
        row![
            status_dot,
            Space::new(8, 0),
            text("Sincronizado").size(12).color(SUCCESS),
            Space::new(16, 0),
            text("·").size(12).color(TEXT_MUTED),
            Space::new(16, 0),
            text(format!("{inst} pacotes instalados"))
                .size(12)
                .color(TEXT_SECONDARY),
            {
                let upd_area: Element<'a, Message> = if upd > 0 {
                    row![
                        Space::new(16, 0),
                        text("·").size(12).color(TEXT_MUTED),
                        Space::new(16, 0),
                        text(format!("{upd} atualização(ões) disponível(is)"))
                            .size(12)
                            .color(WARNING),
                    ]
                    .into()
                } else {
                    Space::new(0, 0).into()
                };
                upd_area
            },
            Space::new(Length::Fill, 0),
            refresh_btn,
        ]
        .align_y(iced::Alignment::Center),
    )
    .padding([8, 20])
    .width(Length::Fill)
    .style(|_theme| container::Style {
        background: Some(Background::Color(BG_SECONDARY)),
        border: Border {
            color: BORDER_DIM,
            width: 1.0,
            radius: 0.0_f32.into(),
        },
        text_color: None,
        shadow: Shadow::default(),
    })
    .into()
}
