use bevy::prelude::*;
use crate::{components::*, types::TileType};

pub fn update_stats(
    tile_query: Query<&Tile, With<TileSprite>>,
    mut text_query: Query<&mut Text, With<StatsText>>,
    game_state: Res<GameState>,
) {
    let mut empty_count = 0;
    let mut land_count = 0;
    let mut water_count = 0;
    let mut forest_count = 0;

    for tile in tile_query.iter() {
        match tile.tile_type {
            TileType::Empty => empty_count += 1,
            TileType::Land => land_count += 1,
            TileType::Water => water_count += 1,
            TileType::Forest => forest_count += 1,
        }
    }

    let selected_card_info = if let Some(index) = game_state.selected_card_index {
        if index < game_state.hand.len() {
            format!("\nSelected: {}", game_state.hand[index].name())
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let game_status = if !game_state.has_cards() {
        "\n\n*** GAME OVER ***\nNo more cards available!"
    } else if !game_state.can_play_cards() {
        "\n\nNo cards in hand! The game will end when all cards are used."
    } else {
        ""
    };

    for mut text in text_query.iter_mut() {
        **text = format!(
            "Eden2 Ecosystem Builder\nSelect a card, then click a tile to play it\n\nStats:\nEmpty: {}\nLand: {}\nWater: {}\nForest: {}\n\nCards in deck: {}\nCards in hand: {}{}{}",
            empty_count, land_count, water_count, forest_count,
            game_state.deck.len(), game_state.hand.len(), selected_card_info, game_status
        );
    }
}
