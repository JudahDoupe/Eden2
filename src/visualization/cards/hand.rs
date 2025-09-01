use bevy::prelude::*;
use crate::gameplay::GameState;
use crate::gameplay::species::species::get_species;
use crate::visualization::ScreenLayout;
use crate::visualization::display::FontSizeClass;
use crate::visualization::ui::SelectedCard;
use super::{CardComponent, CardSprite, CardText};

/// Spawn hand UI with cards
pub fn init_hand_cards(commands: &mut Commands, game_state: &GameState, screen_layout: &ScreenLayout) {
    let card_size = screen_layout.calculate_card_size(game_state.hand.len());
    let card_spacing = screen_layout.calculate_card_spacing(game_state.hand.len());
    let total_width = if game_state.hand.len() > 1 {
        (card_size.x * game_state.hand.len() as f32) + (card_spacing * (game_state.hand.len() - 1) as f32)
    } else {
        card_size.x
    };
    let start_x = -total_width / 2.0 + card_size.x / 2.0;
    
    for (index, card) in game_state.hand.iter().enumerate() {
        let x_position = start_x + (index as f32 * (card_size.x + card_spacing));
        
        // Spawn card background (green rectangle)
        let card_entity = commands.spawn((
            Sprite {
                color: get_species(card.name()).expect("Species definition not found").color,
                custom_size: Some(card_size),
                ..default()
            },
            Transform::from_translation(Vec3::new(x_position, screen_layout.card_area_y, 1.0)),
            CardComponent {
                species: card.clone(),
                hand_index: index,
                is_selected: false,
            },
            CardSprite, // Add this marker component for click detection
        )).id();
        
        // Calculate text size based on card size
        let text_size = screen_layout.text_font_size(FontSizeClass::Small);
        
        // Spawn card title text as a child of the card sprite
        let text_entity = commands.spawn((
            Text2d::new(card.name()),
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

/// Update hand UI when cards change
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
            commands.entity(entity).despawn();
        }
        
        // Spawn new hand
        init_hand_cards(&mut commands, &game_state, &screen_layout);
    }
}

/// Update card positions when screen layout changes
pub fn update_hand_layout(
    mut card_query: Query<(&mut Transform, &mut Sprite, &CardComponent), With<CardSprite>>,
    mut text_query: Query<(&mut Transform, &mut TextFont), (With<CardText>, Without<CardSprite>)>,
    screen_layout: Res<ScreenLayout>,
    game_state: Res<GameState>,
    selected_card: Res<SelectedCard>,
) {
    if screen_layout.is_changed() || game_state.is_changed() {
        let card_size = screen_layout.calculate_card_size(game_state.hand.len());
        let card_spacing = screen_layout.calculate_card_spacing(game_state.hand.len());
        let total_width = if game_state.hand.len() > 1 {
            (card_size.x * game_state.hand.len() as f32) + (card_spacing * (game_state.hand.len() - 1) as f32)
        } else {
            card_size.x
        };
        let start_x = -total_width / 2.0 + card_size.x / 2.0;
        
        // Update card sprite positions and sizes (text will follow automatically as children)
        for (mut transform, mut sprite, card) in card_query.iter_mut() {
            let x_position = start_x + (card.hand_index as f32 * (card_size.x + card_spacing));
            let is_selected = selected_card.get_selected() == Some(card.hand_index);
            
            // Set position and size based on selection state
            transform.translation = Vec3::new(x_position, screen_layout.card_area_y, if is_selected { 2.0 } else { 1.0 });
            sprite.custom_size = Some(if is_selected { card_size * 1.3 } else { card_size });
        }
        
        // Update text font sizes (positions are handled by parent-child relationship)
        let text_size = screen_layout.text_font_size(FontSizeClass::Small);
        for (mut transform, mut text_font) in text_query.iter_mut() {
            text_font.font_size = text_size;
            // Update the relative position to account for new card size
            transform.translation = Vec3::new(0.0, card_size.y * 0.3, 1.0);
        }
    }
}
