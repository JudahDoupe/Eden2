use bevy::prelude::*;
use crate::components::*;

pub fn update_tile_colors(
    mut tile_query: Query<(&Tile, &mut Sprite), With<TileSprite>>,
) {
    for (tile, mut sprite) in tile_query.iter_mut() {
        sprite.color = if tile.is_hovered {
            tile.tile_type.hover_color()
        } else {
            tile.tile_type.color()
        };
    }
}
