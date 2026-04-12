use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game::camera::components::CameraMarker;
use crate::game::input::PlayerAction;
use crate::game::is_running;
use crate::game::player::components::{JumpState, PlayerMarker, WallState};

pub const MOVE_SPEED: f32 = 8.0;
pub const JUMP_FORCE: f32 = 12.0;
pub const GRAVITY: f32 = 30.0;

// Wall run tuning constants
/// Minimum horizontal speed toward the wall to trigger a wall run
pub const WALL_RUN_MIN_SPEED: f32 = 4.0;
/// Upward impulse given when first contacting a wall head-on
pub const WALL_RUN_UP_IMPULSE: f32 = 8.0;
/// Gravity scale while sliding down (friction from hands/feet)
pub const WALL_SLIDE_GRAVITY_SCALE: f32 = 0.15;
/// Once upward velocity drops below this, we switch to sliding
pub const WALL_SLIDE_THRESHOLD: f32 = 1.0;
/// Lateral speed boost when wall-jumping away
pub const WALL_JUMP_LATERAL_FORCE: f32 = 6.0;

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

fn player_movement(
    time: Res<Time>,
    move_and_slide: MoveAndSlide,
    spatial_query: SpatialQuery,
    camera: Query<&Transform, With<CameraMarker>>,
    mut player: Query<
        (
            Entity,
            &mut Transform,
            &Collider,
            &mut DesiredVelocity,
            &mut JumpState,
            &mut WallState,
            &ActionState<PlayerAction>,
        ),
        (With<PlayerMarker>, Without<CameraMarker>),
    >,
) {
    let Ok(cam_transform) = camera.single() else {
        return;
    };
    let Ok((entity, mut transform, collider, mut des_vel, mut jump_state, mut wall_state, action)) =
        player.single_mut()
    else {
        return;
    };

    let dt = time.delta_secs();

    // ── Input ────────────────────────────────────────────────────────────────
    let move_axis = action.axis_pair(&PlayerAction::Move);
    let raw_input = Vec3::new(move_axis.x, 0.0, -move_axis.y);

    let forward = -cam_transform.forward().xz().extend(0.0).xzy();
    let right = cam_transform.right().xz().extend(0.0).xzy();
    let horizontal = (forward * raw_input.z + right * raw_input.x).normalize_or_zero() * MOVE_SPEED;

    // ── Ground movement ──────────────────────────────────────────────────────
    des_vel.value.x = horizontal.x;
    des_vel.value.z = horizontal.z;

    // ── Wall interaction ─────────────────────────────────────────────────────
    let was_on_wall = wall_state.is_on_wall;

    if wall_state.is_on_wall && !jump_state.is_grounded {
        let normal = wall_state.wall_normal;

        // How much of our horizontal velocity is pointed INTO the wall (negative = into wall)
        let into_wall = des_vel.value.dot(-normal);

        // First frame touching wall: give upward impulse if we hit it with enough speed
        if !was_on_wall && into_wall >= WALL_RUN_MIN_SPEED {
            des_vel.value.y = WALL_RUN_UP_IMPULSE;
            wall_state.is_wall_sliding = false;
            wall_state.wall_run_timer = 0.0;
        }

        wall_state.wall_run_timer += dt;

        // Transition to sliding once upward velocity is too low
        if des_vel.value.y < WALL_SLIDE_THRESHOLD {
            wall_state.is_wall_sliding = true;
        }

        if wall_state.is_wall_sliding {
            // Reduced gravity — friction from hands/feet slowing the fall
            des_vel.value.y -= GRAVITY * WALL_SLIDE_GRAVITY_SCALE * dt;
            // Clamp so we don't accelerate endlessly downward on the wall
            des_vel.value.y = des_vel.value.y.max(-MOVE_SPEED * 0.5);
        } else {
            // Still running up — apply normal gravity so the arc feels natural
            des_vel.value.y -= GRAVITY * dt;
        }

        // Keep the player pressed against the wall (prevents drifting off)
        des_vel.value -= normal * into_wall.min(0.0);

        // ── Wall jump ────────────────────────────────────────────────────────
        if action.just_pressed(&PlayerAction::Jump) {
            // Launch away from the wall and upward
            des_vel.value = normal * WALL_JUMP_LATERAL_FORCE
                + Vec3::Y * JUMP_FORCE
                // Preserve some of the along-wall momentum
                + horizontal * 0.4;
            jump_state.is_jumping = true;
            wall_state.is_on_wall = false;
            wall_state.is_wall_sliding = false;
            wall_state.wall_run_timer = 0.0;
        }
    } else {
        // Not on a wall — normal gravity
        des_vel.value.y -= GRAVITY * dt;
    }

    // ── Ground jump ──────────────────────────────────────────────────────────
    if action.pressed(&PlayerAction::Jump) && !jump_state.is_jumping && jump_state.is_grounded {
        des_vel.value.y = JUMP_FORCE;
        jump_state.is_jumping = true;
    }

    if !action.pressed(&PlayerAction::Jump) {
        jump_state.is_jumping = false;
    }

    // ── Ground check ─────────────────────────────────────────────────────────
    let ground_hit = spatial_query.cast_shape(
        &collider,
        transform.translation,
        Quat::IDENTITY,
        Dir3::NEG_Y,
        &ShapeCastConfig {
            max_distance: 1.3,
            ..default()
        },
        &SpatialQueryFilter::from_excluded_entities([entity]),
    );
    jump_state.is_grounded = ground_hit.is_some();

    // Reset wall state each frame; the callback below re-sets it if still touching
    wall_state.is_on_wall = false;
    wall_state.wall_normal = Vec3::ZERO;

    // ── Move and slide ────────────────────────────────────────────────────────
    let MoveAndSlideOutput {
        position,
        projected_velocity,
    } = move_and_slide.move_and_slide(
        &collider,
        transform.translation,
        transform.rotation,
        des_vel.value,
        time.delta(),
        &MoveAndSlideConfig::default(),
        &SpatialQueryFilter::from_excluded_entities([entity]),
        |hit| {
            // Only treat roughly-vertical surfaces as walls (not ceiling/floor)
            let up_dot = hit.normal.dot(Vec3::Y).abs();
            if up_dot < 0.4 {
                wall_state.is_on_wall = true;
                wall_state.wall_normal = **hit.normal;
            }
            MoveAndSlideHitResponse::Accept
        },
    );

    transform.translation = position;
    des_vel.value = projected_velocity;
}
