use avian3d::prelude::*;
use bevy::color::Color;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game::camera::components::CameraMarker;
use crate::game::input::CameraAction;
use crate::game::input::PlayerAction;
use crate::game::player::components::PlayerMarker;
use crate::game::player::components::{JumpState, WallState};
use crate::game::player::systems::DesiredVelocity;
use crate::game::GameState;

pub fn is_running(res: Res<State<GameState>>) -> bool {
    res.get() == &GameState::Playground
}

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_mat = materials.add(StandardMaterial::from_color(Color::srgb(0.9, 0.2, 0.2)));

    commands.spawn((
        Name::new("Player"),
        PlayerMarker,
        RigidBody::Kinematic,
        Collider::sphere(0.5),
        CustomPositionIntegration,
        PlayerAction::input_map(),
        ActionState::<PlayerAction>::default(),
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(player_mat),
        Transform::from_xyz(0.0, 2.0, 0.0),
        DesiredVelocity::default(),
        JumpState::default(),
        WallState::default(),
    ));

    commands.spawn((
        Name::new("Camera"),
        CameraMarker,
        CameraAction::input_map(),
        ActionState::<CameraAction>::default(),
        Camera3d::default(),
        Transform::from_xyz(0.0, 15.0, 20.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
}

pub fn setup_playground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_mat = materials.add(StandardMaterial::from_color(Color::srgb(0.2, 0.3, 0.2)));
    let wall_mat = materials.add(StandardMaterial::from_color(Color::srgb(0.2, 0.1, 0.3)));
    let ramp_mat = materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.4)));

    commands.spawn((
        Name::new("Ground"),
        RigidBody::Static,
        Collider::cuboid(50.0, 0.5, 50.0),
        Mesh3d(meshes.add(Cuboid::new(50.0, 0.5, 50.0))),
        MeshMaterial3d(ground_mat.clone()),
        Transform::from_xyz(0.0, -0.25, 0.0),
    ));

    for i in 0..10 {
        let y = 5.0 - (i as f32 * 0.5);
        commands.spawn((
            Name::new(format!("ZoneA_Stair_Step{}", i + 1)),
            RigidBody::Static,
            Collider::cuboid(3.0, y, 3.0),
            Mesh3d(meshes.add(Cuboid::new(3.0, y, 3.0))),
            MeshMaterial3d(wall_mat.clone()),
            Transform::from_xyz(-12.0 + (i as f32 * 3.0), y / 2.0, -12.0),
        ));
    }

    for i in 0..10 {
        let angle = -0.1 - (i as f32 * 0.1);
        let y = 0.5 + (i as f32 * 0.5);
        commands.spawn((
            Name::new(format!("ZoneB_Ramp{}", i + 1)),
            RigidBody::Static,
            Collider::cuboid(3.0, 0.2, 8.0),
            Mesh3d(meshes.add(Cuboid::new(3.0, 0.2, 8.0))),
            MeshMaterial3d(ramp_mat.clone()),
            Transform::from_xyz(-12.0 + (i as f32 * 3.0), y, 8.0)
                .with_rotation(Quat::from_rotation_x(angle)),
        ));
    }

    commands.spawn((
        Name::new("Directional Light"),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
