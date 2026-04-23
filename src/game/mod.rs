pub mod camera;
pub mod grapple;
pub mod input;
pub mod physics;
pub mod player;
pub mod plugin;
pub mod setup;
pub mod states;
pub mod ui;

pub use camera::{CameraMarker, CameraPlugin};
pub use grapple::{GrappleCooldown, GrapplePlugin, GrappleProjectile, SwingState};
pub use input::{CameraAction, GlobalAction, InputPlugin, PlayerAction};
pub use physics::{accumulate_forces, apply_forces, detect_ground, resolve_collisions};
pub use physics::{
    Contacts, ForceApplier, GroundState, PhysicsConfig, PhysicsMaterial, PhysicsPlugin,
    PhysicsVelocity, SpringAnchor, TensionAnchor,
};
pub use player::{PlayerMarker, PlayerPlugin};
pub use plugin::GamePlugin;
pub use setup::{is_running, setup_playground, GameWorldSpawned};
pub use states::GameState;
pub use ui::systems::{spawn_button, spawn_label, spawn_small_label, spawn_ui_screen};
pub use ui::{MainMenuPlugin, MainMenuRoot, PauseMenuPlugin, PauseMenuRoot, UIPlugin};
