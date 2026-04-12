use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component, Default)]
pub struct WallState {
    pub is_on_wall: bool,
    pub wall_normal: Vec3,     // Direction away from wall surface
    pub wall_run_timer: f32,   // How long we've been on this wall
    pub is_wall_sliding: bool, // True when sliding down (low upward velocity)
}

#[derive(Component, Default)]
pub struct JumpState {
    pub is_jumping: bool,
    pub is_grounded: bool,
}
