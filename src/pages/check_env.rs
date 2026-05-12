use iced::widget::{button, column, container, row, text, Space};
use iced::{Background, Border, Color, Element, Length, Shadow};

use crate::core::config::*;
use crate::{Message, SetupState};

pub fn view_booting<'a>() -> Element<'a, Message> {
    centered(
        column![
            cyber_title("AUR HELPER"),
            Space::new(0, 20),
            text("Verificando dependências do sistema...")
                .size(15)
                .color(TEXT_SECONDARY),
            Space::new(0, 10),
            spinning_dots(),
        ]
        .spacing(8)
        .align_x(iced::Alignment::Center),
    )
}

pub fn view<'a>(state: &'a SetupState) -> Element<'a, Message> {
    let git_row = dep_row("git", state.has_git);
    let bd_row = dep_row("base-devel", state.has_base_devel);
    let paru_row = dep_row("paru", state.has_paru);

    let status_section = container(
        column![
            text("Dependências detectadas:")
                .size(14)
                .color(TEXT_SECONDARY),
            Space::new(0, 12),
            git_row,
            Space::new(0, 6),
            bd_row,
            Space::new(0, 6),
            paru_row,
        ]
        .spacing(0),
    )
    .padding([18, 22])
    .style(|_theme| container::Style {
        background: Some(Background::Color(BG_CARD)),
        border: Border {
            color: BORDER_DIM,
            width: 1.0,
            radius: 10.0_f32.into(),
        },
        text_color: None,
        shadow: Shadow::default(),
    })
    .width(Length::Fixed(400.0));

    let message_area = if !state.message.is_empty() {
        let color = if state.message.contains("falha") || state.message.contains("Falha") {
            DANGER
        } else {
            TEXT_SECONDARY
        };
        container(
            text(&state.message)
                .size(13)
                .color(color),
        )
        .padding([10, 14])
        .style(|_theme| container::Style {
            background: Some(Background::Color(BG_SECONDARY)),
            border: Border {
                color: BORDER_DIM,
                width: 1.0,
                radius: 8.0_f32.into(),
            },
            text_color: None,
            shadow: Shadow::default(),
        })
        .width(Length::Fixed(400.0))
    } else {
        container(Space::new(0, 0)).width(Length::Fixed(400.0))
    };

    let action = if state.is_running {
        container(
            text("Instalando... aguarde o terminal.")
                .size(14)
                .color(WARNING),
        )
        .padding([10, 0])
    } else {
        container(
            button(
                text("  Instalar Dependências  ")
                    .size(15)
                    .color(BG_PRIMARY),
            )
            .padding([12, 28])
            .style(|_theme, status| button::Style {
                background: Some(Background::Color(match status {
                    button::Status::Hovered | button::Status::Pressed => {
                        Color { r: 0.0, g: 0.75, b: 0.85, a: 1.0 }
                    }
                    _ => CYAN,
                })),
                text_color: BG_PRIMARY,
                border: Border {
                    color: CYAN,
                    width: 0.0,
                    radius: 8.0_f32.into(),
                },
                shadow: Shadow {
                    color: Color { a: 0.5, ..CYAN },
                    offset: iced::Vector { x: 0.0, y: 4.0 },
                    blur_radius: 16.0,
                },
            })
            .on_press(Message::SetupLaunchTerminal),
        )
        .padding([10, 0])
    };

    centered(
        column![
            cyber_title("AUR HELPER"),
            Space::new(0, 6),
            text("Gerenciador de Pacotes AUR para Arch Linux")
                .size(14)
                .color(TEXT_MUTED),
            Space::new(0, 32),
            container(
                text("CONFIGURAÇÃO NECESSÁRIA")
                    .size(11)
                    .color(PINK),
            )
            .padding([4, 10])
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color { a: 0.12, ..PINK })),
                border: Border {
                    color: Color { a: 0.3, ..PINK },
                    width: 1.0,
                    radius: 4.0_f32.into(),
                },
                text_color: None,
                shadow: Shadow::default(),
            }),
            Space::new(0, 24),
            status_section,
            Space::new(0, 20),
            message_area,
            Space::new(0, 20),
            action,
        ]
        .spacing(0)
        .align_x(iced::Alignment::Center),
    )
}

fn dep_row<'a>(name: &'a str, present: bool) -> Element<'a, Message> {
    let (icon, color) = if present {
        ("✔", SUCCESS)
    } else {
        ("✖", DANGER)
    };

    row![
        text(icon).size(14).color(color),
        Space::new(10, 0),
        text(name).size(14).color(TEXT_PRIMARY),
        Space::new(Length::Fill, 0),
        text(if present { "instalado" } else { "ausente" })
            .size(12)
            .color(color),
    ]
    .align_y(iced::Alignment::Center)
    .into()
}

fn cyber_title<'a>(label: &'a str) -> Element<'a, Message> {
    text(label)
        .size(36)
        .color(CYAN)
        .font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        })
        .into()
}

fn spinning_dots<'a>() -> Element<'a, Message> {
    row![
        dot(CYAN),
        Space::new(6, 0),
        dot(PURPLE),
        Space::new(6, 0),
        dot(PINK),
    ]
    .align_y(iced::Alignment::Center)
    .into()
}

fn dot<Message: 'static>(color: Color) -> Element<'static, Message> {
    container(Space::new(8.0, 8.0))
        .style(move |_theme| container::Style {
            background: Some(Background::Color(color)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0_f32.into(),
            },
            text_color: None,
            shadow: Shadow {
                color: Color { a: 0.8, ..color },
                offset: iced::Vector { x: 0.0, y: 0.0 },
                blur_radius: 6.0,
            },
        })
        .into()
}

fn centered<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .style(|_theme| container::Style {
            background: Some(Background::Color(BG_PRIMARY)),
            border: Border::default(),
            text_color: None,
            shadow: Shadow::default(),
        })
        .into()
}
