pub mod core;
pub mod ui;
pub mod types;

pub use types::*;

use bevy::prelude::*;
use core::simulation::*;
use ui::systems::layout::setup_ui;
use ui::systems::*;

/// Creates the main Bevy app with shared configuration for native and web builds
pub fn create_app(window_config: Window) -> App {
    let mut app = App::new();
    
    // Add plugins and window configuration
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window_config),
        ..default()
    }));
    
    // Register events
    app.add_event::<CardPlayEvent>();
    app.add_event::<core::simulation_systems::AddSpeciesEvent>();
    app.add_event::<core::simulation_systems::TriggerSimulationEvent>();
    
    // Add resources
    app.init_resource::<core::GameState>();
    
    // Add startup systems
    app.add_systems(Startup, (
        setup_ui,
        initialize_screen_layout,
        core::simulation_systems::spawn_garden,
    ).chain());
    
    // Add update systems
    app.add_systems(Update, (
        // UI Systems
        handle_window_resize,
        handle_card_clicks,
        update_resource_display,
        update_species_display,
        update_hand_ui,
        update_hand_layout,
        update_card_visuals,
        // Core Game Systems
        handle_card_play,
        // Simulation Systems (order matters!)
        core::simulation_systems::handle_add_species,
        core::simulation_systems::trigger_simulation_on_card_play.after(core::simulation_systems::handle_add_species),
        core::simulation_systems::run_daily_simulation.after(core::simulation_systems::trigger_simulation_on_card_play),
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
