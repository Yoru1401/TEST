use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game::camera::components::CameraMarker;

use crate::game::input::PlayerAction;
use crate::game::is_running;
use crate::game::player::components::PlayerMarker;

pub const MOVE_SPEED: f32 = 8.0;
pub const JUMP_FORCE: f32 = 12.0;
pub const GRAVITY: f32 = 30.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement.run_if(is_running));
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct DesiredVelocity {
    pub value: Vec3,
}

#[derive(Component, Default)]
pub struct JumpState {
    pub is_jumping: bool,
}

fn player_movement(
    time: Res<Time>,
    move_and_slide: MoveAndSlide,
    camera: Query<&Transform, With<CameraMarker>>,
    mut player: Query<
        (
            Entity,
            &mut Transform,
            &mut DesiredVelocity,
            &mut JumpState,
            &ActionState<PlayerAction>,
        ),
        (With<PlayerMarker>, Without<CameraMarker>),
    >,
) {
    let Ok(cam_transform) = camera.single() else {
        return;
    };

    let Ok((entity, mut transform, mut des_vel, mut jump_state, action)) = player.single_mut()
    else {
        return;
    };

    let move_axis = action.axis_pair(&PlayerAction::Move);
    let raw_input = Vec3::new(move_axis.x, 0.0, -move_axis.y);

    let forward = -cam_transform.forward().xz().extend(0.0).xzy();
    let right = cam_transform.right().xz().extend(0.0).xzy();
    let horizontal = (forward * raw_input.z + right * raw_input.x).normalize_or_zero() * MOVE_SPEED;

    des_vel.value.x = horizontal.x;
    des_vel.value.z = horizontal.z;
    des_vel.value.y -= GRAVITY * time.delta_secs();

    if action.pressed(&PlayerAction::Jump) && !jump_state.is_jumping {
        des_vel.value.y = JUMP_FORCE;
        jump_state.is_jumping = true;
    }

    if !action.pressed(&PlayerAction::Jump) {
        jump_state.is_jumping = false;
    }

    let MoveAndSlideOutput {
        position,
        projected_velocity,
    } = move_and_slide.move_and_slide(
        &Collider::sphere(0.5),
        transform.translation,
        transform.rotation,
        des_vel.value,
        time.delta(),
        &MoveAndSlideConfig::default(),
        &SpatialQueryFilter::from_excluded_entities([entity]),
        |_| MoveAndSlideHitResponse::Accept,
    );

    transform.translation = position;
    des_vel.value = projected_velocity;
}
