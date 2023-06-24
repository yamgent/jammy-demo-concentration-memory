use bevy::prelude::*;

use crate::{
    cards::{CardSpawn, CardType},
    game_state::GameState,
};

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_dummy.in_schedule(OnEnter(GameState::Playing)));
    }
}

// TODO: Remove this once done testing
fn setup_dummy(mut commands: Commands) {
    commands.spawn(CardSpawn {
        open: true,
        card_type: CardType::Club(1),
    });
}
