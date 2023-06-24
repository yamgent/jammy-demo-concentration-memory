use bevy::prelude::*;

mod game_assets;
mod game_state;
mod loading;

use game_state::GameState;
use loading::LoadingPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugin(LoadingPlugin);
    }
}
