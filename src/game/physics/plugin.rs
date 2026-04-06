use crate::game::physics::systems::{handle_jump, handle_movement, spawn_cube_on_action};
use avian3d::prelude::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());

        app.add_systems(Update, handle_movement);
        app.add_systems(Update, handle_jump);
        app.add_systems(Update, spawn_cube_on_action);
    }
}
