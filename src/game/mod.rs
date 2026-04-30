pub mod camera;
pub mod input;
pub mod physics;
pub mod player;
pub mod plugin;
pub mod setup;
pub mod states;
pub mod ui;

pub use camera::{CameraMarker, CameraPlugin};
pub use input::{CameraAction, GlobalAction, InputPlugin, PlayerAction};
pub use physics::{accumulate_forces, apply_forces, resolve_collisions};
pub use physics::{
    Contacts, ForceApplier, GroundState, PhysicsConfig, PhysicsMaterial, PhysicsPlugin,
    PhysicsVelocity, SpringAnchor, TensionAnchor,
};
pub use player::{PlayerMarker, PlayerPlugin};
pub use plugin::GamePlugin;
pub use setup::{setup_playground, GameWorldSpawned};
pub use states::GameState;
pub use ui::components::ButtonColors;
pub use ui::components::MenuButton;
pub use ui::plugins::UIPlugin;
