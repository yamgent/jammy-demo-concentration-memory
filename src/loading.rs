use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{game_assets::GameAssetsPlugin, game_state::GameState};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Continue to menu state instead
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        )
        .add_plugin(GameAssetsPlugin);
    }
}
