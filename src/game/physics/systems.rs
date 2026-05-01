use bevy::prelude::*;
use bevy_rapier3d::plugin::RapierPhysicsPlugin;
use bevy_rapier3d::prelude::*;

use crate::game::physics::components::*;

pub const SKIN_WIDTH: f32 = 0.01;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin::default());

        app.add_systems(
            Update,
            (accumulate_forces, apply_forces, collide_and_slide)
                .chain()
                .run_if(in_state(crate::game::states::GameState::Playing)),
        );
    }
}

pub fn accumulate_forces(
    mut bodies: Query<(Entity, &PhysicsVelocity, &PhysicsConfig, &mut ForceApplier)>,
) {
    for (_entity, vel, config, mut force_app) in &mut bodies {
        force_app.add_force(Vec3::NEG_Y * config.gravity);

        let speed = vel.linear.length();
        if speed > 0.001 {
            let vel_dir = vel.linear / speed;
            let drag = -vel_dir * config.drag * speed * speed;
            force_app.add_force(drag);
        }
    }
}

pub fn apply_forces(time: Res<Time>, mut forces: Query<(&mut ForceApplier, &mut PhysicsVelocity)>) {
    let dt = time.delta_secs();
    for (mut force_app, mut vel) in &mut forces {
        force_app.apply_to(&mut vel, dt);
    }
}

pub fn collide_and_slide(
    time: Res<Time>,
    context: ReadRapierContext,
    materials: Query<&PhysicsMaterial>,
    mut bodies: Query<(
        Entity,
        &mut PhysicsVelocity,
        &PhysicsConfig,
        &mut Contacts,
        &mut Transform,
        &Collider,
    )>,
) {
    let context = context.single().unwrap();
    let dt = time.delta_secs();
    const SLIDE_ITER: usize = 4;
    const BIAS_FACTOR: f32 = 0.12;
    const MAX_BIAS: f32 = 0.006;
    const MIN_SPEED: f32 = 1e-4;

    for (entity, mut vel, cfg, mut contacts, mut transform, collider) in &mut bodies {
        contacts.clear();
        let filter = QueryFilter::default().exclude_collider(entity);

        let mut remaining = dt;
        let mut pos = transform.translation;
        let mut lin_vel = vel.linear;

        for _ in 0..SLIDE_ITER {
            let sweep = remaining * lin_vel;
            let sweep_len = sweep.length();
            if sweep_len < MIN_SPEED {
                break;
            }
            let dir = match Dir3::new(sweep) {
                Ok(d) => d,
                Err(_) => break,
            };

            let cast_opts = ShapeCastOptions {
                max_time_of_impact: sweep_len,
                target_distance: SKIN_WIDTH * 2.0,
                stop_at_penetration: false,
                compute_impact_geometry_on_penetration: true,
            };

            let Some((_hit_ent, hit)) = context.cast_shape(
                pos,
                Quat::IDENTITY,
                *dir,
                collider.into(),
                cast_opts,
                filter,
            ) else {
                pos += sweep;
                break;
            };

            let impact_dist = hit.time_of_impact;
            pos += *dir * impact_dist;

            let normal = hit.details.map_or(Vec3::ZERO, |d| d.normal1).normalize();

            let point = hit.details.map_or(pos, |d| d.witness2);
            contacts.add(_hit_ent, normal, point);

            let toi = (impact_dist / sweep_len).min(0.9);
            remaining *= 1.0 - toi;

            let vel_n = lin_vel.dot(normal);
            let mut impulse_n: f32 = 0.0;
            if vel_n < 0.0 {
                let restitution = materials.get(_hit_ent).map_or(0.001, |m| m.restitution);
                impulse_n = -(1.0 + restitution) * vel_n;
                let impulse_vec = impulse_n * normal;

                lin_vel += impulse_vec;

                let r = point - pos;
                let torque = r.cross(impulse_vec);
                vel.angular += torque * cfg.torsion * dt;
            }

            let vel_t = lin_vel - normal * lin_vel.dot(normal);
            let speed_t = vel_t.length();

            if speed_t > 1e-3 {
                let friction = materials.get(_hit_ent).map_or(0.0, |m| m.friction);

                let max_f = impulse_n * friction;
                let tangent = vel_t.normalize_or_zero();
                let impulse_t = -speed_t.min(max_f).max(-max_f) * tangent;

                lin_vel += impulse_t;

                let r = point - pos;
                let torque = r.cross(impulse_t);
                vel.angular += torque * cfg.torsion * dt;
            }

            let penetration = (point - pos).dot(normal);
            if penetration < 0.0 {
                let mut bias = -penetration * BIAS_FACTOR;
                if bias > MAX_BIAS {
                    bias = MAX_BIAS;
                }
                pos += bias * normal;
            }

            if remaining < 1e-6 {
                break;
            }
        }
        transform.translation = pos;
        vel.linear = lin_vel;
    }
}
