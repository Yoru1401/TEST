use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct GroundState {
    pub is_grounded: bool,
    pub ground_normal: Vec3,
}

impl Default for GroundState {
    fn default() -> Self {
        Self {
            is_grounded: false,
            ground_normal: Vec3::Y,
        }
    }
}
