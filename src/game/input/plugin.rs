use crate::game::input::PlayerAction;
use leafwing_input_manager::prelude::InputManagerPlugin;

pub struct InputPlugin;

impl InputPlugin {
    pub fn build(self) -> InputManagerPlugin<PlayerAction> {
        InputManagerPlugin::<PlayerAction>::default()
    }
}
