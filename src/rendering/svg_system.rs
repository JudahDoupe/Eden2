use bevy::prelude::*;
use crate::rendering::svg::{SvgAsset, SvgComponent};

/// System to update sprites when SVG assets are loaded
pub fn update_svg_sprites(
    mut commands: Commands,
    svg_assets: Res<Assets<SvgAsset>>,
    mut query: Query<(Entity, &Handle<SvgAsset>, &mut SpriteBundle), With<SvgComponent>>,
) {
    for (entity, svg_handle, mut sprite_bundle) in query.iter_mut() {
        if let Some(svg) = svg_assets.get(svg_handle) {
            // Update sprite with the loaded texture
            sprite_bundle.texture = svg.texture.clone();
            
            // Update sprite size to match SVG aspect ratio
            sprite_bundle.sprite.custom_size = Some(svg.size);
            
            // Remove the component to indicate this entity has been processed
            commands.entity(entity).remove::<SvgComponent>();
        }
    }
}
