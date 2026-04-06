use crate::prelude::{KeyCode, Reflect};
use leafwing_input_manager::prelude::{Actionlike, InputMap};

#[derive(Actionlike, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum PlayerAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
}

impl PlayerAction {
    pub fn input_map() -> InputMap<Self> {
        InputMap::new([
            (Self::MoveForward, KeyCode::KeyW),
            (Self::MoveForward, KeyCode::ArrowUp),
            (Self::MoveBackward, KeyCode::KeyS),
            (Self::MoveBackward, KeyCode::ArrowDown),
            (Self::MoveLeft, KeyCode::KeyA),
            (Self::MoveLeft, KeyCode::ArrowLeft),
            (Self::MoveRight, KeyCode::KeyD),
            (Self::MoveRight, KeyCode::ArrowRight),
            (Self::Jump, KeyCode::Space),
        ])
    }
}
