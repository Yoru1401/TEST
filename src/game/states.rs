use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
}

pub fn is_running(res: Res<State<GameState>>) -> bool {
    res.get() == &GameState::Playing
}
