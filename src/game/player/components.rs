use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component, Default)]
pub struct WallState {
    pub is_on_wall: bool,
}

#[derive(Component, Default)]
pub struct JumpState {
    pub is_jumping: bool,
    pub is_grounded: bool,
}
