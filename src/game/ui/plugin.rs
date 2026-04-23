use crate::prelude::*;
use univis_ui::prelude::UnivisUiPlugin;

use super::main_menu::MainMenuPlugin;
use super::pause_menu::PauseMenuPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UnivisUiPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(PauseMenuPlugin);
    }
}
