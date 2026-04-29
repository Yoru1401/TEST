use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<crate::game::input::PlayerAction>::default(),
            InputManagerPlugin::<crate::game::input::CameraAction>::default(),
            InputManagerPlugin::<crate::game::input::GlobalAction>::default(),
        ));
    }
}
