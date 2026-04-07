use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game::camera::components::CameraMarker;
use crate::game::input::PlayerAction;
use crate::game::states::GameState;

use super::abilities::{JumpAbilityState, JumpAbilityType};
use super::components::PlayerMarker;

#[derive(Component)]
pub struct CharacterMotor {
    pub is_grounded: bool,
    pub target_height_above_ground: f32,
    pub desired_movement: Vec3,
    pub desired_horizontal_speed: f32,
    pub charges: i32,
    pub spring_disabled_timer: f32,
}

impl Default for CharacterMotor {
    fn default() -> Self {
        Self {
            is_grounded: false,
            target_height_above_ground: 1.5,
            desired_movement: Vec3::ZERO,
            desired_horizontal_speed: 8.0,
            charges: 1,
            spring_disabled_timer: 0.0,
        }
    }
}

#[derive(Component)]
pub enum InputSource {
    PlayerControlled,
    AiControlled(Vec3),
}

#[derive(Resource, Default)]
pub struct JumpInfo {
    pub jump_type: super::abilities::JumpAbilityType,
    pub charges: i32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                update_spring_timer,
                probe_for_ground,
                calculate_desired_movement,
            )
                .chain(),
        );

        app.add_systems(PreUpdate, apply_forces);
    }
}

const PROBE_SPHERE_RADIUS: f32 = 0.3;
const GROUND_PROBE_DISTANCE: f32 = 2.0;

const HOVER_SPRING_STRENGTH: f32 = 120.0;
const HOVER_DAMPING: f32 = 12.0;
const MOVEMENT_ACCELERATION: f32 = 30.0;
const MAX_MOVEMENT_FORCE: f32 = 15.0;

const JUMP_IMPULSE: f32 = 8.0;
const SPRING_DISABLE_TIME: f32 = 0.15;

fn update_spring_timer(
    time: Res<Time>,
    mut character_query: Query<&mut CharacterMotor, With<PlayerMarker>>,
) {
    let Ok(mut motor) = character_query.single_mut() else {
        return;
    };

    if motor.spring_disabled_timer > 0.0 {
        motor.spring_disabled_timer -= time.delta_secs();
    }
}

fn probe_for_ground(
    spatial_query: SpatialQuery,
    mut character_query: Query<
        (Entity, &Transform, &mut CharacterMotor),
        (With<PlayerMarker>, With<InputSource>),
    >,
) {
    let Ok((character_entity, character_transform, mut character_motor)) =
        character_query.single_mut()
    else {
        return;
    };

    let shape_cast_max_distance =
        character_motor.target_height_above_ground + GROUND_PROBE_DISTANCE;
    let shape_cast_config = ShapeCastConfig::from_max_distance(shape_cast_max_distance);
    let shape_cast_filter =
        SpatialQueryFilter::default().with_excluded_entities([character_entity]);

    let ground_check_hit = spatial_query.cast_shape(
        &Collider::sphere(PROBE_SPHERE_RADIUS),
        character_transform.translation,
        Quat::IDENTITY,
        Dir3::NEG_Y,
        &shape_cast_config,
        &shape_cast_filter,
    );

    let was_grounded = character_motor.is_grounded;
    character_motor.is_grounded = ground_check_hit.is_some();

    if character_motor.is_grounded && !was_grounded {
        character_motor.charges = 1;
    }
}

fn calculate_desired_movement(
    state: Res<State<GameState>>,
    mut character_query: Query<(&mut CharacterMotor, &InputSource), With<PlayerMarker>>,
    camera_query: Query<&Transform, With<CameraMarker>>,
    action_state_query: Query<&ActionState<PlayerAction>, With<PlayerMarker>>,
) {
    if state.get() != &GameState::Playground {
        return;
    }

    let Ok((mut character_motor, input_source)) = character_query.single_mut() else {
        return;
    };

    let horizontal_movement = match input_source {
        InputSource::PlayerControlled => {
            let Ok(camera_transform) = camera_query.single() else {
                return;
            };
            let Ok(action_state) = action_state_query.single() else {
                return;
            };
            calculate_horizontal_input(action_state, camera_transform)
        }
        InputSource::AiControlled(ai_direction) => *ai_direction,
    };

    let vertical_movement = if character_motor.is_grounded {
        0.0
    } else {
        -9.81
    };

    character_motor.desired_movement = Vec3::new(
        horizontal_movement.x,
        vertical_movement,
        horizontal_movement.z,
    );
}

