use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Actionlike, Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum PlayerAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
    SpawnCube,
}

impl PlayerAction {
    pub fn direction_vector(&self) -> Option<Vec2> {
        match self {
            PlayerAction::MoveForward => Some(Vec2::Y),
            PlayerAction::MoveBackward => Some(-Vec2::Y),
            PlayerAction::MoveLeft => Some(-Vec2::X),
            PlayerAction::MoveRight => Some(Vec2::X),
            _ => None,
        }
    }
}
