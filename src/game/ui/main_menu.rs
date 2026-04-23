use bevy::picking::prelude::Pointer;
use bevy::prelude::*;

use super::{spawn_button, spawn_label, spawn_small_label, spawn_ui_screen};
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

fn on_start_click(_trigger: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

fn spawn_main_menu(mut commands: Commands) {
    let (screen, node, layout, cam, camera) = spawn_ui_screen();
    commands.spawn((Name::new("UI Camera"), cam, camera));
    commands
        .spawn((
            Name::new("Main Menu Root"),
            MainMenuRoot,
            screen,
            node,
            layout,
        ))
        .with_children(|parent| {
            parent.spawn(spawn_label("BEVY PLAYGROUND"));
            let (n, l, i, c) = spawn_button(
                "Start Game",
                Color::srgb(0.2, 0.6, 0.9),
                Color::srgb(0.3, 0.7, 1.0),
                Color::srgb(0.1, 0.5, 0.8),
            );
            parent
                .spawn((n, l, i, c))
                .observe(on_start_click)
                .with_children(|p| {
                    p.spawn(spawn_small_label("Start Game"));
                });
        });
}

fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
    }
}
