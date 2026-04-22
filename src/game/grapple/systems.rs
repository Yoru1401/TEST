use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game::input::PlayerAction;
use crate::game::is_running;
use crate::game::physics::components::{PhysicsConfig, PhysicsVelocity};
use crate::game::player::components::PlayerMarker;

pub struct GrapplePlugin;

impl Plugin for GrapplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fire_grapple.run_if(is_running));
        app.add_systems(Update, update_grapple_projectile);
        app.add_systems(Update, update_swing);
    }
}

#[derive(Component)]
pub struct GrappleProjectile {
    pub owner: Entity,
    pub speed: f32,
}

impl GrappleProjectile {
    pub fn new(owner: Entity) -> Self {
        Self { owner, speed: 50.0 }
    }
}

#[derive(Component)]
pub struct SwingState {
    pub is_swinging: bool,
    pub hook_entity: Option<Entity>,
    pub rest_length: f32,
    pub swing_stiffness: f32,
}

impl Default for SwingState {
    fn default() -> Self {
        Self {
            is_swinging: false,
            hook_entity: None,
            rest_length: 5.0,
            swing_stiffness: 20.0,
        }
    }
}

#[derive(Component)]
pub struct GrappleCooldown {
    pub timer: f32,
}

impl Default for GrappleCooldown {
    fn default() -> Self {
        Self { timer: 0.0 }
    }
}

fn fire_grapple(
    _time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cooldowns: Query<&GrappleCooldown>,
    action: Query<&ActionState<PlayerAction>>,
    camera: Query<&Transform, With<crate::game::camera::components::CameraMarker>>,
    player: Query<Entity, With<PlayerMarker>>,
) {
    let _dt = _time.delta_secs();

    let Ok(cam_transform) = camera.single() else {
        return;
    };
    let Ok(player_entity) = player.single() else {
        return;
    };
    let Ok(action_state) = action.single() else {
        return;
    };

    if let Ok(cooldown) = cooldowns.single() {
        if cooldown.timer > 0.0 {
            return;
        }
    }

    if action_state.just_pressed(&PlayerAction::Jump) {
        let direction = -cam_transform.forward();

        commands.spawn((
            GrappleProjectile::new(player_entity),
            RigidBody::Kinematic,
            Collider::sphere(0.1),
            CustomPositionIntegration,
            PhysicsConfig {
                gravity: 5.0,
                drag: 0.01,
                torsion: 0.0,
                air_control: 1.0,
                ground_control: 1.0,
            },
            PhysicsVelocity::new(),
            Mesh3d(meshes.add(Sphere::new(0.1))),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(1.0, 0.8, 0.0)))),
            Transform::from_translation(cam_transform.translation + direction * 0.5),
        ));
    }
}

fn update_grapple_projectile(
    time: Res<Time>,
    mut commands: Commands,
    mut projectiles: Query<(Entity, &GrappleProjectile, &mut PhysicsVelocity, &Transform)>,
) {
    let _dt = time.delta_secs();

    for (entity, _proj, vel, transform) in &mut projectiles {
        if transform.translation.y < -10.0 {
            commands.entity(entity).despawn();
            continue;
        }

        let distance = vel.linear.length();
        if distance > 50.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn update_swing(
    _time: Res<Time>,
    player: Query<&Transform, With<PlayerMarker>>,
    projectiles: Query<(Entity, &Transform), With<GrappleProjectile>>,
    mut swing_states: Query<&mut SwingState>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    for mut swing in &mut swing_states {
        if swing.is_swinging {
            if let Some(hook_entity) = swing.hook_entity {
                if let Ok((_, hook_transform)) = projectiles.get(hook_entity) {
                    let distance =
                        (hook_transform.translation - player_transform.translation).length();

                    if distance > swing.rest_length * 3.0 {
                        swing.is_swinging = false;
                        swing.hook_entity = None;
                    }
                } else {
                    swing.is_swinging = false;
                    swing.hook_entity = None;
                }
            } else {
                swing.is_swinging = false;
            }
        }
    }
}
