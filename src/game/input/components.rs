pub use bevy::prelude::{GamepadButton, KeyCode, Reflect};
pub use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    Jump,
}

impl PlayerAction {
    pub fn input_map() -> InputMap<Self> {
        let mut map = InputMap::default();
        map.insert(Self::Jump, KeyCode::Space);
        map.insert(Self::Jump, GamepadButton::South);
        map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
        map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
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
        map.insert_dual_axis(Self::Look, MouseMove::default());
        map.insert_dual_axis(Self::Look, GamepadStick::RIGHT.inverted_y());
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
        map.insert(Self::Pause, KeyCode::Escape);
        map.insert(Self::Pause, KeyCode::KeyP);
        map.insert(Self::Pause, GamepadButton::Start);
        map
    }
}
