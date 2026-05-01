use bevy::prelude::*;
/// Marker component for objects that have a physics velocity.
#[derive(Component)]
pub struct PhysicsVelocity {
    pub linear: Vec3,
    pub angular: Vec3,
}

impl Default for PhysicsVelocity {
    fn default() -> Self {
        Self {
            linear: Vec3::ZERO,
            angular: Vec3::ZERO,
        }
    }
}

/// Component that stores forces to be applied each frame.
#[derive(Component, Default)]
pub struct ForceApplier {
    force: Vec3,
    impulse: Vec3,
}

impl ForceApplier {
    pub fn add_force(&mut self, force: Vec3) {
        self.force += force;
    }

    pub fn add_impulse(&mut self, impulse: Vec3) {
        self.impulse += impulse;
    }

    pub fn apply_to(&mut self, velocity: &mut PhysicsVelocity, dt: f32) {
        velocity.linear += self.force * dt + self.impulse;
        self.force = Vec3::ZERO;
        self.impulse = Vec3::ZERO;
    }
}

/// Simple contact list used by the resolver.
#[derive(Component, Default)]
pub struct Contacts {
    pub entries: Vec<(Entity, Vec3, Vec3)>, // (other entity, normal, point)
}
impl Contacts {
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn add(&mut self, other: Entity, normal: Vec3, point: Vec3) {
        self.entries.push((other, normal, point));
    }
}

/// Physics configuration – tweak per‑entity if you like.
#[derive(Component)]
pub struct PhysicsConfig {
    pub gravity: f32,
    pub air_control: f32,
    pub ground_control: f32,
    pub drag: f32,
    pub torsion: f32,
}

impl PhysicsConfig {
    pub const fn player() -> Self {
        Self {
            gravity: 30.0,
            drag: 0.05,
            torsion: 2.0,
            air_control: 1.0,
            ground_control: 1.0,
        }
    }
}

/// Simple material component for restitution / friction.
#[derive(Component)]
pub struct PhysicsMaterial {
    pub restitution: f32,
    pub friction: f32,
}

impl PhysicsMaterial {
    pub const fn default() -> Self {
        Self {
            restitution: 0.01,
            friction: 0.6,
        }
    }

    pub const fn bouncy() -> Self {
        Self {
            restitution: 0.8,
            friction: 0.2,
        }
    }

    pub const fn slippery() -> Self {
        Self {
            restitution: 0.01,
            friction: 0.0,
        }
    }
}
