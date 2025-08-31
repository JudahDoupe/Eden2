use bevy::prelude::*;
use crate::core::simulation::CardPlayEvent;
use crate::ui::{CardComponent, CardSprite, ScreenLayout};

/// Handles clicking/touching cards to play them
pub fn handle_card_clicks(
    card_query: Query<(&CardComponent, &Transform), With<CardSprite>>,
    screen_layout: Res<ScreenLayout>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut card_play_events: EventWriter<CardPlayEvent>,
) {
    // Get required components
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = camera_query.single() else { return };
    
    // Check for user interaction (mouse or touch)
    let interaction_pos = get_interaction_position(&mouse_input, &touches, &window);
    
    if let Some(screen_pos) = interaction_pos {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            handle_card_interaction(world_pos, &card_query, &screen_layout, &mut card_play_events);
        }
    }
}

/// Gets the position of user interaction (mouse click or touch)
fn get_interaction_position(
    mouse_input: &ButtonInput<MouseButton>,
    touches: &Touches,
    window: &Window,
) -> Option<Vec2> {
    // Check for mouse click
    if mouse_input.just_pressed(MouseButton::Left) {
        return window.cursor_position();
    }
    
    // Check for touch input (handle first touch only)
    touches.iter_just_pressed().next().map(|touch| touch.position())
}

/// Handles interaction with cards at the given world position
fn handle_card_interaction(
    world_pos: Vec2,
    card_query: &Query<(&CardComponent, &Transform), With<CardSprite>>,
    screen_layout: &ScreenLayout,
    card_play_events: &mut EventWriter<CardPlayEvent>,
) {
    for (card, transform) in card_query.iter() {
        if is_point_in_card(world_pos, transform.translation.truncate(), screen_layout, card.hand_index) {
            card_play_events.write(CardPlayEvent {
                hand_index: card.hand_index,
            });
            break;
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