fn calculate_horizontal_input(
    action_state: &ActionState<PlayerAction>,
    camera_transform: &Transform,
) -> Vec3 {
    let camera_forward = camera_transform.forward().normalize_or_zero();
    let camera_right = camera_transform.right().normalize_or_zero();

    let forward_flat = Vec3::new(camera_forward.x, 0.0, camera_forward.z).normalize();
    let right_flat = Vec3::new(camera_right.x, 0.0, camera_right.z).normalize();

    let mut movement_input = Vec3::ZERO;

    if action_state.pressed(&PlayerAction::MoveForward) {
        movement_input += forward_flat;
    }
    if action_state.pressed(&PlayerAction::MoveBackward) {
        movement_input -= forward_flat;
    }
    if action_state.pressed(&PlayerAction::MoveLeft) {
        movement_input -= right_flat;
    }
    if action_state.pressed(&PlayerAction::MoveRight) {
        movement_input += right_flat;
    }

    if movement_input.length() > 0.01 {
        movement_input.normalize()
    } else {
        Vec3::ZERO
    }
}

fn apply_forces(
    state: Res<State<GameState>>,
    spatial_query: SpatialQuery,
    mut character_query: Query<
        (Entity, &Transform, &mut CharacterMotor, Forces),
        (With<PlayerMarker>, With<InputSource>),
    >,
    mut jump_info: ResMut<JumpInfo>,
    jump_state_query: Query<&JumpAbilityState, With<PlayerMarker>>,
    action_state_query: Query<&ActionState<PlayerAction>, With<PlayerMarker>>,
) {
    if state.get() != &GameState::Playground {
        return;
    }

    let Ok((character_entity, character_transform, mut character_motor, mut forces)) =
        character_query.single_mut()
    else {
        return;
    };

    let jump_state = jump_state_query.single().ok();
    let action_state = action_state_query.single().ok();

    jump_info.jump_type = jump_state
        .map(|s| s.current_type)
        .unwrap_or(JumpAbilityType::Normal);
    jump_info.charges = character_motor.charges;

    let current_velocity = forces.linear_velocity();
    let current_velocity_y = current_velocity.y;
    let current_horizontal_velocity = Vec3::new(current_velocity.x, 0.0, current_velocity.z);
    let desired_movement = character_motor.desired_movement;
    let is_grounded = character_motor.is_grounded;
    let spring_disabled_timer = character_motor.spring_disabled_timer;
    let target_height = character_motor.target_height_above_ground;
    let desired_horizontal_speed = character_motor.desired_horizontal_speed;

    let mut jump_force_applied = false;

    if let (Some(action_state), Some(_jump_state)) = (action_state, jump_state) {
        if action_state.just_pressed(&PlayerAction::Jump) && character_motor.charges > 0 {
            jump_force_applied = true;
            character_motor.spring_disabled_timer = SPRING_DISABLE_TIME;
            character_motor.charges -= 1;
            forces.apply_linear_impulse(Vec3::Y * JUMP_IMPULSE);
        }
    }

    let spring_force = if is_grounded && spring_disabled_timer <= 0.0 && !jump_force_applied {
        let shape_cast_config = ShapeCastConfig {
            max_distance: target_height * 2.0,
            ..default()
        };
        let shape_cast_filter =
            SpatialQueryFilter::default().with_excluded_entities([character_entity]);

        if let Some(ground_hit) = spatial_query.cast_shape(
            &Collider::sphere(PROBE_SPHERE_RADIUS),
            character_transform.translation,
            Quat::IDENTITY,
            Dir3::NEG_Y,
            &shape_cast_config,
            &shape_cast_filter,
        ) {
            let height_error = target_height - ground_hit.distance;
            let spring = height_error * HOVER_SPRING_STRENGTH;
            let damping = -current_velocity_y * HOVER_DAMPING;
            Vec3::Y * (spring + damping)
        } else {
            Vec3::ZERO
        }
    } else {
        Vec3::ZERO
    };

    let desired_velocity = desired_movement * desired_horizontal_speed;
    let velocity_error = desired_velocity - current_horizontal_velocity;
    let mut acceleration_force = velocity_error * MOVEMENT_ACCELERATION;

    let horizontal_force_magnitude = Vec2::new(acceleration_force.x, acceleration_force.z).length();
    if horizontal_force_magnitude > MAX_MOVEMENT_FORCE {
        let scale = MAX_MOVEMENT_FORCE / horizontal_force_magnitude;
        acceleration_force.x *= scale;
        acceleration_force.z *= scale;
    }

    let movement_force = Vec3::new(
        acceleration_force.x,
        desired_movement.y * 2.0,
        acceleration_force.z,
    );

    forces.apply_force(spring_force + movement_force);
}
