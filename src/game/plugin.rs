use crate::game::{
    setup_playground, CameraPlugin, GameState, GrapplePlugin, InputPlugin, PhysicsPlugin,
    PlayerPlugin, UIPlugin,
};
use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Playground".into(),
                ..default()
            }),
            ..default()
        }));
        app.add_plugins((
            PhysicsPlugin,
            InputPlugin,
            PlayerPlugin,
            CameraPlugin,
            GrapplePlugin,
            UIPlugin,
        ));
        app.init_state::<GameState>();
        app.add_systems(OnEnter(GameState::Playing), setup_playground);
    }
}
