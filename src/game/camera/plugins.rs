use bevy::prelude::*;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        use crate::game::camera::systems::update_camera;
        app.add_systems(
            Update,
            update_camera.run_if(crate::game::states::is_running),
        );
    }
}
