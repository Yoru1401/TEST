use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const CAM_DIST: f32 = 10.0;
const CAM_HEIGHT: f32 = 2.0;
const LOOK_SENSITIVITY: f32 = 3.0;
const CAM_COLLISION_RADIUS: f32 = 0.3;

pub fn update_camera(
    time: Res<Time>,
    player: Query<(Entity, &Transform), With<crate::game::player::PlayerMarker>>,
    mut camera: Query<
        (
            &mut Transform,
            &ActionState<crate::game::input::CameraAction>,
        ),
        (
            With<crate::game::camera::CameraMarker>,
            Without<crate::game::player::PlayerMarker>,
        ),
    >,
    spatial: SpatialQuery,
    mut yaw: Local<f32>,
    mut pitch: Local<f32>,
) {
    let Ok((player_entity, player_transform)) = player.single() else {
        return;
    };
    let Ok((mut cam_transform, action)) = camera.single_mut() else {
        return;
    };

    let input = action.axis_pair(&crate::game::input::CameraAction::Look);
    if input.x.abs() > 0.05 || input.y.abs() > 0.05 {
        *yaw -= input.x * LOOK_SENSITIVITY * time.delta_secs();
        *pitch -= input.y * LOOK_SENSITIVITY * time.delta_secs();
        *pitch = pitch.clamp(-1.2, 1.2);
    }

    cam_transform.rotation = Quat::from_euler(EulerRot::YXZ, *yaw, *pitch, 0.0);

    let forward = cam_transform.forward();
    let desired_pos = player_transform.translation + Vec3::Y * CAM_HEIGHT - forward * CAM_DIST;

    let ray_dir = desired_pos - player_transform.translation;
    let ray_dist = ray_dir.length();

    if ray_dist > 0.0 {
        let filter = SpatialQueryFilter::from_excluded_entities([player_entity]);
        let dir_normalized = ray_dir / ray_dist;
        let dir3 = Dir3::new(dir_normalized).unwrap_or(Dir3::Y);

        if let Some(hit) = spatial.cast_shape(
            &Collider::sphere(CAM_COLLISION_RADIUS),
            player_transform.translation,
            Quat::IDENTITY,
            dir3,
            &ShapeCastConfig {
                max_distance: ray_dist,
                ..default()
            },
            &filter,
        ) {
            let safe_dist = (hit.distance - CAM_COLLISION_RADIUS).max(1.0);
            cam_transform.translation = player_transform.translation + dir_normalized * safe_dist;
        } else {
            cam_transform.translation = desired_pos;
        }
    } else {
        cam_transform.translation = desired_pos;
    }
}
