use bevy::prelude::*;
use bevy::window::{WindowResized, PrimaryWindow};
use crate::{
    components::*,
    types::ResourceType,
};

// System to initialize screen layout on startup
pub fn initialize_screen_layout(
    mut screen_layout: ResMut<ScreenLayout>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(window) = window_query.iter().next() {
        let window_size = Vec2::new(window.width(), window.height());
        if window_size.x > 0.0 && window_size.y > 0.0 {
            screen_layout.update_for_window_size(window_size);
        }
    }
    // If no window is found or window size is invalid, keep the default layout
}

// Window resize handling system
pub fn handle_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut screen_layout: ResMut<ScreenLayout>,
    mut layout_initialized: ResMut<LayoutInitialized>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut garden_query: Query<&mut Transform, With<GardenBackground>>,
    mut garden_sprite_query: Query<&mut Sprite, With<GardenBackground>>,
    mut resource_query: Query<&mut Transform, (With<ResourceDisplayText>, Without<GardenBackground>, Without<SpeciesDisplayText>)>,
    mut species_query: Query<&mut Transform, (With<SpeciesDisplayText>, Without<GardenBackground>, Without<ResourceDisplayText>)>,
) {
    let mut layout_updated = false;
    
    // Handle window resize events
    for event in resize_events.read() {
        let new_size = Vec2::new(event.width, event.height);
        screen_layout.update_for_window_size(new_size);
        layout_updated = true;
    }
    
    // If not initialized yet, try to initialize from current window
    if !layout_initialized.0 {
        if let Some(window) = window_query.iter().next() {
            let window_size = Vec2::new(window.width(), window.height());
            if window_size.x > 0.0 && window_size.y > 0.0 && window_size != Vec2::new(800.0, 600.0) {
                screen_layout.update_for_window_size(window_size);
                layout_initialized.0 = true;
                layout_updated = true;
            }
        }
    }
    
    // Update UI elements if layout was updated
    if layout_updated {
        // Update garden background size and position
        for mut transform in garden_query.iter_mut() {
            transform.translation = Vec3::new(screen_layout.garden_center.x, screen_layout.garden_center.y, 0.0);
        }
        
        for mut sprite in garden_sprite_query.iter_mut() {
            sprite.custom_size = Some(screen_layout.garden_area);
        }
        
        // Update relative positions of child text elements when garden size changes
        for mut transform in resource_query.iter_mut() {
            transform.translation = Vec3::new(
                -screen_layout.garden_area.x / 3.0,
                0.0,
                1.0
            );
        }
        
        for mut transform in species_query.iter_mut() {
            transform.translation = Vec3::new(
                screen_layout.garden_area.x / 3.0,
                0.0,
                1.0
            );
        }
    }
}

// Update the resource display when garden state changes
pub fn update_resource_display(
    garden_state: Res<GardenState>,
    mut text_query: Query<&mut Text2d, With<ResourceDisplayText>>,
) {
    if garden_state.is_changed() {
        // Create new resource display text
        let mut resource_text = String::from("Resources:");
        for resource_type in ResourceType::all() {
            let amount = garden_state.get_resource(resource_type);
            resource_text.push_str(&format!("\n{}: {}", resource_type.name(), amount));
        }
        
        // Update existing text (entity is created in setup, so it always exists)
        if let Ok(mut text) = text_query.single_mut() {
            **text = resource_text;
        }
    }
}

// Update the species display when garden state changes
pub fn update_species_display(
    garden_state: Res<GardenState>,
    mut text_query: Query<&mut Text2d, With<SpeciesDisplayText>>,
) {
    if garden_state.is_changed() {
        // Create new species display text
        let mut species_text = String::from("Species:");
        if garden_state.species.is_empty() {
            species_text.push_str("\nNo species yet");
        } else {
            for instance in &garden_state.species {
                species_text.push_str(&format!("\n{} ({})", 
                    instance.species_type.name(), 
                    instance.population
                ));
            }
        }
        
        // Update existing text (entity is created in setup, so it always exists)
        if let Ok(mut text) = text_query.single_mut() {
            **text = species_text;
        }
    }
}

// Update card positions when screen layout changes
pub fn update_hand_layout(
    mut card_query: Query<(&mut Transform, &mut Sprite, &Card), With<CardSprite>>,
    mut text_query: Query<(&mut Transform, &mut TextFont), (With<CardText>, Without<CardSprite>)>,
    screen_layout: Res<ScreenLayout>,
    game_state: Res<GameState>,
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
            transform.translation = Vec3::new(x_position, screen_layout.card_area_y, 1.0);
            sprite.custom_size = Some(card_size);
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

// Display the garden state as text
pub fn update_garden_display(
    mut commands: Commands,
    garden_state: Res<GardenState>,
    screen_layout: Res<ScreenLayout>,
    mut text_query: Query<&mut Text2d, With<GardenText>>,
    garden_background_query: Query<Entity, With<GardenBackground>>,
) {
    if garden_state.is_changed() {
        // Create resource display text
        let mut resource_text = String::from("Resources:\n");
        for resource_type in ResourceType::all() {
            let amount = garden_state.get_resource(resource_type);
            resource_text.push_str(&format!("{}: {}\n", resource_type.name(), amount));
        }
        
        // Create species display text
        let mut species_text = String::from("\nSpecies in Garden:\n");
        if garden_state.species.is_empty() {
            species_text.push_str("No species yet");
        } else {
            for instance in &garden_state.species {
                species_text.push_str(&format!("{} ({})\n", 
                    instance.species_type.name(), 
                    instance.population
                ));
            }
        }
        
        let full_text = format!("{}{}", resource_text, species_text);
        
        // Try to update existing text first
        if let Ok(mut text) = text_query.single_mut() {
            **text = full_text;
        } else {
            // No existing text, create new one as child of garden background
            if let Ok(garden_entity) = garden_background_query.single() {
                let garden_text_entity = commands.spawn((
                    Text2d::new(full_text),
                    TextFont {
                        font_size: screen_layout.text_font_size(FontSizeClass::Medium),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)), // Relative to garden center
                    GardenText,
                )).id();
                
                // Make garden text a child of the garden background
                commands.entity(garden_entity).add_child(garden_text_entity);
            }
        }
    }
}
