use bevy::color::Color;
use bevy::prelude::*;
use univis_ui::prelude::*;
use univis_ui::widget::text_label::UTextLabel;

use crate::game::states::GameState;
use crate::game::ui::components::{ButtonBundle, ButtonColors, MenuButton, MenuButtonAction};
use crate::game::GlobalAction;
use crate::prelude::ActionState;

pub fn handle_pause_input(
    action: Query<&ActionState<GlobalAction>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for action_state in &action {
        if action_state.just_pressed(&GlobalAction::Pause) {
            next_state.set(GameState::Paused);
            return;
        }
    }
}

pub fn exit_menu(mut commands: Commands, query: Query<Entity, With<UScreenRoot>>) {
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
    }
}

pub fn exit_game(
    mut commands: Commands,
    query: Query<Entity, With<crate::game::setup::GameWorldSpawned>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn setup_pause_menu(mut commands: Commands, next_state: ResMut<NextState<GameState>>) {
    let _state = next_state.clone();

    commands
        .spawn((
            Name::new("Pause Menu"),
            UScreenRoot,
            UNode {
                width: UVal::Percent(100.0),
                height: UVal::Percent(100.0),
                background_color: Color::oklcha(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Title"),
                UTextLabel {
                    text: "PAUSED".into(),
                    font_size: 48.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            parent
                .spawn((
                    Name::new("Resume Button"),
                    MenuButton::resume(),
                    ButtonBundle::new(
                        ButtonColors::green().normal,
                        ButtonColors::green().hovered,
                        ButtonColors::green().pressed,
                    ),
                ))
                .observe(
                    |_trigger: On<Pointer<Click>>,
                     button: Query<&MenuButton>,
                     mut next_state: ResMut<NextState<GameState>>| {
                        if let Ok(btn) = button.get(_trigger.event_target()) {
                            match &btn.action {
                                MenuButtonAction::Transition(state) => {
                                    next_state.set(*state);
                                }
                                MenuButtonAction::Quit => {
                                    std::process::exit(0);
                                }
                            }
                        }
                    },
                )
                .with_children(|p| {
                    p.spawn(UTextLabel {
                        text: "Resume".into(),
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    });
                });

            parent
                .spawn((
                    Name::new("Quit Button"),
                    MenuButton::quit(),
                    ButtonBundle::new(
                        ButtonColors::red().normal,
                        ButtonColors::red().hovered,
                        ButtonColors::red().pressed,
                    ),
                ))
                .observe(
                    |_trigger: On<Pointer<Click>>,
                     button: Query<&MenuButton>,
                     mut next_state: ResMut<NextState<GameState>>| {
                        if let Ok(btn) = button.get(_trigger.event_target()) {
                            match &btn.action {
                                MenuButtonAction::Transition(state) => {
                                    next_state.set(*state);
                                }
                                MenuButtonAction::Quit => {
                                    std::process::exit(0);
                                }
                            }
                        }
                    },
                )
                .with_children(|p| {
                    p.spawn(UTextLabel {
                        text: "Quit to Menu".into(),
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    });
                });
        });
}

pub fn setup_main_menu(
    mut commands: Commands,
    camera: Query<Entity, With<Camera2d>>,
    next_state: ResMut<NextState<GameState>>,
) {
    let _state = next_state.clone();

    if !camera.iter().next().is_some() {
        commands.spawn((
            Name::new("UI Camera"),
            Camera2d,
            Camera {
                clear_color: ClearColorConfig::None,
                order: 1,
                ..default()
            },
        ));
    }

    commands
        .spawn((
            Name::new("Main Menu"),
            UScreenRoot,
            UNode {
                width: UVal::Percent(100.0),
                height: UVal::Percent(100.0),
                background_color: Color::srgb(0.1, 0.1, 0.15),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Title"),
                UTextLabel {
                    text: "BEVY PLAYGROUND".into(),
                    font_size: 48.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            parent
                .spawn((
                    Name::new("Start Button"),
                    MenuButton::start(),
                    ButtonBundle::new(
                        ButtonColors::blue().normal,
                        ButtonColors::blue().hovered,
                        ButtonColors::blue().pressed,
                    ),
                ))
                .observe(
                    |_trigger: On<Pointer<Click>>,
                     button: Query<&MenuButton>,
                     mut next_state: ResMut<NextState<GameState>>| {
                        if let Ok(btn) = button.get(_trigger.event_target()) {
                            match &btn.action {
                                MenuButtonAction::Transition(state) => {
                                    next_state.set(*state);
                                }
                                MenuButtonAction::Quit => {
                                    std::process::exit(0);
                                }
                            }
                        }
                    },
                )
                .with_children(|p| {
                    p.spawn(UTextLabel {
                        text: "Start Game".into(),
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    });
                });
        });
}
