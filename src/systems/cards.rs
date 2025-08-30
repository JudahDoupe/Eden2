use bevy::prelude::*;
use crate::{
    components::*,
    constants::*,
    types::CardType,
};

pub fn handle_card_clicks(
    mut card_query: Query<(&mut Card, &Transform), With<CardSprite>>,
    mut game_state: ResMut<GameState>,
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
                for (mut card, transform) in card_query.iter_mut() {
                    if is_point_in_card(world_pos, transform.translation.truncate()) {
                        // Only allow card selection if hand has cards
                        if !game_state.can_play_cards() {
                            println!("No cards available to play!");
                            return;
                        }
                        
                        // Select/deselect card
                        if game_state.selected_card_index == Some(card.hand_index) {
                            game_state.selected_card_index = None;
                            card.is_selected = false;
                            println!("Deselected card");
                        } else {
                            game_state.selected_card_index = Some(card.hand_index);
                            card.is_selected = true;
                            println!("Selected card: {} (index: {})", card.card_type.name(), card.hand_index);
                        }
                        break;
                    }
                }
                
                // Update selection states for all cards
                for (mut card, _) in card_query.iter_mut() {
                    card.is_selected = game_state.selected_card_index == Some(card.hand_index);
                }
            }
        }
    }
}

pub fn handle_card_hover(
    mut card_query: Query<(&mut Card, &Transform), With<CardSprite>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = camera_query.single() else { return };

    // Reset all hover states
    for (mut card, _) in card_query.iter_mut() {
        card.is_hovered = false;
    }

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            // Check which card is being hovered
            for (mut card, transform) in card_query.iter_mut() {
                if is_point_in_card(world_pos, transform.translation.truncate()) {
                    card.is_hovered = true;
                    break;
                }
            }
        }
    }
}

pub fn update_card_visuals(
    mut card_query: Query<(&Card, &mut Sprite), With<CardSprite>>,
) {
    for (card, mut sprite) in card_query.iter_mut() {
        let base_color = card.card_type.color();
        
        if card.is_selected {
            // Make selected cards brighter
            sprite.color = Color::srgb(
                (base_color.to_srgba().red + 0.3).min(1.0),
                (base_color.to_srgba().green + 0.3).min(1.0),
                (base_color.to_srgba().blue + 0.3).min(1.0),
            );
        } else if card.is_hovered {
            // Make hovered cards slightly brighter
            sprite.color = Color::srgb(
                (base_color.to_srgba().red + 0.15).min(1.0),
                (base_color.to_srgba().green + 0.15).min(1.0),
                (base_color.to_srgba().blue + 0.15).min(1.0),
            );
        } else {
            sprite.color = base_color;
        }
    }
}

pub fn spawn_hand_ui(commands: &mut Commands, game_state: &GameState) {
    // Don't spawn cards if hand is empty
    if game_state.hand.is_empty() {
        return;
    }
    
    // Calculate hand positioning
    let hand_width = game_state.hand.len() as f32 * CARD_WIDTH + (game_state.hand.len() - 1) as f32 * CARD_SPACING;
    let start_x = -hand_width / 2.0 + CARD_WIDTH / 2.0;

    for (index, &card_type) in game_state.hand.iter().enumerate() {
        let pos_x = start_x + index as f32 * (CARD_WIDTH + CARD_SPACING);
        
        spawn_card(commands, card_type, index, pos_x, HAND_Y_OFFSET);
    }
}

pub fn update_hand_ui(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    card_query: Query<Entity, Or<(With<CardSprite>, With<CardText>)>>,
) {
    if game_state.is_changed() {
        // Clear invalid card selection if hand changed
        if let Some(selected_index) = game_state.selected_card_index {
            if selected_index >= game_state.hand.len() {
                game_state.selected_card_index = None;
            }
        }
        
        // Despawn all existing cards and text
        for entity in card_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Respawn cards with updated hand
        spawn_hand_ui(&mut commands, &game_state);
    }
}

pub fn check_game_end(
    game_state: Res<GameState>,
) {
    if game_state.is_changed() && !game_state.has_cards() {
        println!("Game Over! No more cards available.");
        println!("Final game state: {} cards in deck, {} cards in hand", 
            game_state.deck.len(), game_state.hand.len());
    }
}

fn spawn_card(commands: &mut Commands, card_type: CardType, hand_index: usize, x: f32, y: f32) {
    // Spawn card background
    commands.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(CARD_WIDTH + 4.0, CARD_HEIGHT + 4.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(x, y, 2.0)),
    ));

    // Spawn main card
    commands.spawn((
        Sprite {
            color: card_type.color(),
            custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
            ..default()
        },
        Transform::from_translation(Vec3::new(x, y, 3.0)),
        Card {
            card_type,
            hand_index,
            is_selected: false,
            is_hovered: false,
        },
        CardSprite,
    ));

    // Spawn card text (using world coordinates)
    commands.spawn((
        Text::new(format!("{}\n\n{}", card_type.name(), card_type.description())),
        Transform::from_translation(Vec3::new(x, y, 4.0)),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::WHITE),
        CardText,
    ));
}

fn is_point_in_card(point: Vec2, card_pos: Vec2) -> bool {
    let half_width = CARD_WIDTH / 2.0;
    let half_height = CARD_HEIGHT / 2.0;
    point.x >= card_pos.x - half_width
        && point.x <= card_pos.x + half_width
        && point.y >= card_pos.y - half_height
        && point.y <= card_pos.y + half_height
}
