use bevy::prelude::App;
use bevy_playground::game::plugin::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
