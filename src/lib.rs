pub mod components;
pub mod systems;
pub mod types;

pub use components::*;
pub use systems::*;
pub use types::*;

use bevy::prelude::*;

// Shared app configuration to ensure consistency between native and web builds
pub fn create_app(window_config: Window) -> App {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window_config),
        ..default()
    }))
    .add_systems(Startup, (setup, initialize_screen_layout).chain())
    .add_systems(Update, (
        handle_window_resize,
        handle_card_clicks,
        update_resource_display,
        update_species_display,
        update_hand_ui,
        update_hand_layout,
        update_card_visuals,
    ));
    
    app
}

// Native window configuration
pub fn native_window_config() -> Window {
    Window {
        title: "Eden2 - Ecosystem Card Game".to_string(),
        resolution: (400.0, 700.0).into(),
        ..default()
    }
}

// Web window configuration
#[cfg(target_arch = "wasm32")]
pub fn web_window_config() -> Window {
    Window {
        title: "Eden2 - Ecosystem Card Game".to_string(),
        canvas: Some("#bevy".to_string()),
        prevent_default_event_handling: false,
        fit_canvas_to_parent: true,
        ..default()
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Eden2 starting...".into());

    create_app(web_window_config()).run();
}
