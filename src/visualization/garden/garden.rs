use bevy::prelude::*;
use crate::gameplay::lifecycle::{EcosystemPopulation, MatterType};
use crate::visualization::display::{ScreenLayout};
use super::super::display::responsive_size_utils::FontSizeClass;

/// UI Component marker for garden background
#[derive(Component)]
pub struct GardenBackground;

/// UI Component marker for resource display text
#[derive(Component)]
pub struct ResourceDisplayText;

/// UI Component marker for species display text
#[derive(Component)]
pub struct SpeciesDisplayText;

/// Resource flag to track if layout has been initialized to actual window size
#[derive(Resource, Default)]
pub struct LayoutInitialized(pub bool);

pub fn init_garden_ui(commands: &mut Commands, screen_layout: &ScreenLayout) -> Entity {
    let garden_background_entity = commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.6, 0.2), // Garden green
            custom_size: Some(screen_layout.garden_area),
            ..default()
        },
        Transform::from_translation(Vec3::new(screen_layout.garden_center.x, screen_layout.garden_center.y, 0.0)),
        GardenBackground,
    )).id();
    
    let resource_text_entity = commands.spawn((
        Text2d::new("Resources: Loading..."),
        TextFont {
            font_size: screen_layout.text_font_size(FontSizeClass::Medium),
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Left),
        ResourceDisplayText,
    )).id();
    commands.entity(garden_background_entity).add_child(resource_text_entity);
    
    let species_text_entity =  commands.spawn((
        Text2d::new("Species:\nNo species yet"),
        TextFont {
            font_size: screen_layout.text_font_size(FontSizeClass::Medium),
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Right),
        SpeciesDisplayText,
    )).id();
    commands.entity(garden_background_entity).add_child(species_text_entity);
    
    garden_background_entity
}

pub fn update_resource_display(
    ecosystem_state: Res<EcosystemPopulation>,
    mut text_query: Query<&mut Text2d, With<ResourceDisplayText>>,
) {
    if ecosystem_state.is_changed() {
        let mut resource_text = String::from("Ecosystem Matter:");
        let matter = &ecosystem_state.ecosystem_matter;
        
        resource_text.push_str(&format!("\nSoil Nutrients: {}", matter.get_amount(MatterType::SoilNutrients)));
        resource_text.push_str(&format!("\nDead Plant Matter: {}", matter.get_amount(MatterType::DeadPlantMatter)));
        resource_text.push_str(&format!("\nDead Animal Matter: {}", matter.get_amount(MatterType::DeadAnimalMatter)));
        
        // Calculate total living matter
        let biomass = ecosystem_state.total_living_biomass();
        let plant_matter = biomass.get(&MatterType::PlantMatter).unwrap_or(&0);
        let animal_matter = biomass.get(&MatterType::AnimalMatter).unwrap_or(&0);
        
        resource_text.push_str(&format!("\nLiving Plant Matter: {}", plant_matter));
        resource_text.push_str(&format!("\nLiving Animal Matter: {}", animal_matter));
        resource_text.push_str(&format!("\nDay: {}", ecosystem_state.current_day));
        
        if let Ok(mut text) = text_query.single_mut() {
            **text = resource_text;
        }
    }
}

pub fn update_species_display(
    ecosystem_state: Res<EcosystemPopulation>,
    mut text_query: Query<&mut Text2d, With<SpeciesDisplayText>>,
) {
    if ecosystem_state.is_changed() {
        let mut species_text = String::from("Species:");
        
        if ecosystem_state.creatures.is_empty() {
            species_text.push_str("\nNo species yet");
        } else {
            // Group creatures by species and count them
            for (species_name, count) in &ecosystem_state.living_population_by_species {
                if *count > 0 {
                    species_text.push_str(&format!("\n{}: {}", species_name, count));
                }
            }
            
            // Show total creature count
            let total_creatures = ecosystem_state.living_creatures().count();
            species_text.push_str(&format!("\nTotal: {} creatures", total_creatures));
        }
        
        if let Ok(mut text) = text_query.single_mut() {
            **text = species_text;
        }
    }
}
