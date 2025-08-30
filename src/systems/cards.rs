use bevy::prelude::*;
use crate::{
    components::*,
    types::CardType,
};

// Handle clicking/touching cards to play them
pub fn handle_card_clicks(
    card_query: Query<(&Card, &Transform), With<CardSprite>>,
    mut game_state: ResMut<GameState>,
    mut garden_state: ResMut<GardenState>,
    screen_layout: Res<ScreenLayout>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
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
            let card_size = screen_layout.calculate_card_size(game_state.hand.len());
            
            // Check which card was clicked/touched
            for (card, transform) in card_query.iter() {
                if is_point_in_card(world_pos, transform.translation.truncate(), card_size) {
                    // Try to play the card
                    if card.hand_index < game_state.hand.len() {
                        let played_card = game_state.hand[card.hand_index];
                        
                        // Try to add the plant to the garden
                        match played_card {
                            CardType::Plant(plant_type) => {
                                if garden_state.add_plant(plant_type) {
                                    // Successfully played - now remove from hand and draw new
                                    game_state.play_card(card.hand_index);
                                    println!("Played {} successfully!", played_card.name());
                                } else {
                                    println!("Not enough resources to play {}!", played_card.name());
                                }
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
}

// Check if a point is inside a card
fn is_point_in_card(point: Vec2, card_center: Vec2, card_size: Vec2) -> bool {
    let half_size = card_size / 2.0;
    
    point.x >= card_center.x - half_size.x &&
    point.x <= card_center.x + half_size.x &&
    point.y >= card_center.y - half_size.y &&
    point.y <= card_center.y + half_size.y
}

// Update card visuals based on state
pub fn update_card_visuals(
    mut card_query: Query<(&Card, &mut Sprite), With<CardSprite>>,
) {
    for (card, mut sprite) in card_query.iter_mut() {
        sprite.color = if card.is_selected {
            Color::srgb(1.0, 1.0, 0.8) // Light yellow when selected
        } else {
            card.card_type.color()
        };
    }
}

// Spawn hand UI with cards
pub fn spawn_hand_ui(commands: &mut Commands, game_state: &GameState, screen_layout: &ScreenLayout) {
    let card_size = screen_layout.calculate_card_size(game_state.hand.len());
    let card_spacing = screen_layout.calculate_card_spacing(game_state.hand.len());
    let total_width = if game_state.hand.len() > 1 {
        (card_size.x * game_state.hand.len() as f32) + (card_spacing * (game_state.hand.len() - 1) as f32)
    } else {
        card_size.x
    };
    let start_x = -total_width / 2.0 + card_size.x / 2.0;
    
    for (index, card_type) in game_state.hand.iter().enumerate() {
        let x_position = start_x + (index as f32 * (card_size.x + card_spacing));
        
        // Spawn card background (green rectangle)
        let card_entity = commands.spawn((
            Sprite {
                color: card_type.color(),
                custom_size: Some(card_size),
                ..default()
            },
            Transform::from_translation(Vec3::new(x_position, screen_layout.card_area_y, 1.0)),
            Card {
                card_type: *card_type,
                hand_index: index,
                is_selected: false,
            },
            CardSprite, // Add this marker component for click detection
        )).id();
        
        // Calculate text size based on card size
        let text_size = screen_layout.text_font_size(FontSizeClass::Small);
        
        // Spawn card title text as a child of the card sprite
        let text_entity = commands.spawn((
            Text2d::new(card_type.name()),
            TextFont {
                font_size: text_size,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(0.0, card_size.y * 0.3, 1.0)), // Relative to parent card
            CardText,
        )).id();
        
        // Make text a child of the card
        commands.entity(card_entity).add_child(text_entity);
    }
}

// Update hand UI when cards change
pub fn update_hand_ui(
    mut commands: Commands,
    game_state: Res<GameState>,
    screen_layout: Res<ScreenLayout>,
    card_query: Query<Entity, With<CardSprite>>,
) {
    // Handle card updates when game state changes
    if game_state.is_changed() {
        // Remove old cards (children text will be automatically removed)
        for entity in card_query.iter() {
            commands.entity(entity).despawn(); // Automatically recursive in Bevy 0.16
        }
        
        // Spawn new hand
        spawn_hand_ui(&mut commands, &game_state, &screen_layout);
    }
}
