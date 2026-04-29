use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        use crate::game::ui::systems::{exit_game, exit_menu, handle_pause_input, setup_main_menu};

        app.add_plugins(univis_ui::prelude::UnivisUiPlugin)
            .add_systems(
                OnEnter(crate::game::states::GameState::MainMenu),
                (exit_menu, exit_game, setup_main_menu),
            )
            .add_systems(OnExit(crate::game::states::GameState::MainMenu), exit_menu)
            .add_systems(
                OnEnter(crate::game::states::GameState::Paused),
                crate::game::ui::systems::setup_pause_menu,
            )
            .add_systems(OnExit(crate::game::states::GameState::Paused), exit_menu)
            .add_systems(
                Update,
                handle_pause_input.run_if(in_state(crate::game::states::GameState::Playing)),
            );
    }
}
