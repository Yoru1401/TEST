use crate::game::camera::components::CameraMarker;
use crate::game::player::components::PlayerMarker;
use crate::game::states::GameState;
use crate::prelude::*;
use avian3d::prelude::{Collider, ShapeCastConfig, SpatialQuery, SpatialQueryFilter};
use bevy::input::mouse::AccumulatedMouseMotion;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraRotation::default());
        app.add_systems(Update, update_camera);
    }
}

#[derive(Resource)]
pub struct CameraRotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for CameraRotation {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.3,
        }
    }
}

const CAMERA_DISTANCE: f32 = 12.0;
const CAMERA_HEIGHT: f32 = 5.0;
const MOUSE_SENSITIVITY: f32 = 0.003;
const CAMERA_COLLISION_RADIUS: f32 = 0.3;
const CAMERA_MIN_DISTANCE: f32 = 6.0;

fn update_camera(
    state: Res<State<GameState>>,
    mut camera_rot: ResMut<CameraRotation>,
    player_query: Query<&Transform, (With<PlayerMarker>, Without<CameraMarker>)>,
    mut camera_query: Query<&mut Transform, With<CameraMarker>>,
    accumulated_motion: Res<AccumulatedMouseMotion>,
    spatial_query: SpatialQuery,
) {
    if state.get() != &GameState::Playground {
        return;
    }

    if accumulated_motion.delta != Vec2::ZERO {
        camera_rot.yaw -= accumulated_motion.delta.x * MOUSE_SENSITIVITY;
        camera_rot.pitch -= accumulated_motion.delta.y * MOUSE_SENSITIVITY;
        camera_rot.pitch = camera_rot.pitch.clamp(-1.2, 1.2);
    }

    let player_pos = match player_query.single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    let offset = Vec3::new(
        camera_rot.yaw.cos() * camera_rot.pitch.cos() * CAMERA_DISTANCE,
        camera_rot.pitch.sin() * CAMERA_DISTANCE + CAMERA_HEIGHT,
        camera_rot.yaw.sin() * camera_rot.pitch.cos() * CAMERA_DISTANCE,
    );

    let desired_pos = player_pos + offset;
    let final_pos = apply_camera_collision(player_pos, desired_pos, &spatial_query);

    if let Ok(mut camera_transform) = camera_query.single_mut() {
        camera_transform.translation = final_pos;
        camera_transform.look_at(player_pos, Vec3::Y);
    }
}

fn apply_camera_collision(
    player_pos: Vec3,
    desired_pos: Vec3,
    spatial_query: &SpatialQuery,
) -> Vec3 {
    let direction = desired_pos - player_pos;
    let distance = direction.length();

    if distance < CAMERA_MIN_DISTANCE {
        return player_pos + direction.normalize() * CAMERA_MIN_DISTANCE;
    }

    let direction_normalized = direction / distance;
    let direction_dir3 = Dir3::new(direction_normalized).unwrap_or(Dir3::Y);
    let config = ShapeCastConfig::from_max_distance(distance);

    if let Some(hit) = spatial_query.cast_shape(
        &Collider::sphere(CAMERA_COLLISION_RADIUS),
        player_pos,
        Quat::default(),
        direction_dir3,
        &config,
        &SpatialQueryFilter::default(),
    ) {
        let adjusted_distance = (hit.distance - CAMERA_COLLISION_RADIUS).max(CAMERA_MIN_DISTANCE);
        return player_pos + *direction_dir3 * adjusted_distance;
    }

    desired_pos
}
