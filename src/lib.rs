use bevy::prelude::*;

mod cards;
mod game_assets;
mod game_state;
mod loading;
mod playing;

use cards::CardsPlugin;
use game_state::GameState;
use loading::LoadingPlugin;
use playing::PlayingPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(PlayingPlugin)
            .add_plugin(CardsPlugin)
            .add_startup_system(setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    // TODO: How if we use jammy loading screen?
    commands.spawn(Camera2dBundle::default());
}
