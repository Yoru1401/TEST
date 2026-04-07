use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game::input::PlayerAction;
use crate::game::states::GameState;

use super::components::PlayerMarker;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum JumpAbilityType {
    Normal,
    Charge,
    Multi,
}

impl Default for JumpAbilityType {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Component, Default, Reflect)]
pub struct JumpAbilityState {
    pub current_type: JumpAbilityType,
    pub is_charging: bool,
}

pub struct JumpAbilityPlugin;

impl Plugin for JumpAbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_jump_type_switch,
                handle_jump_ability.after(handle_jump_type_switch),
            )
                .chain(),
        );
    }
}

fn handle_jump_type_switch(
    mut jump_state_query: Query<&mut JumpAbilityState, With<PlayerMarker>>,
    action_state_query: Query<&ActionState<PlayerAction>, With<PlayerMarker>>,
) {
    let Ok(mut jump_state) = jump_state_query.single_mut() else {
        return;
    };

    let Ok(action_state) = action_state_query.single() else {
        return;
    };

    if action_state.just_pressed(&PlayerAction::SwitchJumpType) {
        jump_state.current_type = match jump_state.current_type {
            JumpAbilityType::Normal => JumpAbilityType::Charge,
            JumpAbilityType::Charge => JumpAbilityType::Multi,
            JumpAbilityType::Multi => JumpAbilityType::Normal,
        };
        jump_state.is_charging = false;
    }
}

fn handle_jump_ability(
    state: Res<State<GameState>>,
    mut jump_state_query: Query<&mut JumpAbilityState, With<PlayerMarker>>,
    action_state_query: Query<&ActionState<PlayerAction>, With<PlayerMarker>>,
    character_query: Query<&super::CharacterMotor, With<PlayerMarker>>,
) {
    if state.get() != &GameState::Playground {
        return;
    }

    let Ok(action_state) = action_state_query.single() else {
        return;
    };

    let Ok(character_motor) = character_query.single() else {
        return;
    };

    let Ok(mut jump_state) = jump_state_query.single_mut() else {
        return;
    };

    match jump_state.current_type {
        JumpAbilityType::Normal => {}
        JumpAbilityType::Charge => {
            if action_state.pressed(&PlayerAction::Jump)
                && character_motor.is_grounded
                && !jump_state.is_charging
            {
                jump_state.is_charging = true;
            }

            if action_state.just_released(&PlayerAction::Jump) && jump_state.is_charging {
                jump_state.is_charging = false;
            }
        }
        JumpAbilityType::Multi => {}
    }
}
