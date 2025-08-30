use bevy::prelude::*;
use crate::{
    components::*,
    types::ResourceType,
};

// Simple placeholder systems to match lib.rs references
pub fn handle_window_resize() {
    // Placeholder - implement window resize handling if needed
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
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(-250.0, 150.0, 10.0)),
            ResourceDisplayText,
        ));
    }
}

// Update the species display when garden state changes
pub fn update_species_display(
    mut commands: Commands,
    garden_state: Res<GardenState>,
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
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(200.0, 150.0, 10.0)),
            SpeciesDisplayText,
        ));
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
