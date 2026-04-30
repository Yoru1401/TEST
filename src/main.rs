use bevy::prelude::App;
use bevy_playground::game::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
