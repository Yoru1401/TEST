use crate::game::input::plugin::InputPlugin;
use crate::game::input::PlayerAction;
use crate::game::physics::components::PlayerMarker;
use crate::game::physics::plugin::PhysicsPlugin;
use crate::game::states::GameState;
use crate::game::ui::plugin::UiPlugin;
use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(bevy::window::WindowPlugin {
                primary_window: Some(bevy::window::Window {
                    title: "Bevy Playground".into(),
                    ..default()
                }),
                ..default()
            }),
            InputPlugin,
            PhysicsPlugin,
            UiPlugin,
        ));

        app.init_state::<GameState>();

        app.add_systems(OnEnter(GameState::Playground), setup_playground);
        app.add_systems(PostUpdate, enter_playground);
    }
}

fn enter_playground(
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if current_state.get() == &GameState::MainMenu {
        next_state.set(GameState::Playground);
    }
}

fn setup_playground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let input_map = InputMap::<PlayerAction>::new([
        (PlayerAction::MoveForward, KeyCode::KeyW),
        (PlayerAction::MoveForward, KeyCode::ArrowUp),
        (PlayerAction::MoveBackward, KeyCode::KeyS),
        (PlayerAction::MoveBackward, KeyCode::ArrowDown),
        (PlayerAction::MoveLeft, KeyCode::KeyA),
        (PlayerAction::MoveLeft, KeyCode::ArrowLeft),
        (PlayerAction::MoveRight, KeyCode::KeyD),
        (PlayerAction::MoveRight, KeyCode::ArrowRight),
        (PlayerAction::Jump, KeyCode::Space),
        (PlayerAction::SpawnCube, KeyCode::KeyE),
    ]);

    commands.spawn((
        Name::new("Ground"),
        DespawnOnExit(GameState::Playground),
        RigidBody::Static,
        Collider::cuboid(20.0, 0.5, 20.0),
        Mesh3d(meshes.add(Cuboid::new(20.0, 0.5, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.5, 0.3)))),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));

    commands.spawn((
        Name::new("Player Cube"),
        DespawnOnExit(GameState::Playground),
        PlayerMarker,
        input_map,
        ActionState::<PlayerAction>::default(),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        GravityScale(1.0),
        LinearDamping(0.5),
        AngularDamping(0.5),
        LockedAxes::ROTATION_LOCKED,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.8, 0.2, 0.2)))),
        Transform::from_xyz(0.0, 3.0, 0.0),
    ));

    commands.spawn((
        Name::new("Directional Light"),
        DespawnOnExit(GameState::Playground),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("3D Camera"),
        DespawnOnExit(GameState::Playground),
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Transform::from_xyz(8.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("UI Camera"),
        DespawnOnExit(GameState::Playground),
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        IsDefaultUiCamera,
    ));
}
