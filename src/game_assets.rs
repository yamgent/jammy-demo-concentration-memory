use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game_state::GameState;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, SpritesAsset>(GameState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct SpritesAsset {
    #[asset(path = "sprites/cards.png")]
    pub cards: Handle<Image>,
}
