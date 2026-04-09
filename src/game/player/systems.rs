use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game::input::PlayerAction;
use crate::game::player::components::PlayerMarker;
use crate::game::states::GameState;

pub const MOVE_SPEED: f32 = 8.0;
pub const JUMP_FORCE: f32 = 12.0;
pub const GRAVITY: f32 = 30.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement.in_set(MovementSystemSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MovementSystemSet;

#[derive(Component, Default, Deref, DerefMut)]
pub struct DesiredVelocity {
    pub value: Vec3,
}

#[derive(Component, Default)]
pub struct JumpState {
    pub is_jumping: bool,
}

fn player_movement(
    state: Res<State<GameState>>,
    time: Res<Time>,
    move_and_slide: MoveAndSlide,
    mut player: Query<
        (
            Entity,
            &mut Transform,
            &mut DesiredVelocity,
            &mut JumpState,
            &ActionState<PlayerAction>,
        ),
        With<PlayerMarker>,
    >,
) {
    if state.get() != &GameState::Playground {
        return;
    }

    let Ok((entity, mut transform, mut des_vel, mut jump_state, action)) = player.single_mut()
    else {
        return;
    };

    let mut input_dir = Vec3::ZERO;
    if action.pressed(&PlayerAction::MoveForward) {
        input_dir.z -= 1.0;
    }
    if action.pressed(&PlayerAction::MoveBackward) {
        input_dir.z += 1.0;
    }
    if action.pressed(&PlayerAction::MoveLeft) {
        input_dir.x -= 1.0;
    }
    if action.pressed(&PlayerAction::MoveRight) {
        input_dir.x += 1.0;
    }

    if input_dir.length_squared() > 0.0 {
        input_dir = input_dir.normalize() * MOVE_SPEED;
    }
    des_vel.value.x = input_dir.x;
    des_vel.value.z = input_dir.z;

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

    if des_vel.value.y <= 0.1 && position.y <= 0.6 {
        des_vel.value.y = -GRAVITY * time.delta_secs().min(0.5);
    } else if des_vel.value.y > 0.1 {
        des_vel.value.y -= GRAVITY * time.delta_secs();
    }
}
