pub mod components;
pub mod config;
pub mod systems;

pub use systems::{
    accumulate_forces, apply_forces, detect_ground, resolve_collisions, PhysicsPlugin,
};

pub mod prelude {
    pub use super::components::*;
    pub use super::config::*;
}
