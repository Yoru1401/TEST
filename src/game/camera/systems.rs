use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::Ball};
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
    rapier_context: ReadRapierContext,
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
        let filter = QueryFilter::default().exclude_collider(player_entity);
        let dir_normalized = ray_dir / ray_dist;
        let shape_vel = dir_normalized * ray_dist;
        let cam_collider = Ball::new(CAM_COLLISION_RADIUS);
        let options = ShapeCastOptions {
            max_time_of_impact: 1.0,
            target_distance: 0.0,
            stop_at_penetration: false,
            compute_impact_geometry_on_penetration: false,
        };

        let context = rapier_context.single().unwrap();

        if let Some((_, hit)) = context.cast_shape(
            player_transform.translation,
            Quat::IDENTITY,
            shape_vel,
            &cam_collider,
            options,
            filter,
        ) {
            let safe_dist = (hit.time_of_impact * ray_dist - CAM_COLLISION_RADIUS).max(1.0);
            cam_transform.translation = player_transform.translation + dir_normalized * safe_dist;
        } else {
            cam_transform.translation = desired_pos;
        }
    } else {
        cam_transform.translation = desired_pos;
    }
}
