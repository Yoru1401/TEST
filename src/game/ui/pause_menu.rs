use bevy::picking::prelude::Pointer;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use univis_ui::prelude::*;
use univis_ui::widget::text_label::UTextLabel;

use crate::game::input::GlobalAction;
use crate::game::setup::functions::GameWorldSpawned;
use crate::game::states::GameState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), despawn_pause_menu)
            .add_systems(OnEnter(GameState::MainMenu), despawn_game_world)
            .add_systems(
                Update,
                handle_pause_input.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct PauseMenuRoot;

#[derive(Component)]
pub struct ResumeButton;

#[derive(Component)]
pub struct QuitButton;

fn handle_pause_input(
    action: Query<&ActionState<GlobalAction>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for action_state in &action {
        if action_state.just_pressed(&GlobalAction::Pause) {
            next_state.set(GameState::Paused);
            return;
        }
    }
}

fn on_resume_click(_trigger: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

fn on_quit_click(_trigger: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::MainMenu);
}

fn despawn_game_world(mut commands: Commands, spawned: Query<Entity, With<GameWorldSpawned>>) {
    for entity in &spawned {
        commands.entity(entity).despawn();
    }
}

fn spawn_pause_menu(mut commands: Commands) {
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
            Name::new("Pause Menu Root"),
            PauseMenuRoot,
            UScreenRoot,
            UNode {
                width: UVal::Percent(100.0),
                height: UVal::Percent(100.0),
                background_color: bevy::color::Color::oklcha(0.0, 0.0, 0.0, 0.7).into(),
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
                text: "PAUSED".into(),
                font_size: 48.0,
                color: bevy::color::Color::WHITE.into(),
                ..default()
            });

            parent
                .spawn((
                    UNode {
                        width: UVal::Px(200.0),
                        height: UVal::Px(50.0),
                        background_color: bevy::color::Color::srgb(0.2, 0.8, 0.3).into(),
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
                        normal: bevy::color::Color::srgb(0.2, 0.8, 0.3),
                        hovered: bevy::color::Color::srgb(0.3, 0.9, 0.4),
                        pressed: bevy::color::Color::srgb(0.1, 0.6, 0.2),
                    },
                    ResumeButton,
                ))
                .observe(on_resume_click)
                .with_children(|parent| {
                    parent.spawn(UTextLabel {
                        text: "Resume".into(),
                        font_size: 20.0,
                        color: bevy::color::Color::WHITE.into(),
                        ..default()
                    });
                });

            parent
                .spawn((
                    UNode {
                        width: UVal::Px(200.0),
                        height: UVal::Px(50.0),
                        background_color: bevy::color::Color::srgb(0.8, 0.2, 0.2).into(),
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
                        normal: bevy::color::Color::srgb(0.8, 0.2, 0.2),
                        hovered: bevy::color::Color::srgb(0.9, 0.3, 0.3),
                        pressed: bevy::color::Color::srgb(0.6, 0.1, 0.1),
                    },
                    QuitButton,
                ))
                .observe(on_quit_click)
                .with_children(|parent| {
                    parent.spawn(UTextLabel {
                        text: "Quit to Menu".into(),
                        font_size: 20.0,
                        color: bevy::color::Color::WHITE.into(),
                        ..default()
                    });
                });
        });
}

fn despawn_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenuRoot>>,
    ui_cameras: Query<Entity, With<Camera2d>>,
) {
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
    }
    for entity in &ui_cameras {
        commands.entity(entity).despawn();
    }
}
