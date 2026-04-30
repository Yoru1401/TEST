use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct DepenetrationConfig {
    pub depenetration_iterations: usize,
    pub max_depenetration_error: f32,
    pub penetration_rejection_threshold: f32,
    pub skin_width: f32,
}

impl Default for DepenetrationConfig {
    fn default() -> Self {
        Self {
            depenetration_iterations: 16,
            max_depenetration_error: 0.0001,
            penetration_rejection_threshold: 0.5,
            skin_width: 0.01,
        }
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.add_systems(
            Update,
            (accumulate_forces, apply_forces, resolve_collisions)
                .chain()
                .run_if(in_state(crate::game::states::GameState::Playing)),
        );
    }
}

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

pub fn resolve_collisions(
    time: Res<Time>,
    context: ReadRapierContext,
    materials: Query<&crate::game::physics::PhysicsMaterial>,
    mut bodies: Query<(
        Entity,
        &mut crate::game::physics::PhysicsVelocity,
        &crate::game::physics::PhysicsConfig,
        &mut crate::game::physics::Contacts,
        &mut Transform,
        &Collider,
    )>,
) {
    let context = context.single().unwrap();
    let dt = time.delta_secs();
    let move_and_slide_iterations = 4;
    let depenetration_config = DepenetrationConfig::default();

    for (entity, mut vel, physics_config, mut contacts, mut transform, collider) in &mut bodies {
        contacts.clear();
        let filter = QueryFilter::default().exclude_collider(entity);

        let mut velocity = vel.linear;
        let mut position = transform.translation;
        let mut time_left = dt;

        for _ in 0..move_and_slide_iterations {
            let sweep = time_left * velocity;
            let length = sweep.length();
            if length < 1e-4 {
                break;
            }

            let dir = match Dir3::new(sweep) {
                Ok(d) => d,
                Err(_) => break,
            };

            let options = ShapeCastOptions {
                max_time_of_impact: length,
                target_distance: depenetration_config.skin_width,
                stop_at_penetration: false,
                compute_impact_geometry_on_penetration: true,
            };

            let Some((hit_entity, hit)) = context.cast_shape(
                position,
                Quat::IDENTITY,
                *dir,
                collider.into(),
                options,
                filter,
            ) else {
                position += sweep;
                break;
            };
            let toi = hit.time_of_impact / length;
            position += *dir * hit.time_of_impact;
            time_left *= 1.0 - toi;

            let hit_normal = hit.details.map_or(Vec3::Y, |d| d.normal1);
            let hit_point = hit.details.map_or(position, |d| d.witness2);
            contacts.add(hit_entity, hit_normal, hit_point);

            let velocity_along_normal = velocity.dot(hit_normal);
            if velocity_along_normal < 0.0 {
                let normal_part = velocity_along_normal * hit_normal;
                let tangent_part = velocity - normal_part;
                let restitution = materials.get(hit_entity).map_or(0.0, |m| m.restitution);
                let friction = materials.get(hit_entity).map_or(0.0, |m| m.friction);

                velocity -= normal_part * (1.0 + restitution);
                let tangent_speed = tangent_part.length();
                if tangent_speed > 1e-3 {
                    velocity -= tangent_part * (friction * (tangent_speed / (tangent_speed + 1.0)));
                }

                let r = hit_point - position;
                let torque = r.cross(normal_part * velocity_along_normal);
                vel.angular += torque * physics_config.torsion * dt;
            }

            if time_left < 1e-6 {
                break;
            }
        }

        transform.translation = position;
        vel.linear = velocity;
    }
}
