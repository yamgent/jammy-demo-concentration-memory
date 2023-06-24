use bevy::prelude::*;
use jammy_demo_concentration_memory::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
