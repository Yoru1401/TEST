use bevy::picking::prelude::Pointer;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{spawn_button, spawn_label, spawn_small_label, spawn_ui_screen};
use crate::game::input::GlobalAction;
use crate::game::setup::GameWorldSpawned;
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
    let (screen, mut node, layout, cam, camera) = spawn_ui_screen();
    node.background_color = Color::oklcha(0.0, 0.0, 0.0, 0.7);
    commands.spawn((Name::new("UI Camera"), cam, camera));
    commands
        .spawn((
            Name::new("Pause Menu Root"),
            PauseMenuRoot,
            screen,
            node,
            layout,
        ))
        .with_children(|parent| {
            parent.spawn(spawn_label("PAUSED"));
            let (n, l, i, c) = spawn_button(
                "Resume",
                Color::srgb(0.2, 0.8, 0.3),
                Color::srgb(0.3, 0.9, 0.4),
                Color::srgb(0.1, 0.6, 0.2),
            );
            parent
                .spawn((n, l, i, c))
                .observe(on_resume_click)
                .with_children(|p| {
                    p.spawn(spawn_small_label("Resume"));
                });
            let (n, l, i, c) = spawn_button(
                "Quit to Menu",
                Color::srgb(0.8, 0.2, 0.2),
                Color::srgb(0.9, 0.3, 0.3),
                Color::srgb(0.6, 0.1, 0.1),
            );
            parent
                .spawn((n, l, i, c))
                .observe(on_quit_click)
                .with_children(|p| {
                    p.spawn(spawn_small_label("Quit to Menu"));
                });
        });
}

fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuRoot>>) {
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
    }
}
