pub mod camera;
pub mod input;
pub mod player;
pub mod plugin;
pub mod setup;
pub mod states;
pub mod ui;

pub use camera::CameraPlugin;
pub use input::InputPlugin;
pub use player::{JumpAbilityPlugin, JumpAbilityState, JumpAbilityType, JumpInfo, PlayerPlugin};
pub use plugin::GamePlugin;
pub use setup::setup_playground;
pub use states::GameState;
pub use ui::UIPlugin;
