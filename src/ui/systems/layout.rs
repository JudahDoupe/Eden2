use bevy::prelude::*;
use bevy::window::{WindowResized, PrimaryWindow};
use crate::core::{GameState, GardenState};
use crate::ui::{ScreenLayout, LayoutInitialized, GardenBackground, ResourceDisplayText, SpeciesDisplayText};
use crate::ui::systems::cards::spawn_hand_cards;

/// System to initialize screen layout on startup
pub fn initialize_screen_layout(
    mut screen_layout: ResMut<ScreenLayout>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(window) = window_query.iter().next() {
        let window_size = Vec2::new(window.width(), window.height());
        if window_size.x > 0.0 && window_size.y > 0.0 {
            screen_layout.update_for_window_size(window_size);
        }
    }
    // If no window is found or window size is invalid, keep the default layout
}

/// Window resize handling system
pub fn handle_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut screen_layout: ResMut<ScreenLayout>,
    mut layout_initialized: ResMut<LayoutInitialized>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut garden_query: Query<&mut Transform, With<GardenBackground>>,
    mut garden_sprite_query: Query<&mut Sprite, With<GardenBackground>>,
    mut resource_query: Query<&mut Transform, (With<ResourceDisplayText>, Without<GardenBackground>, Without<SpeciesDisplayText>)>,
    mut species_query: Query<&mut Transform, (With<SpeciesDisplayText>, Without<GardenBackground>, Without<ResourceDisplayText>)>,
) {
    let mut layout_updated = false;
    
    // Handle window resize events
    for event in resize_events.read() {
        let new_size = Vec2::new(event.width, event.height);
        screen_layout.update_for_window_size(new_size);
        layout_updated = true;
    }
    
    // If not initialized yet, try to initialize from current window
    if !layout_initialized.0 {
        if let Some(window) = window_query.iter().next() {
            let window_size = Vec2::new(window.width(), window.height());
            if window_size.x > 0.0 && window_size.y > 0.0 && window_size != Vec2::new(800.0, 600.0) {
                screen_layout.update_for_window_size(window_size);
                layout_initialized.0 = true;
                layout_updated = true;
            }
        }
    }
    
    // Update UI elements if layout was updated
    if layout_updated {
        // Update garden background size and position
        for mut transform in garden_query.iter_mut() {
            transform.translation = Vec3::new(screen_layout.garden_center.x, screen_layout.garden_center.y, 0.0);
        }
        
        for mut sprite in garden_sprite_query.iter_mut() {
            sprite.custom_size = Some(screen_layout.garden_area);
        }
        
        // Update relative positions of child text elements when garden size changes
        for mut transform in resource_query.iter_mut() {
            transform.translation = Vec3::new(
                -screen_layout.garden_area.x / 4.0,
                0.0,
                1.0
            );
        }
        
        for mut transform in species_query.iter_mut() {
            transform.translation = Vec3::new(
                screen_layout.garden_area.x / 4.0,
                0.0,
                1.0
            );
        }
    }
}

/// Initial setup system for UI elements
pub fn setup_ui(mut commands: Commands) {
    // Spawn camera - needed for any rendering
    commands.spawn(Camera2d);
    
    // Initialize and insert game states and screen layout
    let mut game_state = GameState::default();
    game_state.draw_initial_hand();
    let garden_state = GardenState::default();
    let screen_layout = ScreenLayout::default();
    
    commands.insert_resource(game_state.clone());
    commands.insert_resource(garden_state);
    commands.insert_resource(screen_layout.clone());
    commands.insert_resource(LayoutInitialized::default());
    
    // Spawn garden background - positioned dynamically
    let garden_entity = commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.6, 0.2), // Garden green
            custom_size: Some(screen_layout.garden_area),
            ..default()
        },
        Transform::from_translation(Vec3::new(screen_layout.garden_center.x, screen_layout.garden_center.y, 0.0)),
        GardenBackground,
    )).id();
    
    // Spawn resource display as child of garden background (left half)
    let resource_text_entity = spawn_resource_display(&mut commands, &screen_layout);
    commands.entity(garden_entity).add_child(resource_text_entity);
    
    // Spawn species display as child of garden background (right half)  
    let species_text_entity = spawn_species_display(&mut commands, &screen_layout);
    commands.entity(garden_entity).add_child(species_text_entity);
    
    // Spawn hand cards
    spawn_hand_cards(&mut commands, &game_state, &screen_layout);
}

fn spawn_resource_display(commands: &mut Commands, screen_layout: &ScreenLayout) -> Entity {    
    commands.spawn((
        Text2d::new("Resources:\nGround Water: 5\nSunlight: 5\nSoil Nutrients: 5\nCO₂: 10\nO₂: 10\nGreen Vegetation: 0\nFruit: 0\nDead Matter: 0\nPlant Population: 0\nAnimal Population: 0\nFungi Population: 0"),
        TextFont {
            font_size: screen_layout.text_font_size(crate::ui::FontSizeClass::Medium),
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Left),
        ResourceDisplayText,
    )).id()
}

fn spawn_species_display(commands: &mut Commands, screen_layout: &ScreenLayout) -> Entity {    
    commands.spawn((
        Text2d::new("Species:\nNo species yet"),
        TextFont {
            font_size: screen_layout.text_font_size(crate::ui::FontSizeClass::Medium),
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Right),
        SpeciesDisplayText,
    )).id()
}
