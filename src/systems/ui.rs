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
    mut garden_query: Query<&mut Transform, (With<GardenBackground>, Without<ResourceDisplayText>, Without<SpeciesDisplayText>)>,
    mut resource_query: Query<&mut Transform, (With<ResourceDisplayText>, Without<GardenBackground>, Without<SpeciesDisplayText>)>,
    mut species_query: Query<&mut Transform, (With<SpeciesDisplayText>, Without<GardenBackground>, Without<ResourceDisplayText>)>,
    mut garden_sprite_query: Query<&mut Sprite, With<GardenBackground>>,
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
        
        // Update resource text position
        for mut transform in resource_query.iter_mut() {
            transform.translation = screen_layout.resource_text_position();
        }
        
        // Update species text position
        for mut transform in species_query.iter_mut() {
            transform.translation = screen_layout.species_text_position();
        }
    }
}

pub fn handle_garden_clicks() {
    // Placeholder - implement garden click handling if needed
}

pub fn handle_garden_hover() {
    // Placeholder - implement garden hover handling if needed
}

pub fn handle_card_hover() {
    // Placeholder - implement card hover handling if needed
}

pub fn update_resource_text() {
    // Placeholder - implement resource text updates if needed
}

pub fn update_species_text() {
    // Placeholder - implement species text updates if needed
}

pub fn check_game_end() {
    // Placeholder - implement game end checking if needed
}

pub fn update_stats() {
    // Placeholder - implement stats updates if needed
}

// Update the resource display when garden state changes
pub fn update_resource_display(
    mut commands: Commands,
    garden_state: Res<GardenState>,
    screen_layout: Res<ScreenLayout>,
    text_query: Query<Entity, With<ResourceDisplayText>>,
) {
    if garden_state.is_changed() {
        // Remove old text
        for entity in text_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Create new resource display text
        let mut resource_text = String::from("Resources:");
        for resource_type in ResourceType::all() {
            let amount = garden_state.get_resource(resource_type);
            resource_text.push_str(&format!("\n{}: {}", resource_type.name(), amount));
        }
        
        commands.spawn((
            Text2d::new(resource_text),
            TextFont {
                font_size: screen_layout.text_font_size(FontSizeClass::Medium),
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(screen_layout.resource_text_position()),
            ResourceDisplayText,
        ));
    }
}

// Update the species display when garden state changes
pub fn update_species_display(
    mut commands: Commands,
    garden_state: Res<GardenState>,
    screen_layout: Res<ScreenLayout>,
    text_query: Query<Entity, With<SpeciesDisplayText>>,
) {
    if garden_state.is_changed() {
        // Remove old text
        for entity in text_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Create new species display text
        let mut plants_text = String::from("Plants:");
        if garden_state.plants.is_empty() {
            plants_text.push_str("\nNo plants yet");
        } else {
            for plant in &garden_state.plants {
                plants_text.push_str(&format!("\n{}", plant.name()));
            }
        }
        
        commands.spawn((
            Text2d::new(plants_text),
            TextFont {
                font_size: screen_layout.text_font_size(FontSizeClass::Medium),
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(screen_layout.species_text_position()),
            SpeciesDisplayText,
        ));
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
    text_query: Query<Entity, With<GardenText>>,
) {
    if garden_state.is_changed() {
        // Remove old garden text
        for entity in text_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Create resource display text
        let mut resource_text = String::from("Resources:\n");
        for resource_type in ResourceType::all() {
            let amount = garden_state.get_resource(resource_type);
            resource_text.push_str(&format!("{}: {}\n", resource_type.name(), amount));
        }
        
        // Create plants display text
        let mut plants_text = String::from("\nPlants in Garden:\n");
        if garden_state.plants.is_empty() {
            plants_text.push_str("No plants yet");
        } else {
            for plant in &garden_state.plants {
                plants_text.push_str(&format!("{}\n", plant.name()));
            }
        }
        
        let full_text = format!("{}{}", resource_text, plants_text);
        
        // Spawn the text using correct Bevy 0.16 Text2d syntax
        commands.spawn((
            Text2d::new(full_text),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(0.0, 50.0, 10.0)),
            GardenText,
        ));
    }
}
