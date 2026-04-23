mod main_menu;
mod pause_menu;

pub mod plugin;
pub mod systems;

pub use main_menu::MainMenuPlugin;
pub use main_menu::MainMenuRoot;
pub use pause_menu::PauseMenuPlugin;
pub use pause_menu::PauseMenuRoot;
pub use plugin::UIPlugin;
pub use systems::{spawn_button, spawn_label, spawn_small_label, spawn_ui_screen};
