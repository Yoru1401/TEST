use crate::game::input::PlayerAction;
use crate::game::physics::components::PlayerMarker;
use crate::game::states::GameState;
use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const PLAYER_JUMP_FORCE: f32 = 8.0;
const PLAYER_MOVE_FORCE: f32 = 15.0;

pub fn handle_movement(
    state: Res<State<GameState>>,
    player_query: Query<(Entity, &ActionState<PlayerAction>), With<PlayerMarker>>,
    mut force_query: Query<Forces>,
) {
    if state.get() != &GameState::Playground {
        return;
    }

    for (entity, action_state) in &player_query {
        let mut direction = Vec2::ZERO;

        if action_state.pressed(&PlayerAction::MoveForward) {
            direction.y += 1.0;
        }
        if action_state.pressed(&PlayerAction::MoveBackward) {
            direction.y -= 1.0;
        }
        if action_state.pressed(&PlayerAction::MoveLeft) {
            direction.x -= 1.0;
        }
        if action_state.pressed(&PlayerAction::MoveRight) {
            direction.x += 1.0;
        }

        if direction != Vec2::ZERO {
            let direction = direction.normalize();
            let force = Vec3::new(
                direction.x * PLAYER_MOVE_FORCE,
                0.0,
                direction.y * PLAYER_MOVE_FORCE,
            );

            if let Ok(mut forces) = force_query.get_mut(entity) {
                forces.apply_force(force);
            }
        }
    }
}

pub fn handle_jump(
    state: Res<State<GameState>>,
    player_query: Query<(Entity, &ActionState<PlayerAction>), With<PlayerMarker>>,
    mut force_query: Query<Forces>,
) {
    if state.get() != &GameState::Playground {
        return;
    }

    for (entity, action_state) in &player_query {
        if action_state.just_pressed(&PlayerAction::Jump) {
            if let Ok(mut forces) = force_query.get_mut(entity) {
                forces.apply_linear_impulse(Vec3::Y * PLAYER_JUMP_FORCE);
            }
        }
    }
}

pub fn spawn_cube_on_action(
    state: Res<State<GameState>>,
    mut commands: Commands,
    player_query: Query<(&Transform, &ActionState<PlayerAction>), With<PlayerMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if state.get() != &GameState::Playground {
        return;
    }

    for (transform, action_state) in &player_query {
        if action_state.just_pressed(&PlayerAction::SpawnCube) {
            let spawn_position = transform.translation + Vec3::Y * 2.0 + Vec3::X * 2.0;

            let colors = [
                Color::srgb(0.2, 0.8, 0.2),
                Color::srgb(0.2, 0.2, 0.8),
                Color::srgb(0.8, 0.8, 0.2),
                Color::srgb(0.8, 0.2, 0.8),
            ];

            let color = colors[fastrand::usize(0..colors.len())];

            commands.spawn((
                Name::new("Spawned Cube"),
                DespawnOnExit(GameState::Playground),
                RigidBody::Dynamic,
                Collider::cuboid(1.0, 1.0, 1.0),
                Restitution::new(0.7),
                Friction::new(0.3),
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(StandardMaterial::from_color(color))),
                Transform::from_translation(spawn_position),
            ));
        }
    }
}
