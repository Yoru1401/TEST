use crate::game::states::GameState;
use bevy::prelude::*;
use univis_ui::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UnivisUiPlugin, UnivisBadgePlugin))
            .add_systems(OnEnter(GameState::Playground), setup_hud);
    }
}

fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Name::new("HUD Root"),
            UScreenRoot,
            UNode {
                width: UVal::Percent(100.0),
                height: UVal::Percent(100.0),
                background_color: bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
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
                text: "Controls:\nWASD/Arrows - Move\nSpace - Jump\nE - Spawn Cube".into(),
                font_size: 20.0,
                color: bevy::color::Color::srgb(0.8, 0.8, 0.8).into(),
                ..default()
            });
        });
}
