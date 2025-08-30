use bevy::prelude::*;
use crate::components::*;

pub fn setup(mut commands: Commands) {
    // Spawn camera - needed for any rendering
    commands.spawn(Camera2d);
    
    // Initialize and insert game states and screen layout
    let mut game_state = GameState::default();
    game_state.draw_initial_hand();
    let garden_state = GardenState::default();
    let screen_layout = ScreenLayout::default();
    
    commands.insert_resource(game_state.clone());
    commands.insert_resource(garden_state);
    commands.insert_resource(screen_layout.clone());
    commands.insert_resource(LayoutInitialized::default());
    
    // Spawn garden background - positioned dynamically
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.6, 0.2), // Garden green
            custom_size: Some(screen_layout.garden_area),
            ..default()
        },
        Transform::from_translation(Vec3::new(screen_layout.garden_center.x, screen_layout.garden_center.y, 0.0)),
        GardenBackground,
    ));
    
    // Spawn resource display on the left (using dynamic positioning)
    spawn_resource_display(&mut commands, &screen_layout);
    
    // Spawn species display on the right (using dynamic positioning)
    spawn_species_display(&mut commands, &screen_layout);
    
    // Re-enable card spawning in setup now that we have proper parent-child relationships
    spawn_hand_cards(&mut commands, &game_state, &screen_layout);
}

fn spawn_hand_cards(commands: &mut Commands, game_state: &GameState, screen_layout: &ScreenLayout) {
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
                color: Color::srgb(0.1, 0.4, 0.1), // Darker green for cards
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

fn spawn_resource_display(commands: &mut Commands, screen_layout: &ScreenLayout) {
    commands.spawn((
        Text2d::new("Resources:\nWater: 5\nSunlight: 5\nNutrients: 5"),
        TextFont {
            font_size: screen_layout.text_font_size(FontSizeClass::Medium),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(screen_layout.resource_text_position()),
        ResourceDisplayText,
    ));
}

fn spawn_species_display(commands: &mut Commands, screen_layout: &ScreenLayout) {
    commands.spawn((
        Text2d::new("Plants:\nNo plants yet"),
        TextFont {
            font_size: screen_layout.text_font_size(FontSizeClass::Medium),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(screen_layout.species_text_position()),
        SpeciesDisplayText,
    ));
}
