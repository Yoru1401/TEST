use crate::prelude::*;

#[derive(Component)]
pub struct GameWorldSpawned;

pub fn setup_playground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spawned_query: Query<Entity, With<GameWorldSpawned>>,
) {
    if spawned_query.iter().next().is_some() {
        return;
    }

    let concrete = crate::game::PhysicsMaterial::concrete();

    let mut player_cmd = commands.spawn((
        Name::new("Player"),
        crate::game::PlayerMarker,
        GameWorldSpawned,
        RigidBody::Kinematic,
        Collider::sphere(0.5),
        CustomPositionIntegration,
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.9, 0.2, 0.2)))),
        Transform::from_xyz(12.0, 20.0, 8.0),
        crate::game::PhysicsConfig::player(),
        crate::game::PhysicsVelocity::default(),
        crate::game::GroundState::default(),
        crate::game::ForceApplier::default(),
        crate::game::Contacts::default(),
    ));
    player_cmd.insert(crate::game::PlayerAction::input_map());
    player_cmd.insert(crate::game::GlobalAction::input_map());
    player_cmd.insert(ActionState::<crate::game::PlayerAction>::default());
    player_cmd.insert(ActionState::<crate::game::GlobalAction>::default());

    commands.spawn((
        Name::new("GameCamera"),
        crate::game::CameraMarker,
        GameWorldSpawned,
        crate::game::CameraAction::input_map(),
        ActionState::<crate::game::CameraAction>::default(),
        Camera3d::default(),
        Transform::from_xyz(0.0, 15.0, 20.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));

    let ground_mat = materials.add(StandardMaterial::from_color(Color::srgb(0.2, 0.3, 0.2)));
    let wall_mat = materials.add(StandardMaterial::from_color(Color::srgb(0.2, 0.1, 0.3)));
    let ramp_mat = materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.4)));

    commands.spawn((
        Name::new("Ground"),
        GameWorldSpawned,
        RigidBody::Static,
        Collider::cuboid(50.0, 0.5, 50.0),
        concrete,
        Mesh3d(meshes.add(Cuboid::new(50.0, 0.5, 50.0))),
        MeshMaterial3d(ground_mat),
        Transform::from_xyz(0.0, -0.25, 0.0),
    ));

    for i in 0..10 {
        let y = 5.0 - (i as f32 * 0.5);
        commands.spawn((
            Name::new(format!("ZoneA_Stair_Step{i}")),
            GameWorldSpawned,
            RigidBody::Static,
            Collider::cuboid(3.0, y, 3.0),
            concrete,
            Mesh3d(meshes.add(Cuboid::new(3.0, y, 3.0))),
            MeshMaterial3d(wall_mat.clone()),
            Transform::from_xyz(-12.0 + (i as f32 * 3.0), y / 2.0, -12.0),
        ));
    }

    for i in 0..10 {
        let angle = -0.1 - (i as f32 * 0.1);
        let y = 0.5 + (i as f32 * 0.5);
        commands.spawn((
            Name::new(format!("ZoneB_Ramp{i}")),
            GameWorldSpawned,
            RigidBody::Static,
            Collider::cuboid(3.0, 0.2, 8.0),
            concrete,
            Mesh3d(meshes.add(Cuboid::new(3.0, 0.2, 8.0))),
            MeshMaterial3d(ramp_mat.clone()),
            Transform::from_xyz(-12.0 + (i as f32 * 3.0), y, 8.0)
                .with_rotation(Quat::from_rotation_x(angle)),
        ));
    }

    commands.spawn((
        Name::new("ZoneC Wall"),
        GameWorldSpawned,
        RigidBody::Static,
        Collider::cuboid(0.1, 50.0, 50.0),
        concrete,
        Mesh3d(meshes.add(Cuboid::new(0.1, 50.0, 50.0))),
        MeshMaterial3d(wall_mat),
        Transform::from_xyz(-15.0, 25.0, 0.0),
    ));

    commands.spawn((
        Name::new("Directional Light"),
        GameWorldSpawned,
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
