use crate::prelude::{Component, Vec3};

#[derive(Component, Debug)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct Motor {
    pub grounded: bool,
    pub hover_height: f32,
    pub jumps_remaining: i32,
    pub max_jumps: i32,
    pub desired_velocity: Vec3,
}

impl Default for Motor {
    fn default() -> Self {
        Self {
            grounded: false,
            hover_height: 1.5,
            jumps_remaining: 0,
            max_jumps: 1,
            desired_velocity: Vec3::ZERO,
        }
    }
}
