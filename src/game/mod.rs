pub mod camera;
pub mod input;
pub mod player;
pub mod plugin;
pub mod setup;
pub mod states;
pub mod ui;

pub use camera::{CameraMarker, CameraPlugin};
pub use input::{CameraInputPlugin, InputPlugin};
pub use player::PlayerPlugin;
pub use plugin::GamePlugin;
pub use setup::{is_running, setup_playground};
pub use states::GameState;
pub use ui::UIPlugin;
