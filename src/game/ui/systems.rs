use bevy::color::Color;
use bevy::prelude::*;
use univis_ui::prelude::*;
use univis_ui::widget::text_label::UTextLabel;

pub fn spawn_ui_screen() -> (UScreenRoot, UNode, ULayout, Camera2d, Camera) {
    (
        UScreenRoot,
        UNode {
            width: UVal::Percent(100.0),
            height: UVal::Percent(100.0),
            background_color: Color::srgb(0.1, 0.1, 0.15),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            align_items: UAlignItems::Center,
            justify_content: UJustifyContent::Center,
            ..default()
        },
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::None,
            order: 1,
            ..default()
        },
    )
}

pub fn spawn_button(
    _text: &str,
    normal: Color,
    hovered: Color,
    pressed: Color,
) -> (UNode, ULayout, UInteraction, UInteractionColors) {
    (
        UNode {
            width: UVal::Px(200.0),
            height: UVal::Px(50.0),
            background_color: normal,
            border_radius: UCornerRadius::all(8.0),
            ..default()
        },
        ULayout {
            display: UDisplay::Flex,
            align_items: UAlignItems::Center,
            justify_content: UJustifyContent::Center,
            ..default()
        },
        UInteraction::default(),
        UInteractionColors {
            normal,
            hovered,
            pressed,
        },
    )
}

pub fn spawn_label(text: &str) -> UTextLabel {
    UTextLabel {
        text: text.into(),
        font_size: 48.0,
        color: Color::WHITE,
        ..default()
    }
}

pub fn spawn_small_label(text: &str) -> UTextLabel {
    UTextLabel {
        text: text.into(),
        font_size: 20.0,
        color: Color::WHITE,
        ..default()
    }
}
