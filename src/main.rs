use bevy::prelude::*;
use eden2::create_app;

fn main() {
    // Create the application with default window configuration
    let mut app = create_app(Window {
        title: "Eden2 - Ecosystem Card Game".to_string(),
        ..default()
    });
    
    app.run();
}
