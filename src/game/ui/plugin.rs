use crate::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            univis_ui::prelude::UnivisUiPlugin,
            crate::game::MainMenuPlugin,
            crate::game::PauseMenuPlugin,
        ));
    }
}
