use bevy::prelude::*;
use eden2::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_window_resize,
            handle_tile_clicks, 
            handle_tile_hover, 
            handle_card_clicks,
            handle_card_hover,
            update_tile_colors,
            update_card_visuals,
            update_hand_ui,
            check_game_end,
            update_stats
        ))
        .run();
}
