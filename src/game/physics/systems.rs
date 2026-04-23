use avian3d::prelude::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(avian3d::prelude::PhysicsPlugins::default());
        app.add_systems(
            PreUpdate,
            detect_ground.run_if(crate::game::setup::is_running),
        );
        app.add_systems(Update, apply_forces.run_if(crate::game::setup::is_running));
        app.add_systems(
            Update,
            accumulate_forces.run_if(crate::game::setup::is_running),
        );
        app.add_systems(
            PostUpdate,
            resolve_collisions.run_if(crate::game::setup::is_running),
        );
    }
}

pub const SKIN_WIDTH: f32 = 0.01;

pub fn apply_forces(
    time: Res<Time>,
    mut forces: Query<(
        &mut crate::game::physics::ForceApplier,
        &mut crate::game::physics::PhysicsVelocity,
    )>,
) {
    let dt = time.delta_secs();
    for (mut force_app, mut vel) in &mut forces {
        force_app.apply_to(&mut vel, dt);
    }
}

#[allow(clippy::type_complexity)]
pub fn accumulate_forces(
    transforms: Query<&Transform>,
    mut bodies: Query<
        (
            Entity,
            &crate::game::physics::PhysicsVelocity,
            &crate::game::physics::PhysicsConfig,
            &mut crate::game::physics::ForceApplier,
            Option<&crate::game::physics::TensionAnchor>,
            Option<&crate::game::physics::SpringAnchor>,
            Option<&crate::game::physics::GroundState>,
        ),
        With<crate::game::physics::PhysicsVelocity>,
    >,
) {
    for (entity, vel, config, mut force_app, tension, spring, ground_state) in &mut bodies {
        let control = ground_state.map_or(config.air_control, |g| {
            if g.is_grounded {
                config.ground_control
            } else {
                config.air_control
            }
        });
        force_app.add_force(Vec3::NEG_Y * config.gravity * control);
        if let Some(&crate::game::physics::TensionAnchor(anchor)) = tension {
            if let (Ok(pos), Ok(anchor_pos)) = (transforms.get(entity), transforms.get(anchor)) {
                let to_anchor = anchor_pos.translation - pos.translation;
                let distance = to_anchor.length();
                if distance > 0.001 {
                    force_app.add_force((to_anchor / distance) * 15.0 * control);
                }
            }
        }
        if let Some(spring_comp) = spring {
            if let (Ok(pos), Ok(target_pos)) =
                (transforms.get(entity), transforms.get(spring_comp.target))
            {
                let to_target = target_pos.translation - pos.translation;
                let distance = to_target.length();
                if distance > 0.001 {
                    let direction = to_target / distance;
                    let extension = distance - spring_comp.rest_length;
                    force_app.add_force(direction * spring_comp.stiffness * extension * control);
                }
            }
        }
        let speed = vel.linear.length();
        if speed > 0.001 {
            let vel_dir = vel.linear / speed;
            force_app.add_force(-vel_dir * config.drag * speed * speed);
        }
    }
}

pub fn detect_ground(
    spatial_query: SpatialQuery,
    transforms: Query<&Transform>,
    mut bodies: Query<
        (Entity, &mut crate::game::physics::GroundState),
        With<crate::game::physics::PhysicsVelocity>,
    >,
) {
    for (entity, mut ground_state) in &mut bodies {
        let Ok(transform) = transforms.get(entity) else {
            continue;
        };
        let filter = SpatialQueryFilter::from_excluded_entities([entity]);
        let hit = spatial_query.cast_shape(
            &Collider::sphere(0.5),
            transform.translation,
            Quat::IDENTITY,
            Dir3::NEG_Y,
            &ShapeCastConfig {
                max_distance: 0.1,
                ..default()
            },
            &filter,
        );
        ground_state.is_grounded = hit.is_some();
        if let Some(hit_data) = hit {
            ground_state.ground_normal = hit_data.normal1;
        }
    }
}

pub fn resolve_collisions(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    move_and_slide: MoveAndSlide,
    materials: Query<&crate::game::physics::PhysicsMaterial>,
    mut bodies: Query<
        (
            Entity,
            &mut crate::game::physics::PhysicsVelocity,
            &crate::game::physics::PhysicsConfig,
            &mut crate::game::physics::Contacts,
            &mut Transform,
        ),
        With<crate::game::physics::PhysicsVelocity>,
    >,
) {
    let dt = time.delta_secs();
    let move_config = MoveAndSlideConfig::default();
    for (entity, mut vel, config, mut contacts, mut transform) in &mut bodies {
        contacts.clear();
        let filter = SpatialQueryFilter::from_excluded_entities([entity]);
        let depen_offset = move_and_slide.depenetrate(
            &Collider::sphere(0.5),
            transform.translation,
            Quat::IDENTITY,
            &(&move_config).into(),
            &filter,
        );
        transform.translation += depen_offset;
        let mut velocity = vel.linear;
        let mut position = transform.translation;
        let mut time_left = dt;
        for _ in 0..move_config.move_and_slide_iterations {
            let sweep = time_left * velocity;
            let length = sweep.length();
            if length < 1e-4 {
                break;
            }
            let vel_dir = match Dir3::new(sweep / length) {
                Ok(dir) => dir,
                Err(_) => break,
            };
            let hit = spatial_query.cast_shape(
                &Collider::sphere(0.5),
                position,
                Quat::IDENTITY,
                vel_dir,
                &ShapeCastConfig {
                    max_distance: length,
                    ..default()
                },
                &filter,
            );
            let Some(sweep_hit) = hit else {
                position += sweep;
                break;
            };
            let fraction = sweep_hit.distance / length;
            position += *vel_dir * (sweep_hit.distance - SKIN_WIDTH).max(0.0);
            time_left *= 1.0 - fraction;
            let hit_entity = sweep_hit.entity;
            let hit_normal: Vec3 = sweep_hit.normal1;
            let hit_point = sweep_hit.point2;
            contacts.add(hit_entity, hit_normal, hit_point);
            let velocity_along_normal = velocity.dot(hit_normal);
            if velocity_along_normal < 0.0 {
                let normal_part = velocity_along_normal * hit_normal;
                let tangent_part = velocity - normal_part;
                let material_restitution = materials.get(hit_entity).map_or(0.0, |m| m.restitution);
                let material_friction = materials.get(hit_entity).map_or(0.0, |m| m.friction);
                velocity -= normal_part * (1.0 - material_restitution);
                let tangent_speed = tangent_part.length();
                if tangent_speed > 0.5 {
                    velocity -= tangent_part * material_friction;
                }
                let r: Vec3 = hit_point - position;
                let torque = r.cross(normal_part * velocity_along_normal);
                vel.angular += torque * config.torsion * dt;
            }
            if time_left < 1e-6 {
                break;
            }
        }
        transform.translation = position;
        vel.linear = velocity;
    }
}
