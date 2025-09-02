use bevy::prelude::*;
use crate::rendering::svg::SvgPlugin;
use crate::rendering::svg_system::update_svg_sprites;
use crate::creatures::svg_renderer::load_creature_svgs;

pub struct SvgRenderingPlugin;

impl Plugin for SvgRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SvgPlugin)
           .add_systems(Update, (
               load_creature_svgs,
               update_svg_sprites,
           ).chain());
    }
}
