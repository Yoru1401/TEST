use bevy::prelude::{GamepadButton, KeyCode, Reflect};
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    Jump,
}

impl PlayerAction {
    pub fn input_map() -> InputMap<Self> {
        let mut map = InputMap::default();
        map.insert(PlayerAction::Jump, KeyCode::Space);
        map.insert(PlayerAction::Jump, GamepadButton::South);

        map.insert_dual_axis(PlayerAction::Move, VirtualDPad::wasd());
        map.insert_dual_axis(PlayerAction::Move, GamepadStick::LEFT);
        map
    }
}

#[derive(Actionlike, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum CameraAction {
    #[actionlike(DualAxis)]
    Look,
}

impl CameraAction {
    pub fn input_map() -> InputMap<Self> {
        let mut map = InputMap::default();

        map.insert_dual_axis(CameraAction::Look, MouseMove::default());
        map.insert_dual_axis(CameraAction::Look, GamepadStick::RIGHT.inverted_y());
        map
    }
}

#[derive(Actionlike, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum GlobalAction {
    Pause,
}

impl GlobalAction {
    pub fn input_map() -> InputMap<Self> {
        let mut map = InputMap::default();
        map.insert(GlobalAction::Pause, KeyCode::Escape);
        map.insert(GlobalAction::Pause, KeyCode::KeyP);
        map.insert(GlobalAction::Pause, GamepadButton::Start);
        map
    }
}
