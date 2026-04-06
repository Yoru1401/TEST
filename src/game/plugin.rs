use crate::game::{setup_playground, CameraPlugin, GameState, InputPlugin, PlayerPlugin, UIPlugin};
use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(bevy::window::WindowPlugin {
                primary_window: Some(bevy::window::Window {
                    title: "Bevy Playground".into(),
                    ..default()
                }),
                ..default()
            }),
            avian3d::prelude::PhysicsPlugins::default(),
            InputPlugin {}.build(),
            UIPlugin,
            PlayerPlugin,
            CameraPlugin,
        ));

        app.init_state::<GameState>();

        app.add_systems(OnEnter(GameState::Playground), setup_playground);
        app.add_systems(PostUpdate, enter_playground);
    }
}

fn enter_playground(
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if current_state.get() == &GameState::MainMenu {
        next_state.set(GameState::Playground);
    }
}
