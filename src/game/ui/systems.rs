use bevy::color::Color;
use bevy::prelude::*;
use univis_ui::prelude::*;
use univis_ui::widget::text_label::UTextLabel;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((univis_ui::prelude::UnivisUiPlugin,))
            .add_systems(
                OnEnter(crate::game::states::GameState::MainMenu),
                enter_main_menu,
            )
            .add_systems(OnExit(crate::game::states::GameState::MainMenu), exit_menu)
            .add_systems(
                OnEnter(crate::game::states::GameState::Paused),
                enter_pause_menu,
            )
            .add_systems(OnExit(crate::game::states::GameState::Paused), exit_menu)
            .add_systems(
                Update,
                handle_pause_input.run_if(in_state(crate::game::states::GameState::Playing)),
            );
    }
}

fn handle_pause_input(
    action: Query<&crate::prelude::ActionState<crate::game::GlobalAction>>,
    mut next_state: ResMut<NextState<crate::game::states::GameState>>,
) {
    for action_state in &action {
        if action_state.just_pressed(&crate::game::GlobalAction::Pause) {
            next_state.set(crate::game::states::GameState::Paused);
            return;
        }
    }
}

fn enter_main_menu(
    mut commands: Commands,
    spawned: Query<Entity, With<crate::game::setup::GameWorldSpawned>>,
    camera: Query<Entity, With<Camera2d>>,
) {
    for entity in &spawned {
        commands.entity(entity).despawn();
    }
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
            Name::new("Main Menu Root"),
            crate::game::ui::ScreenRootBundle::default(),
        ))
        .with_children(|parent| {
            parent.spawn(UTextLabel {
                text: "BEVY PLAYGROUND".into(),
                font_size: 48.0,
                color: Color::WHITE,
                ..default()
            });
            parent
                .spawn(crate::game::ui::ButtonBundle::new(
                    Color::srgb(0.2, 0.6, 0.9),
                    Color::srgb(0.3, 0.7, 1.0),
                    Color::srgb(0.1, 0.5, 0.8),
                ))
                .observe(|_trigger: On<Pointer<Click>>, mut next_state: ResMut<NextState<crate::game::states::GameState>>| {
                    next_state.set(crate::game::states::GameState::Playing);
                })
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

fn enter_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Pause Menu Root"),
            crate::game::ui::ScreenRootBundle {
                node: UNode {
                    width: UVal::Percent(100.0),
                    height: UVal::Percent(100.0),
                    background_color: Color::oklcha(0.0, 0.0, 0.0, 0.7).into(),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(UTextLabel {
                text: "PAUSED".into(),
                font_size: 48.0,
                color: Color::WHITE,
                ..default()
            });
            parent
                .spawn(crate::game::ui::ButtonBundle::new(
                    Color::srgb(0.2, 0.8, 0.3),
                    Color::srgb(0.3, 0.9, 0.4),
                    Color::srgb(0.1, 0.6, 0.2),
                ))
                .observe(|_trigger: On<Pointer<Click>>, mut next_state: ResMut<NextState<crate::game::states::GameState>>| {
                    next_state.set(crate::game::states::GameState::Playing);
                })
                .with_children(|p| {
                    p.spawn(UTextLabel {
                        text: "Resume".into(),
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    });
                });

            parent
                .spawn(crate::game::ui::ButtonBundle::new(
                    Color::srgb(0.8, 0.2, 0.2),
                    Color::srgb(0.9, 0.3, 0.3),
                    Color::srgb(0.6, 0.1, 0.1),
                ))
                .observe(|_trigger: On<Pointer<Click>>, mut next_state: ResMut<NextState<crate::game::states::GameState>>| {
                    next_state.set(crate::game::states::GameState::MainMenu);
                })
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

fn exit_menu(mut commands: Commands, query: Query<Entity, With<UScreenRoot>>) {
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
    }
}
