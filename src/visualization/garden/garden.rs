use bevy::prelude::*;
use crate::gameplay::garden::{Garden, resources::ResourceType};
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
    garden_state: Res<Garden>,
    mut text_query: Query<&mut Text2d, With<ResourceDisplayText>>,
) {
    if garden_state.is_changed() {
        let mut resource_text = String::from("Resources:");
        for resource_type in ResourceType::all() {
            let amount = garden_state.resources.get_resource(resource_type);
            resource_text.push_str(&format!("\n{}: {}", resource_type.name(), amount));
        }
        
        if let Ok(mut text) = text_query.single_mut() {
            **text = resource_text;
        }
    }
}

pub fn update_species_display(
    garden_state: Res<Garden>,
    mut text_query: Query<&mut Text2d, With<SpeciesDisplayText>>,
) {
    if garden_state.is_changed() {
        let mut species_text = String::from("Species:");
        
        if garden_state.is_empty() {
            species_text.push_str("\nNo species yet");
        } else {
            for species in garden_state.current_species() {
                species_text.push_str(&format!("\n{} ({})", 
                    species.name,
                    garden_state.species_population(&species.name)
                ));
            }
        }
        
        if let Ok(mut text) = text_query.single_mut() {
            **text = species_text;
        }
    }
}
