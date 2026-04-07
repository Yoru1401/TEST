use crate::game::states::GameState;
use crate::game::{JumpAbilityType, JumpInfo};
use crate::prelude::*;
use univis_ui::prelude::*;
use univis_ui::widget::text_label::UTextLabel;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UnivisUiPlugin)
            .add_systems(OnEnter(GameState::Playground), setup_hud)
            .add_systems(Last, update_jump_ui);
    }
}

fn setup_hud(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Camera"),
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::None,
            order: 1,
            ..default()
        },
    ));

    commands
        .spawn((
            Name::new("HUD Root"),
            UScreenRoot,
            UNode {
                width: UVal::Percent(100.0),
                height: UVal::Percent(100.0),
                background_color: bevy::color::Color::NONE.into(),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Start,
                justify_content: UJustifyContent::Start,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    UNode {
                        width: UVal::Content,
                        height: UVal::Content,
                        background_color: bevy::color::Color::oklcha(0.4911, 0.2877, 273.17, 0.30)
                            .into(),
                        border_radius: UCornerRadius::all(8.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        align_items: UAlignItems::Start,
                        justify_content: UJustifyContent::Start,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(UTextLabel {
                        text: "Bevy Playground".into(),
                        font_size: 32.0,
                        color: bevy::color::Color::WHITE.into(),
                        ..default()
                    });

                    parent.spawn(UTextLabel {
                        text: "Controls:\nWASD/Arrows - Move\nSpace - Jump\nQ - Switch Jump Type"
                            .into(),
                        font_size: 20.0,
                        color: bevy::color::Color::srgb(0.8, 0.8, 0.8).into(),
                        ..default()
                    });
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Jump Info"),
                    UNode {
                        width: UVal::Content,
                        height: UVal::Content,
                        background_color: bevy::color::Color::oklcha(0.4911, 0.2877, 273.17, 0.30)
                            .into(),
                        border_radius: UCornerRadius::all(8.0),
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        align_items: UAlignItems::Start,
                        justify_content: UJustifyContent::Start,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("JumpTypeLabel"),
                        UTextLabel {
                            text: "Jump Type: Normal".into(),
                            font_size: 24.0,
                            color: bevy::color::Color::WHITE.into(),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        Name::new("JumpChargesLabel"),
                        UTextLabel {
                            text: "Charges: 1".into(),
                            font_size: 24.0,
                            color: bevy::color::Color::WHITE.into(),
                            ..default()
                        },
                    ));
                });
        });
}

fn update_jump_ui(mut jump_type_query: Query<(&Name, &mut UTextLabel)>, jump_info: Res<JumpInfo>) {
    for (name, mut text_label) in jump_type_query.iter_mut() {
        if name.as_str() == "JumpTypeLabel" {
            let type_name = match jump_info.jump_type {
                JumpAbilityType::Normal => "Normal",
                JumpAbilityType::Charge => "Charge",
                JumpAbilityType::Multi => "Multi",
            };
            text_label.text = format!("Jump Type: {}", type_name);
        } else if name.as_str() == "JumpChargesLabel" {
            text_label.text = format!("Charges: {}", jump_info.charges);
        }
    }
}
