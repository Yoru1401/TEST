use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        use crate::game::player::systems::player_input;

        app.add_systems(
            Update,
            player_input.run_if(in_state(crate::game::states::GameState::Playing)),
        );
    }
}
