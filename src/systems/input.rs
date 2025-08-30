use bevy::prelude::*;
use crate::{
    components::*,
    constants::TILE_SIZE,
};

#[cfg(target_arch = "wasm32")]
pub fn handle_window_resize(
    mut windows: Query<&mut Window>,
) {
    use web_sys;
    
    if let Ok(mut bevy_window) = windows.single_mut() {
        if let Some(web_window) = web_sys::window() {
            let inner_width = web_window.inner_width().unwrap().as_f64().unwrap() as f32;
            let inner_height = web_window.inner_height().unwrap().as_f64().unwrap() as f32;
            
            // Update Bevy window size if it doesn't match browser window
            if (bevy_window.width() - inner_width).abs() > 1.0 || (bevy_window.height() - inner_height).abs() > 1.0 {
                bevy_window.resolution.set(inner_width, inner_height);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn handle_window_resize() {
    // No-op for non-WASM builds
}

pub fn handle_tile_clicks(
    mut tile_query: Query<(&mut Tile, &Transform), With<TileSprite>>,
    mut card_query: Query<&mut Card, With<CardSprite>>,
    mut game_state: ResMut<GameState>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let Ok(window) = windows.single() else { return };
        let Ok((camera, camera_transform)) = camera_query.single() else { return };

        if let Some(cursor_pos) = window.cursor_position() {
            // Convert screen coordinates to world coordinates
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                // Check which tile was clicked
                for (mut tile, transform) in tile_query.iter_mut() {
                    if is_point_in_tile(world_pos, transform.translation.truncate()) {
                        // Only allow tile interaction if a card is selected
                        if let Some(selected_index) = game_state.selected_card_index {
                            // Check if the selected card index is valid
                            if selected_index < game_state.hand.len() {
                                // Play the selected card on this tile
                                if let Some(played_card) = game_state.play_card(selected_index) {
                                    tile.tile_type = played_card.target_tile_type();
                                    println!("Played {} on tile ({}, {}) - Changed to: {}", 
                                        played_card.name(), tile.x, tile.y, tile.tile_type.name());
                                    
                                    // Deselect all cards
                                    for mut card in card_query.iter_mut() {
                                        card.is_selected = false;
                                    }
                                }
                            } else {
                                println!("Invalid card selection!");
                                game_state.selected_card_index = None;
                            }
                        } else {
                            // No card selected - inform player they need to select a card first
                            println!("Please select a card first before clicking on tiles!");
                        }
                        break;
                    }
                }
            }
        }
    }
}

pub fn handle_tile_hover(
    mut tile_query: Query<(&mut Tile, &Transform), With<TileSprite>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = camera_query.single() else { return };

    // Reset all hover states
    for (mut tile, _) in tile_query.iter_mut() {
        tile.is_hovered = false;
    }

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            // Check which tile is being hovered
            for (mut tile, transform) in tile_query.iter_mut() {
                if is_point_in_tile(world_pos, transform.translation.truncate()) {
                    tile.is_hovered = true;
                    break;
                }
            }
        }
    }
}

fn is_point_in_tile(point: Vec2, tile_pos: Vec2) -> bool {
    let half_size = Vec2::splat(TILE_SIZE / 2.0);
    point.x >= tile_pos.x - half_size.x
        && point.x <= tile_pos.x + half_size.x
        && point.y >= tile_pos.y - half_size.y
        && point.y <= tile_pos.y + half_size.y
}
