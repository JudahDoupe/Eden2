use bevy::prelude::*;
use crate::core::GardenState;
use crate::ui::{ResourceDisplayText, SpeciesDisplayText};
use crate::types::ResourceType;

/// Update the resource display when garden state changes
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

/// Update the species display when garden state changes
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
