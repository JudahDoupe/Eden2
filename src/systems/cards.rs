use bevy::prelude::*;
use crate::{
    components::*,
    types::CardType,
};

// Handle clicking cards to play them
pub fn handle_card_clicks(
    card_query: Query<(&Card, &Transform), With<CardSprite>>,
    mut game_state: ResMut<GameState>,
    mut garden_state: ResMut<GardenState>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let Ok(window) = windows.single() else { return };
        let Ok((camera, camera_transform)) = camera_query.single() else { return };

        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                // Check which card was clicked
                for (card, transform) in card_query.iter() {
                    if is_point_in_card(world_pos, transform.translation.truncate()) {
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
}

// Check if a point is inside a card
fn is_point_in_card(point: Vec2, card_center: Vec2) -> bool {
    let card_size = Vec2::new(120.0, 160.0); // Card dimensions
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
pub fn spawn_hand_ui(commands: &mut Commands, game_state: &GameState) {
    let card_width = 120.0;
    let card_height = 160.0;
    let card_spacing = 140.0;
    let start_x = -(card_spacing * (game_state.hand.len() as f32 - 1.0)) / 2.0;
    let card_y = -150.0; // Bottom third of screen
    
    for (index, card_type) in game_state.hand.iter().enumerate() {
        let x_position = start_x + (index as f32 * card_spacing);
        
        // Spawn card background (green rectangle)
        commands.spawn((
            Sprite {
                color: card_type.color(),
                custom_size: Some(Vec2::new(card_width, card_height)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x_position, card_y, 1.0)),
            Card {
                card_type: *card_type,
                hand_index: index,
                is_selected: false,
            },
            CardSprite, // Add this marker component for click detection
        ));
        
        // Spawn card title text (white text at top of card)
        commands.spawn((
            Text2d::new(card_type.name()),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(x_position, card_y + 60.0, 2.0)), // Top of card
            CardText,
        ));
    }
}

// Update hand UI when cards change
pub fn update_hand_ui(
    mut commands: Commands,
    game_state: Res<GameState>,
    card_query: Query<Entity, With<CardSprite>>,
    text_query: Query<Entity, With<CardText>>,
) {
    if game_state.is_changed() {
        // Remove old cards and text
        for entity in card_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in text_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Spawn new hand
        spawn_hand_ui(&mut commands, &game_state);
    }
}
