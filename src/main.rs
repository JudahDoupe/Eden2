use eden2::{create_app, native_window_config};
use crate::plugins::svg_rendering_plugin::SvgRenderingPlugin;
use crate::plugins::visualization_plugin::CreatureVisualizationPlugin;

fn main() {
    App::new()
        .add_plugins(SvgRenderingPlugin)
        .add_plugins(CreatureVisualizationPlugin)
        .run();
}
