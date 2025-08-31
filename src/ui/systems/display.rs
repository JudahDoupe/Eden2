use bevy::prelude::*;
use crate::ui::components::{ResourceDisplayText, SpeciesDisplayText, Garden, GardenResources, Species};
use crate::types::ResourceType;

/// Update the resource display when garden state changes
pub fn update_resource_display(
    garden_query: Query<&GardenResources, (With<Garden>, Changed<GardenResources>)>,
    mut text_query: Query<&mut Text2d, With<ResourceDisplayText>>,
) {
    if let Ok(garden_resources) = garden_query.single() {
        // Create new resource display text
        let mut resource_text = String::from("Resources:");
        for resource_type in ResourceType::all() {
            let amount = garden_resources.get_resource(resource_type);
            resource_text.push_str(&format!("\n{}: {}", resource_type.name(), amount));
        }
        
        // Update existing text (entity is created in setup, so it always exists)
        if let Ok(mut text) = text_query.single_mut() {
            **text = resource_text;
        }
    }
}

/// Update the species display when species change
pub fn update_species_display(
    species_query: Query<&Species>,
    mut text_query: Query<&mut Text2d, With<SpeciesDisplayText>>,
) {
    // Always update - no change detection so we catch all changes
    let mut species_text = String::from("Species:");
    
    let species_list: Vec<&Species> = species_query.iter().collect();
    
    if species_list.is_empty() {
        species_text.push_str("\nNo species yet");
    } else {
        for species in species_list {
            species_text.push_str(&format!("\n{} ({})", 
                species.card.name(), 
                species.population
            ));
        }
    }
    
    // Update existing text (entity is created in setup, so it always exists)
    if let Ok(mut text) = text_query.single_mut() {
        **text = species_text;
    }
}
