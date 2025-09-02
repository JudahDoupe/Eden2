use bevy::prelude::*;

/// Plugin for creature visualization using SVG sprites
pub struct CreatureVisualizationPlugin;

impl Plugin for CreatureVisualizationPlugin {
    fn build(&self, _app: &mut App) {
        // Implementation currently disabled - will be enabled when SVG rendering is fixed
        #[cfg(feature = "svg_rendering")]
        {
            // SVG visualization implementation will go here when fixed
        }
    }
}
