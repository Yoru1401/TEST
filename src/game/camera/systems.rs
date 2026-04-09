use avian3d::prelude::{Collider, SpatialQuery, SpatialQueryFilter};
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;

use crate::game::camera::components::CameraMarker;
use crate::game::player::components::PlayerMarker;
use crate::game::states::GameState;

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

const CAM_DIST: f32 = 12.0;
const CAM_HEIGHT: f32 = 5.0;
const MOUSE_SENS: f32 = 0.003;
const CAM_COLLISION: f32 = 0.5;
const CAM_MIN_DIST: f32 = 2.0;

fn update_camera(
    state: Res<State<GameState>>,
    mut rot: ResMut<CameraRotation>,
    player: Query<(Entity, &Transform), (With<PlayerMarker>, Without<CameraMarker>)>,
    mut camera: Query<&mut Transform, With<CameraMarker>>,
    mouse: Res<AccumulatedMouseMotion>,
    spatial: SpatialQuery,
) {
    if state.get() != &GameState::Playground {
        return;
    }

    if mouse.delta != Vec2::ZERO {
        rot.yaw -= mouse.delta.x * MOUSE_SENS;
        rot.pitch -= mouse.delta.y * MOUSE_SENS;
        rot.pitch = rot.pitch.clamp(-1.2, 1.2);
    }

    let (player_entity, player_transform) = match player.single() {
        Ok(p) => p,
        Err(_) => return,
    };

    let player_pos = player_transform.translation;

    let offset = Vec3::new(
        rot.yaw.cos() * rot.pitch.cos() * CAM_DIST,
        rot.pitch.sin() * CAM_DIST + CAM_HEIGHT,
        rot.yaw.sin() * rot.pitch.cos() * CAM_DIST,
    );

    let desired = player_pos + offset;
    let final_pos = resolve_camera_collision(player_pos, desired, player_entity, &spatial);

    if let Ok(mut cam_t) = camera.single_mut() {
        cam_t.translation = final_pos;
        cam_t.look_at(player_pos, Vec3::Y);
    }
}

fn resolve_camera_collision(
    player_pos: Vec3,
    desired: Vec3,
    player_entity: Entity,
    spatial: &SpatialQuery,
) -> Vec3 {
    let dir = desired - player_pos;
    let dist = dir.length();

    if dist < CAM_MIN_DIST {
        return player_pos + dir.normalize() * CAM_MIN_DIST;
    }

    let dir_n = dir / dist;
    let dir3 = Dir3::new(dir_n).unwrap_or_else(|_| Dir3::Y);

    let start_offset = CAM_COLLISION + 0.1;
    let start_pos = player_pos + dir_n * start_offset;
    let cast_dist = dist - start_offset;

    let filter = SpatialQueryFilter::from_excluded_entities([player_entity]);

    if let Some(hit) = spatial.cast_shape(
        &Collider::sphere(CAM_COLLISION),
        start_pos,
        Quat::IDENTITY,
        dir3,
        &avian3d::prelude::ShapeCastConfig::from_max_distance(cast_dist),
        &filter,
    ) {
        let adj_dist = (start_offset + hit.distance - CAM_COLLISION).max(CAM_MIN_DIST);
        return player_pos + dir_n * adj_dist;
    }

    desired
}
