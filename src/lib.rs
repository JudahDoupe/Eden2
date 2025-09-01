pub mod gameplay;
pub mod visualization;

use bevy::prelude::*;
use gameplay::garden::*;
use gameplay::cards::{PlayCardEvent, DiscardCardEvent, handle_play_card_event, handle_discard_card_event};
use visualization::init_ui_elements;
use visualization::*;

/// Creates the main Bevy app with shared configuration for native and web builds
pub fn create_app(window_config: Window) -> App {
    let mut app = App::new();
    
    // Add plugins and window configuration
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window_config),
        ..default()
    }));
    
    // Register events
    app.add_event::<PlayCardEvent>();
    app.add_event::<DiscardCardEvent>();
    app.add_event::<AddSpeciesToGardenEvent>();
    app.add_event::<SimulateDayEvent>();
    
    // Add resources
    app.init_resource::<gameplay::GameState>();
    app.init_resource::<gameplay::garden::Garden>();
    app.init_resource::<visualization::display::ScreenLayout>();
    app.init_resource::<SelectedCard>();
    
    // Add startup systems
    app.add_systems(Startup, (
        init_ui_elements,
        init_screen_layout,
    ).chain());
    
    // Add update systems
    app.add_systems(Update, (
        // UI Systems
        handle_window_resize,
        handle_card_clicks,
        handle_button_clicks,
        update_button_visuals,
        update_button_layout,
        update_resource_display,
        update_species_display,
        update_hand_ui,
        update_hand_layout,
        update_card_visuals,
        clear_selection_after_actions,
        // Core Game Systems
        handle_play_card_event,
        handle_discard_card_event,
        handle_add_species_to_garden_event,
        handle_simulate_day_event,
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
