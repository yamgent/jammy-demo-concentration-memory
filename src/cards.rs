use bevy::prelude::*;

use crate::{game_assets::SpritesAsset, game_state::GameState};

pub struct CardsPlugin;

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_card.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Copy, Clone)]
pub enum CardType {
    Club(u8),
    Diamond(u8),
    Heart(u8),
    Spade(u8),
    JokerRed,
    JokerBlack,
}

#[derive(Component)]
pub struct CardSpawn {
    pub open: bool,
    pub card_type: CardType,
}

const CARD_SPRITE_SIZE: f32 = 64.0;

fn get_card_sprite_rect(card_type: CardType) -> Rect {
    fn get_x_coord(val: u8) -> f32 {
        assert!(val >= 1 && val <= 13, "{val} is not a valid card value");
        CARD_SPRITE_SIZE * (val as f32 - 1.0)
    }

    let top_left = match card_type {
        CardType::Club(val) => Vec2::new(get_x_coord(val), CARD_SPRITE_SIZE * 2.0),
        CardType::Diamond(val) => Vec2::new(get_x_coord(val), CARD_SPRITE_SIZE),
        CardType::Heart(val) => Vec2::new(get_x_coord(val), 0.0),
        CardType::Spade(val) => Vec2::new(get_x_coord(val), CARD_SPRITE_SIZE * 3.0),
        CardType::JokerRed => Vec2::new(CARD_SPRITE_SIZE * 13.0, CARD_SPRITE_SIZE * 2.0),
        CardType::JokerBlack => Vec2::new(CARD_SPRITE_SIZE * 13.0, CARD_SPRITE_SIZE * 3.0),
    };

    Rect {
        min: top_left,
        max: top_left + Vec2::new(CARD_SPRITE_SIZE, CARD_SPRITE_SIZE),
    }
}

fn get_card_back_sprite_rect() -> Rect {
    let top_left = Vec2::new(CARD_SPRITE_SIZE * 13.0, CARD_SPRITE_SIZE * 1.0);

    Rect {
        min: top_left,
        max: top_left + Vec2::new(CARD_SPRITE_SIZE, CARD_SPRITE_SIZE),
    }
}

#[derive(Component)]
pub struct Card {
    open: bool,
    card_type: CardType,
}

fn spawn_card(
    mut commands: Commands,
    query: Query<(&CardSpawn, Entity)>,
    sprite_assets: Res<SpritesAsset>,
) {
    query.for_each(|(card_spawn, card_spawn_entity)| {
        let mut entity = commands.get_entity(card_spawn_entity).unwrap();
        entity.insert((
            SpriteBundle {
                texture: sprite_assets.cards.clone(),
                sprite: Sprite {
                    rect: Some(if card_spawn.open {
                        get_card_sprite_rect(card_spawn.card_type)
                    } else {
                        get_card_back_sprite_rect()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            },
            Card {
                open: card_spawn.open,
                card_type: card_spawn.card_type,
            },
        ));
        entity.remove::<CardSpawn>();
    });
}
