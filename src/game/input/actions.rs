use bevy::prelude::{KeyCode, Reflect};
use leafwing_input_manager::prelude::{Actionlike, InputMap};

#[derive(Actionlike, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum PlayerAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
    SwitchJumpType,
}

impl PlayerAction {
    pub fn input_map() -> InputMap<Self> {
        InputMap::new([
            (Self::MoveForward, KeyCode::KeyW),
            (Self::MoveBackward, KeyCode::KeyS),
            (Self::MoveLeft, KeyCode::KeyA),
            (Self::MoveRight, KeyCode::KeyD),
            (Self::Jump, KeyCode::Space),
            (Self::SwitchJumpType, KeyCode::KeyQ),
        ])
    }
}
