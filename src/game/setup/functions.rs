use crate::game::camera::components::CameraMarker;
use crate::game::input::PlayerAction;
use crate::game::player::abilities::JumpAbilityState;
use crate::game::player::components::PlayerMarker;
use crate::game::player::systems::{CharacterMotor, InputSource};
use avian3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn setup_playground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Ground"),
        RigidBody::Static,
        Collider::cuboid(20.0, 0.5, 20.0),
        Mesh3d(meshes.add(Cuboid::new(20.0, 0.5, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.5, 0.3)))),
        Transform::from_xyz(0.0, -0.25, 0.0),
    ));

    commands.spawn((
        Name::new("Player Sphere"),
        PlayerMarker,
        RigidBody::Dynamic,
        Collider::sphere(0.5),
        LockedAxes::ROTATION_LOCKED,
        CharacterMotor::default(),
        InputSource::PlayerControlled,
        JumpAbilityState::default(),
        PlayerAction::input_map(),
        ActionState::<PlayerAction>::default(),
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.8, 0.2, 0.2)))),
        Transform::from_xyz(0.0, 3.0, 0.0),
    ));

    commands.spawn((
        Name::new("Directional Light"),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("3D Camera"),
        CameraMarker,
        Camera3d::default(),
        Transform::from_xyz(8.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
