use bevy::prelude::*;
use crate::creatures::svg_renderer::*;
use crate::rendering::svg::SvgPlugin;

/// Plugin for creature visualization using SVG sprites
pub struct CreatureVisualizationPlugin;

impl Plugin for CreatureVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add the SVG plugin for asset loading
            .add_plugins(SvgPlugin)
            
            // Initialize garden bounds resource
            .init_resource::<GardenBounds>()
            
            // Register visualization systems
            .add_systems(Update, (
                // Create visual entities for new creatures
                spawn_creature_visualizations,
                
                // Update visuals when lifecycle stage changes
                update_creature_visualizations,
                
                // Clean up visual entities when creatures are removed
                cleanup_visual_entities,
            ));
    }
}
