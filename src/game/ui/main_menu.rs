use bevy::picking::prelude::Pointer;
use bevy::prelude::*;
use univis_ui::prelude::*;
use univis_ui::widget::text_label::UTextLabel;

use crate::game::states::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
    }
}

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct StartButton;

fn on_start_click(_trigger: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

fn spawn_main_menu(mut commands: Commands) {
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
            Name::new("Main Menu Root"),
            MainMenuRoot,
            UScreenRoot,
            UNode {
                width: UVal::Percent(100.0),
                height: UVal::Percent(100.0),
                background_color: bevy::color::Color::srgb(0.1, 0.1, 0.15).into(),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(UTextLabel {
                text: "BEVY PLAYGROUND".into(),
                font_size: 48.0,
                color: bevy::color::Color::WHITE.into(),
                ..default()
            });

            parent
                .spawn((
                    UNode {
                        width: UVal::Px(200.0),
                        height: UVal::Px(50.0),
                        background_color: bevy::color::Color::srgb(0.2, 0.6, 0.9).into(),
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
                        normal: bevy::color::Color::srgb(0.2, 0.6, 0.9),
                        hovered: bevy::color::Color::srgb(0.3, 0.7, 1.0),
                        pressed: bevy::color::Color::srgb(0.1, 0.5, 0.8),
                    },
                    StartButton,
                ))
                .observe(on_start_click)
                .with_children(|parent| {
                    parent.spawn(UTextLabel {
                        text: "Start Game".into(),
                        font_size: 20.0,
                        color: bevy::color::Color::WHITE.into(),
                        ..default()
                    });
                });
        });
}

fn despawn_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuRoot>>,
    ui_cameras: Query<Entity, With<Camera2d>>,
) {
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
    }
    for entity in &ui_cameras {
        commands.entity(entity).despawn();
    }
}
