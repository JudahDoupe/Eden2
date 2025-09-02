use bevy::prelude::*;
use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::utils::BoxedFuture;
use std::io::Cursor;

/// Asset representation of a loaded SVG
#[derive(Debug, TypePath)]
pub struct SvgAsset {
    pub size: Vec2,
    pub texture: Handle<Image>,
}

/// Asset loader for SVG files
#[derive(Default)]
pub struct SvgAssetLoader;

impl AssetLoader for SvgAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            // Parse SVG data
            let opt = usvg::Options::default();
            let svg_tree = usvg::Tree::from_data(bytes, &opt)?;
            
            // Get dimensions
            let pixmap_size = svg_tree.size();
            let width = pixmap_size.width() as u32;
            let height = pixmap_size.height() as u32;
            
            // Create a pixel buffer to render the SVG
            let mut pixmap = tiny_skia::Pixmap::new(width, height)
                .ok_or_else(|| bevy::asset::Error::msg("Failed to create pixmap"))?;
            
            // Render SVG to the pixmap
            resvg::render(&svg_tree, usvg::FitTo::Original, pixmap.as_mut())
                .ok_or_else(|| bevy::asset::Error::msg("Failed to render SVG"))?;
            
            // Convert to RGBA bytes
            let rgba_bytes = pixmap.data();
            
            // Create a Bevy Image
            let image = Image::from_rgba8(width, height, rgba_bytes.to_vec());
            let texture_handle = load_context.set_labeled_asset("texture", LoadedAsset::new(image));
            
            // Create and return the final SvgAsset
            let svg_asset = SvgAsset {
                size: Vec2::new(width as f32, height as f32),
                texture: texture_handle,
            };
            
            load_context.set_default_asset(LoadedAsset::new(svg_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["svg"]
    }
}

/// Component to mark an entity as an SVG sprite
#[derive(Component, Default)]
pub struct SvgComponent;

/// Bundle for rendering SVGs as sprites
#[derive(Bundle, Default)]
pub struct SvgSpriteBundle {
    pub sprite: SpriteBundle,
    pub svg: SvgComponent,
}

// System to update sprites when SVG assets are loaded
pub fn update_svg_sprites(
    mut commands: Commands,
    svg_assets: Res<Assets<SvgAsset>>,
    mut query: Query<(Entity, &Handle<SvgAsset>, &mut Handle<Image>, &mut Sprite), With<SvgComponent>>,
) {
    for (entity, svg_handle, mut texture, mut sprite) in query.iter_mut() {
        if let Some(svg) = svg_assets.get(svg_handle) {
            // Update sprite with the loaded texture
            *texture = svg.texture.clone();
            
            // Update sprite size to match SVG aspect ratio
            sprite.custom_size = Some(svg.size);
        }
    }
}

// Plugin to register SVG asset loader and systems
pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SvgAsset>()
           .init_asset_loader::<SvgAssetLoader>()
           .register_type::<SvgComponent>()
           .add_systems(Update, update_svg_sprites);
    }
}
