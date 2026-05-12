use iced::widget::{button, container, text};
use iced::{Background, Border, Color, Element, Length, Shadow};

use crate::core::config::*;

pub fn cyber_button<'a, Message: Clone + 'a>(
    label: &'a str,
    accent: Color,
    msg: Message,
) -> Element<'a, Message> {
    button(
        text(label)
            .size(13)
            .color(accent),
    )
    .padding([6, 14])
    .style(move |_theme, status| {
        let bg = match status {
            button::Status::Hovered | button::Status::Pressed => Color {
                a: 0.25,
                ..accent
            },
            _ => Color { a: 0.08, ..accent },
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: accent,
            border: Border {
                color: Color { a: 0.6, ..accent },
                width: 1.0,
                radius: 6.0_f32.into(),
            },
            shadow: Shadow::default(),
        }
    })
    .on_press(msg)
    .into()
}

pub fn danger_button<'a, Message: Clone + 'a>(
    label: &'a str,
    msg: Message,
) -> Element<'a, Message> {
    cyber_button(label, DANGER, msg)
}

pub fn success_button<'a, Message: Clone + 'a>(
    label: &'a str,
    msg: Message,
) -> Element<'a, Message> {
    cyber_button(label, NEON_GREEN, msg)
}

pub fn card<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding([14, 18])
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
        .width(Length::Fill)
        .into()
}

pub fn glowing_panel<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    accent: Color,
) -> Element<'a, Message> {
    container(content)
        .padding([16, 20])
        .style(move |_theme| container::Style {
            background: Some(Background::Color(BG_SECONDARY)),
            border: Border {
                color: Color { a: 0.5, ..accent },
                width: 1.0,
                radius: 12.0_f32.into(),
            },
            text_color: None,
            shadow: Shadow {
                color: Color { a: 0.3, ..accent },
                offset: iced::Vector { x: 0.0, y: 0.0 },
                blur_radius: 12.0,
            },
        })
        .width(Length::Fill)
        .into()
}

pub fn badge<'a, Message: 'a>(label: &'a str, color: Color) -> Element<'a, Message> {
    container(text(label).size(11).color(color))
        .padding([2, 8])
        .style(move |_theme| container::Style {
            background: Some(Background::Color(Color { a: 0.15, ..color })),
            border: Border {
                color: Color { a: 0.4, ..color },
                width: 1.0,
                radius: 4.0_f32.into(),
            },
            text_color: None,
            shadow: Shadow::default(),
        })
        .into()
}

pub fn divider<Message: 'static>() -> Element<'static, Message> {
    container(iced::widget::Space::new(Length::Fill, 1.0))
        .style(|_theme| container::Style {
            background: Some(Background::Color(BORDER_DIM)),
            border: Border::default(),
            text_color: None,
            shadow: Shadow::default(),
        })
        .width(Length::Fill)
        .into()
}
