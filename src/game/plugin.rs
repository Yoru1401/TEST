use crate::game::{
    setup_player, setup_playground, CameraInputPlugin, CameraPlugin, GameState, InputPlugin,
    PhysicsPlugin, PlayerPlugin, UIPlugin,
};
use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(bevy::window::WindowPlugin {
            primary_window: Some(bevy::window::Window {
                title: "Bevy Playground".into(),
                ..default()
            }),
            ..default()
        }));
        app.add_plugins(PhysicsPlugin);
        //app.add_plugins(GrapplePlugin);
        app.add_plugins(InputPlugin {}.build());
        app.add_plugins(CameraInputPlugin {}.build());
        app.add_plugins(PlayerPlugin);
        app.add_plugins(CameraPlugin);
        app.add_plugins(UIPlugin);

        app.init_state::<GameState>();

        app.add_systems(
            OnEnter(GameState::Playground),
            (setup_playground, setup_player),
        );
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
