use bevy::prelude::*;

#[derive(Component, Copy, Clone, Default)]
pub struct PhysicsConfig {
    pub gravity: f32,
    pub drag: f32,
    pub torsion: f32,
    pub air_control: f32,
    pub ground_control: f32,
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

#[derive(Component, Copy, Clone, Default)]
pub struct PhysicsMaterial {
    pub restitution: f32,
    pub friction: f32,
}

impl PhysicsMaterial {
    pub const fn concrete() -> Self {
        Self {
            restitution: 0.1,
            friction: 0.05,
        }
    }
}

#[derive(Component)]
pub struct TensionAnchor(pub Entity);

#[derive(Component)]
pub struct SpringAnchor {
    pub target: Entity,
    pub rest_length: f32,
    pub stiffness: f32,
}

#[derive(Component, Copy, Clone, Default)]
pub struct PhysicsVelocity {
    pub linear: Vec3,
    pub angular: Vec3,
}

#[derive(Component, Clone, Default)]
pub struct Contacts {
    pub entities: Vec<Entity>,
    pub normals: Vec<Vec3>,
    pub points: Vec<Vec3>,
}

impl Contacts {
    pub fn clear(&mut self) {
        self.entities.clear();
        self.normals.clear();
        self.points.clear();
    }

    pub fn add(&mut self, entity: Entity, normal: Vec3, point: Vec3) {
        self.entities.push(entity);
        self.normals.push(normal);
        self.points.push(point);
    }
}

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct GroundState {
    pub is_grounded: bool,
    pub ground_normal: Vec3,
}

impl Default for GroundState {
    fn default() -> Self {
        Self {
            is_grounded: false,
            ground_normal: Vec3::Y,
        }
    }
}

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
