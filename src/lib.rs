pub mod components;
pub mod systems;
pub mod constants;
pub mod types;

pub use components::*;
pub use systems::*;
pub use constants::*;
pub use types::*;

#[cfg(target_arch = "wasm32")]
use bevy::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Eden2 starting...".into());

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Eden2 - Ecosystem Card Game".to_string(),
                canvas: Some("#bevy".to_string()),
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
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
