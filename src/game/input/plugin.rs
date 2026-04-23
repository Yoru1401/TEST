use crate::game::input::{CameraAction, GlobalAction, PlayerAction};
use leafwing_input_manager::prelude::InputManagerPlugin;

pub struct InputPlugin;

impl InputPlugin {
    pub fn build(self) -> InputManagerPlugin<PlayerAction> {
        InputManagerPlugin::<PlayerAction>::default()
    }
}

pub struct CameraInputPlugin;

impl CameraInputPlugin {
    pub fn build(self) -> InputManagerPlugin<CameraAction> {
        InputManagerPlugin::<CameraAction>::default()
    }
}

pub struct GlobalInputPlugin;

impl GlobalInputPlugin {
    pub fn build(self) -> InputManagerPlugin<GlobalAction> {
        InputManagerPlugin::<GlobalAction>::default()
    }
}
