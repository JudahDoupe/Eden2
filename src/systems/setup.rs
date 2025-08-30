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
    let garden_entity = commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.6, 0.2), // Garden green
            custom_size: Some(screen_layout.garden_area),
            ..default()
        },
        Transform::from_translation(Vec3::new(screen_layout.garden_center.x, screen_layout.garden_center.y, 0.0)),
        GardenBackground,
    )).id();
    
    // Spawn resource display as child of garden background (left half)
    let resource_text_entity = spawn_resource_display(&mut commands, &screen_layout);
    commands.entity(garden_entity).add_child(resource_text_entity);
    
    // Spawn species display as child of garden background (right half)  
    let species_text_entity = spawn_species_display(&mut commands, &screen_layout);
    commands.entity(garden_entity).add_child(species_text_entity);
    
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

fn spawn_resource_display(commands: &mut Commands, screen_layout: &ScreenLayout) -> Entity {    
    commands.spawn((
        Text2d::new("Resources:\nGround Water: 5\nSunlight: 5\nSoil Nutrients: 5\nCO₂: 10\nO₂: 10\nGreen Vegetation: 0\nFruit: 0\nDead Matter: 0\nPlant Population: 0\nAnimal Population: 0\nFungi Population: 0"),
        TextFont {
            font_size: screen_layout.text_font_size(FontSizeClass::Medium),
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Left),
        ResourceDisplayText,
    )).id()
}

fn spawn_species_display(commands: &mut Commands, screen_layout: &ScreenLayout) -> Entity {    
    commands.spawn((
        Text2d::new("Species:\nNo species yet"),
        TextFont {
            font_size: screen_layout.text_font_size(FontSizeClass::Medium),
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Right),
        SpeciesDisplayText,
    )).id()
}
