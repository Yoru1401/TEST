use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub const MOVE_SPEED: f32 = 50.0;
pub const JUMP_VELOCITY: f32 = 12.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_input.run_if(crate::game::states::is_running));
    }
}

fn player_input(
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
    let horizontal = (forward * -move_axis.y + right * move_axis.x).normalize_or_zero();

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
