use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

pub const MOVE_SPEED: f32 = 50.0;
pub const JUMP_VELOCITY: f32 = 12.0;

pub fn detect_ground(
    context: ReadRapierContext,
    mut bodies: Query<(
        Entity,
        &Transform,
        &Collider,
        &mut crate::game::physics::GroundState,
    )>,
) {
    let context = context.single().unwrap();
    for (entity, transform, collider, mut ground_state) in &mut bodies {
        let filter = QueryFilter::default().exclude_collider(entity);
        let shape_vel = Vec3::NEG_Y * 0.1;
        let options = ShapeCastOptions {
            max_time_of_impact: 1.0,
            target_distance: 0.0,
            stop_at_penetration: false,
            compute_impact_geometry_on_penetration: true,
        };
        if let Some((_, hit)) = context.cast_shape(
            transform.translation,
            Quat::IDENTITY,
            shape_vel,
            collider.into(),
            options,
            filter,
        ) {
            ground_state.is_grounded = true;
            if let Some(details) = hit.details {
                ground_state.ground_normal = details.normal1;
            }
        } else {
            ground_state.is_grounded = false;
        }
    }
}

pub fn player_input(
    camera: Query<&Transform, With<crate::game::camera::CameraMarker>>,
    mut player: Query<
        (
            &mut crate::game::physics::ForceApplier,
            &crate::game::physics::GroundState,
            &crate::game::physics::Contacts,
            &ActionState<crate::game::input::PlayerAction>,
        ),
        With<crate::game::player::PlayerMarker>,
    >,
) {
    let Ok(cam_transform) = camera.single() else {
        return;
    };
    let Ok((mut force_app, ground_state, contacts, action)) = player.single_mut() else {
        return;
    };

    let move_axis = action.axis_pair(&crate::game::input::PlayerAction::Move);
    let forward = -cam_transform.forward().xz().extend(0.0).xzy();
    let right = cam_transform.right().xz().extend(0.0).xzy();
    let horizontal = forward * -move_axis.y + right * move_axis.x;

    force_app.add_force(horizontal * MOVE_SPEED);

    if action.just_pressed(&crate::game::input::PlayerAction::Jump) {
        if ground_state.is_grounded {
            force_app.add_impulse(ground_state.ground_normal * JUMP_VELOCITY);
        } else if !contacts.entities.is_empty() {
            let jump_dir = (contacts.normals[0] + Vec3::Y * 0.3).normalize();
            force_app.add_impulse(jump_dir * JUMP_VELOCITY);
        }
    }
}
