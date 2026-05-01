use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        use crate::game::player::systems::{detect_ground, player_input};

        app.add_systems(
            Update,
            (detect_ground, player_input).run_if(in_state(crate::game::states::GameState::Playing)),
        );
    }
}
