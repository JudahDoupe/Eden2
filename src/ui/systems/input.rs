use bevy::prelude::*;
use crate::core::simulation::CardPlayEvent;
use crate::ui::{Card, CardSprite, ScreenLayout};

/// Handle clicking/touching cards to play them
pub fn handle_card_clicks(
    card_query: Query<(&Card, &Transform), With<CardSprite>>,
    screen_layout: Res<ScreenLayout>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut card_play_events: EventWriter<CardPlayEvent>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = camera_query.single() else { return };
    
    // Handle mouse click
    let mut interaction_pos = None;
    
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window.cursor_position() {
            interaction_pos = Some(cursor_pos);
        }
    }
    
    // Handle touch input - check for any touch that just started
    for touch in touches.iter_just_pressed() {
        interaction_pos = Some(touch.position());
        break; // Only handle first touch for simplicity
    }
    
    // Process the interaction if we have a position
    if let Some(screen_pos) = interaction_pos {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            // Check which card was clicked/touched
            for (card, transform) in card_query.iter() {
                if is_point_in_card(world_pos, transform.translation.truncate(), &screen_layout, card.hand_index) {
                    // Send event to play the card
                    card_play_events.write(CardPlayEvent {
                        hand_index: card.hand_index,
                    });
                    break;
                }
            }
        }
    }
}

/// Check if a point is inside a card
fn is_point_in_card(point: Vec2, card_center: Vec2, screen_layout: &ScreenLayout, _hand_index: usize) -> bool {
    // We need to calculate the card size based on current hand size
    // This is a bit of a hack since we don't have access to GameState here
    // In a real implementation, you might want to store card size on the Card component
    let card_size = screen_layout.calculate_card_size(5); // Assume max hand size for now
    let half_size = card_size / 2.0;
    
    point.x >= card_center.x - half_size.x &&
    point.x <= card_center.x + half_size.x &&
    point.y >= card_center.y - half_size.y &&
    point.y <= card_center.y + half_size.y
}
