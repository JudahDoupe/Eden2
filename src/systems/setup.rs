use bevy::prelude::*;
use crate::components::*;

pub fn setup(mut commands: Commands) {
    // Spawn camera - needed for any rendering
    commands.spawn(Camera2d);
    
    // Initialize and insert game states
    let mut game_state = GameState::default();
    game_state.draw_initial_hand();
    let garden_state = GardenState::default();
    commands.insert_resource(game_state.clone());
    commands.insert_resource(garden_state);
    
    // Spawn garden background - positioned in top 2/3rds
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.6, 0.2), // Garden green
            custom_size: Some(Vec2::new(600.0, 250.0)), // Reduced height for top 2/3rds
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)), // Moved up
    ));
    
    // Spawn resource display on the left (adjusted for new garden position)
    spawn_resource_display(&mut commands);
    
    // Spawn species display on the right (adjusted for new garden position)
    spawn_species_display(&mut commands);
    
    // Spawn cards in the bottom third
    spawn_hand_cards(&mut commands, &game_state);
}

fn spawn_resource_display(commands: &mut Commands) {
    commands.spawn((
        Text2d::new("Resources:\nWater: 5\nSunlight: 5\nNutrients: 5"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::new(-250.0, 150.0, 10.0)), // Adjusted for new garden position
        ResourceDisplayText,
    ));
}

fn spawn_species_display(commands: &mut Commands) {
    commands.spawn((
        Text2d::new("Plants:\nNo plants yet"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::new(200.0, 150.0, 10.0)), // Adjusted for new garden position
        SpeciesDisplayText,
    ));
}

fn spawn_hand_cards(commands: &mut Commands, game_state: &GameState) {
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
                color: Color::srgb(0.1, 0.4, 0.1), // Darker green for cards
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
