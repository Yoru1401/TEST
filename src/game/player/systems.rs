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

pub const WALL_RUN_MIN_SPEED: f32 = 4.0;
pub const WALL_RUN_UP_IMPULSE: f32 = 8.0;
pub const WALL_SLIDE_GRAVITY_SCALE: f32 = 0.15;
pub const WALL_SLIDE_THRESHOLD: f32 = 1.0;
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
    spatial_query: SpatialQuery,
    camera: Query<&Transform, With<CameraMarker>>,
    move_and_slide: MoveAndSlide,
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
    let filter = SpatialQueryFilter::from_excluded_entities([entity]);

    // ── Input ────────────────────────────────────────────────────────────────
    let move_axis = action.axis_pair(&PlayerAction::Move);
    let raw_input = Vec3::new(move_axis.x, 0.0, -move_axis.y);

    let forward = -cam_transform.forward().xz().extend(0.0).xzy();
    let right = cam_transform.right().xz().extend(0.0).xzy();
    let horizontal = (forward * raw_input.z + right * raw_input.x).normalize_or_zero() * MOVE_SPEED;

    des_vel.value.x = horizontal.x;
    des_vel.value.z = horizontal.z;

    // ── Ground check (must run before jump/gravity logic) ────────────────────
    let ground_hit = spatial_query.cast_shape(
        &collider,
        transform.translation,
        Quat::IDENTITY,
        Dir3::NEG_Y,
        &ShapeCastConfig {
            max_distance: 1.3,
            ..default()
        },
        &filter,
    );
    jump_state.is_grounded = ground_hit.is_some();

    // ── Wall interaction ─────────────────────────────────────────────────────
    if wall_state.is_on_wall && !jump_state.is_grounded {
        let normal = wall_state.wall_normal;

        if des_vel.value.y < WALL_SLIDE_THRESHOLD {
            wall_state.is_wall_sliding = true;
        }

        if wall_state.is_wall_sliding {
            des_vel.value.y -= GRAVITY * WALL_SLIDE_GRAVITY_SCALE * dt;
            des_vel.value.y = des_vel.value.y.max(-MOVE_SPEED * 0.5);
        } else {
            des_vel.value.y -= GRAVITY * dt;
        }

        // Wall jump
        if action.just_pressed(&PlayerAction::Jump) {
            des_vel.value =
                normal * WALL_JUMP_LATERAL_FORCE + Vec3::Y * JUMP_FORCE + horizontal * 0.4;
            jump_state.is_jumping = true;
            wall_state.is_on_wall = false;
            wall_state.is_wall_sliding = false;
        }
    } else {
        des_vel.value.y -= GRAVITY * dt;
    }

    // ── Ground jump ───────────────────────────────────────────────────────────
    if action.just_pressed(&PlayerAction::Jump) && jump_state.is_grounded {
        des_vel.value.y = JUMP_FORCE;
        jump_state.is_jumping = true;
    }
    if jump_state.is_grounded && des_vel.value.y <= 0.0 {
        jump_state.is_jumping = false;
    }

    // Snapshot last frame's wall state before resetting
    wall_state.is_on_wall = false;
    wall_state.wall_normal = Vec3::ZERO;

    // ── Move and slide ────────────────────────────────────────────────────────
    const SKIN_WIDTH: f32 = 0.01;

    let config = MoveAndSlideConfig::default();
    let mut velocity = des_vel.value;
    let mut position = transform.translation;
    let mut time_left = dt;

    let depenetration_offset = move_and_slide.depenetrate(
        &collider,
        position,
        transform.rotation,
        &(&config).into(),
        &filter,
    );
    position += depenetration_offset;

    for _ in 0..config.move_and_slide_iterations {
        let sweep = time_left * velocity;
        let length = sweep.length();

        if length < 1e-4 {
            break;
        }

        let vel_dir = Dir3::new(sweep / length).unwrap();

        let hit = spatial_query.cast_shape(
            &collider,
            position,
            transform.rotation,
            vel_dir,
            &ShapeCastConfig {
                max_distance: length,
                ..default()
            },
            &filter,
        );

        let Some(sweep_hit) = hit else {
            position += sweep;
            break;
        };

        let fraction = sweep_hit.distance / length;
        position += *vel_dir * (sweep_hit.distance - SKIN_WIDTH).max(0.0);
        time_left *= 1.0 - fraction;

        let hit_normal: Vec3 = sweep_hit.normal1.into();
        let up_dot = hit_normal.dot(Vec3::Y);

        if up_dot < 0.7 && up_dot > -0.3 {
            wall_state.is_on_wall = true;
            wall_state.wall_normal = hit_normal;
        }

        let dot = velocity.dot(hit_normal);
        if dot < 0.0 {
            velocity -= dot * hit_normal;
        }

        if time_left < 1e-6 {
            break;
        }
    }

    let depenetration_offset = move_and_slide.depenetrate(
        &collider,
        position,
        transform.rotation,
        &(&config).into(),
        &filter,
    );
    position += depenetration_offset;

    transform.translation = position;
    des_vel.value = velocity;
}
