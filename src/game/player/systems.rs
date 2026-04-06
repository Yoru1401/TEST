use crate::prelude::*;
use avian3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game::camera::components::CameraMarker;
use crate::game::input::PlayerAction;
use crate::game::states::GameState;

use super::components::{Motor, PlayerMarker};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (motor_ground_probe, motor_hover, motor_move, player_jump),
        );
    }
}

fn motor_ground_probe(
    spatial_query: SpatialQuery,
    mut motor_q: Query<(Entity, &Transform, &mut Motor), With<PlayerMarker>>,
) {
    let Ok((entity, transform, mut motor)) = motor_q.single_mut() else {
        return;
    };

    let config = ShapeCastConfig {
        max_distance: motor.hover_height + 0.5,
        ..default()
    };
    let filter = SpatialQueryFilter::default().with_excluded_entities([entity]);

    let hit = spatial_query.cast_shape(
        &Collider::sphere(0.3),
        transform.translation,
        Quat::IDENTITY,
        Dir3::NEG_Y,
        &config,
        &filter,
    );

    let was_grounded = motor.grounded;
    motor.grounded = hit.is_some();

    if motor.grounded && !was_grounded {
        motor.jumps_remaining = motor.max_jumps;
    }
}

fn motor_hover(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut q: Query<(Entity, &Transform, &mut LinearVelocity, &Motor), With<PlayerMarker>>,
) {
    let Ok((entity, transform, mut linvel, motor)) = q.single_mut() else {
        return;
    };

    if !motor.grounded {
        linvel.0.y -= 9.81 * time.delta_secs();
        return;
    }

    let config = ShapeCastConfig {
        max_distance: motor.hover_height * 2.0,
        ..default()
    };
    let filter = SpatialQueryFilter::default().with_excluded_entities([entity]);

    if let Some(hit) = spatial_query.cast_shape(
        &Collider::sphere(0.3),
        transform.translation,
        Quat::IDENTITY,
        Dir3::NEG_Y,
        &config,
        &filter,
    ) {
        let error = motor.hover_height - hit.distance;
        let spring_strength = 120.0;
        let damping = 18.0;
        let vertical_vel = linvel.0.y;

        let spring_force = error * spring_strength - vertical_vel * damping;
        linvel.0.y += spring_force * time.delta_secs();
    }
}

fn player_jump(
    state: Res<State<GameState>>,
    mut player_query: Query<(&mut Motor, &mut LinearVelocity), With<PlayerMarker>>,
    action_state_query: Query<&ActionState<PlayerAction>>,
) {
    if state.get() != &GameState::Playground {
        return;
    }

    let Ok(action_state) = action_state_query.single() else {
        return;
    };

    if action_state.just_pressed(&PlayerAction::Jump) {
        let Ok((mut motor, mut linvel)) = player_query.single_mut() else {
            return;
        };

        if motor.jumps_remaining > 0 {
            linvel.0.y = 8.0;
            motor.jumps_remaining -= 1;
        }
    }
}

const MOVE_SPEED: f32 = 8.0;

fn motor_move(
    time: Res<Time>,
    mut q: Query<(&mut LinearVelocity, &mut Motor), With<PlayerMarker>>,
    camera: Query<&Transform, With<CameraMarker>>,
    action_state_query: Query<&ActionState<PlayerAction>>,
) {
    let Ok(cam_transform) = camera.single() else {
        return;
    };
    let Ok((mut linvel, mut motor)) = q.single_mut() else {
        return;
    };
    let Ok(action_state) = action_state_query.single() else {
        return;
    };

    let forward = cam_transform.forward().normalize_or_zero();
    let right = cam_transform.right().normalize_or_zero();

    let mut move_dir = Vec2::ZERO;

    if action_state.pressed(&PlayerAction::MoveForward) {
        move_dir.y += 1.0;
    }
    if action_state.pressed(&PlayerAction::MoveBackward) {
        move_dir.y -= 1.0;
    }
    if action_state.pressed(&PlayerAction::MoveLeft) {
        move_dir.x -= 1.0;
    }
    if action_state.pressed(&PlayerAction::MoveRight) {
        move_dir.x += 1.0;
    }

    if move_dir.length() > 0.1 {
        move_dir = move_dir.normalize();
        let cam_relative = move_dir.x * right.xz() + move_dir.y * forward.xz();
        motor.desired_velocity = Vec3::new(cam_relative.x, linvel.0.y, cam_relative.y) * MOVE_SPEED;
    } else {
        motor.desired_velocity = Vec3::new(0.0, linvel.0.y, 0.0);
    }

    let accel = 60.0;
    let current = linvel.0;
    let target = motor.desired_velocity;

    let delta = target - current;
    let accel_step = accel * time.delta_secs();
    let accel_vec = delta.clamp_length_max(accel_step);

    linvel.0 += accel_vec;
}
