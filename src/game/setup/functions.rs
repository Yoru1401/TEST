use avian3d::prelude::*;
use bevy::color::Color;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game::camera::components::CameraMarker;
use crate::game::input::PlayerAction;
use crate::game::player::components::PlayerMarker;
use crate::game::player::systems::{DesiredVelocity, JumpState};

pub fn setup_playground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_mat = materials.add(StandardMaterial::from_color(Color::srgb(0.2, 0.3, 0.2)));
    let wall_mat = materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.35)));
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
    ));

    commands.spawn((
        Name::new("Ground"),
        RigidBody::Static,
        Collider::cuboid(40.0, 1.0, 40.0),
        Mesh3d(meshes.add(Cuboid::new(40.0, 1.0, 40.0))),
        MeshMaterial3d(ground_mat),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));

    commands.spawn((
        Name::new("Wall"),
        RigidBody::Static,
        Collider::cuboid(1.0, 20.0, 20.0),
        Mesh3d(meshes.add(Cuboid::new(1.0, 20.0, 20.0))),
        MeshMaterial3d(wall_mat),
        Transform::from_xyz(10.0, 5.0, 0.0),
    ));

    commands.spawn((
        Name::new("Directional Light"),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("Camera"),
        CameraMarker,
        Camera3d::default(),
        Transform::from_xyz(0.0, 15.0, 20.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
}
