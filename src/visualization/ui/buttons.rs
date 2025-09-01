use bevy::prelude::*;
use crate::gameplay::cards::{PlayCardEvent, DiscardCardEvent};
use crate::gameplay::lifecycle::SimulateDayEvent;
use crate::visualization::ScreenLayout;
use crate::visualization::display::responsive_size_utils::{FontSizeClass, ResponsiveExt};

/// Component marker for action buttons
#[derive(Component)]
pub struct ActionButton {
    pub action: ButtonAction,
    pub enabled: bool,
}

/// Types of button actions
#[derive(Clone, Debug, PartialEq)]
pub enum ButtonAction {
    Play,
    Discard, 
    Pass,
}

/// Component marker for button sprites
#[derive(Component)]
pub struct ButtonSprite;

/// Component marker for button text
#[derive(Component)]
pub struct ButtonText;

/// Resource to track selected card
#[derive(Resource, Default)]
pub struct SelectedCard {
    pub index: Option<usize>,
}

impl SelectedCard {
    pub fn select(&mut self, index: usize) {
        self.index = Some(index);
    }
    
    pub fn clear(&mut self) {
        self.index = None;
    }
    
    pub fn has_selection(&self) -> bool {
        self.index.is_some()
    }
    
    pub fn get_selected(&self) -> Option<usize> {
        self.index
    }
}

/// Initialize action buttons below the hand
pub fn init_action_buttons(commands: &mut Commands, screen_layout: &ScreenLayout) {
    let button_width = screen_layout.w(20.0);
    let button_height = screen_layout.h(8.0);
    let button_spacing = screen_layout.w(5.0);
    
    // Calculate positions for three buttons centered at the bottom
    let total_width = (button_width * 3.0) + (button_spacing * 2.0);
    let start_x = -total_width / 2.0 + button_width / 2.0;
    let button_y = screen_layout.button_area_y; // Use dedicated button area
    
    let buttons = [
        (ButtonAction::Discard, "Discard", Color::srgb(0.8, 0.3, 0.3)), // Red
        (ButtonAction::Pass, "Pass", Color::srgb(0.3, 0.3, 0.8)),       // Blue
        (ButtonAction::Play, "Play", Color::srgb(0.3, 0.8, 0.3)),       // Green
    ];
    
    for (index, (action, text, color)) in buttons.iter().enumerate() {
        let x_position = start_x + (index as f32 * (button_width + button_spacing));
        
        // Initially only Pass button is enabled
        let enabled = matches!(action, ButtonAction::Pass);
        let button_color = if enabled { *color } else { Color::srgb(0.5, 0.5, 0.5) };
        
        // Spawn button background
        let button_entity = commands.spawn((
            Sprite {
                color: button_color,
                custom_size: Some(Vec2::new(button_width, button_height)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x_position, button_y, 1.0)),
            ActionButton {
                action: action.clone(),
                enabled,
            },
            ButtonSprite,
        )).id();
        
        // Spawn button text as child
        let text_entity = commands.spawn((
            Text2d::new(*text),
            TextFont {
                font_size: screen_layout.text_font_size(FontSizeClass::Medium),
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ButtonText,
        )).id();
        
        commands.entity(button_entity).add_child(text_entity);
    }
}

/// Update button visuals based on enabled state and selected card
pub fn update_button_visuals(
    selected_card: Res<SelectedCard>,
    game_state: Res<crate::gameplay::GameState>,
    mut button_query: Query<(&mut ActionButton, &mut Sprite), With<ButtonSprite>>,
) {
    if selected_card.is_changed() || game_state.is_changed() {
        for (mut button, mut sprite) in button_query.iter_mut() {
            let should_be_enabled = match button.action {
                ButtonAction::Pass => true, // Always enabled
                ButtonAction::Discard => selected_card.has_selection(),
                ButtonAction::Play => {
                    // In the new lifecycle system, we allow playing any card
                    // Resource constraints are handled during simulation
                    selected_card.has_selection()
                },
            };
            
            if button.enabled != should_be_enabled {
                button.enabled = should_be_enabled;
                
                sprite.color = if should_be_enabled {
                    match button.action {
                        ButtonAction::Discard => Color::srgb(0.8, 0.3, 0.3), // Red
                        ButtonAction::Pass => Color::srgb(0.3, 0.3, 0.8),    // Blue
                        ButtonAction::Play => Color::srgb(0.3, 0.8, 0.3),    // Green
                    }
                } else {
                    Color::srgb(0.5, 0.5, 0.5) // Gray when disabled
                };
            }
        }
    }
}

/// Handle button clicks
pub fn handle_button_clicks(
    button_query: Query<(&ActionButton, &Transform), With<ButtonSprite>>,
    screen_layout: Res<ScreenLayout>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    selected_card: Res<SelectedCard>,
    mut play_events: EventWriter<PlayCardEvent>,
    mut discard_events: EventWriter<DiscardCardEvent>,
    mut simulate_day_events: EventWriter<SimulateDayEvent>,
) {
    // Get required components
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = camera_query.single() else { return };
    
    // Check for user interaction (mouse or touch)
    let interaction_pos = get_interaction_position(&mouse_input, &touches, &window);
    
    if let Some(screen_pos) = interaction_pos {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            handle_button_interaction(
                world_pos,
                &button_query,
                &screen_layout,
                &selected_card,
                &mut play_events,
                &mut discard_events,
                &mut simulate_day_events,
            );
        }
    }
}

/// Update button layout when screen size changes
pub fn update_button_layout(
    mut button_query: Query<(&mut Transform, &mut Sprite, &ActionButton), With<ButtonSprite>>,
    mut text_query: Query<(&mut Transform, &mut TextFont), (With<ButtonText>, Without<ButtonSprite>)>,
    screen_layout: Res<ScreenLayout>,
) {
    if screen_layout.is_changed() {
        let button_width = screen_layout.w(20.0);
        let button_height = screen_layout.h(8.0);
        let button_spacing = screen_layout.w(5.0);
        
        let total_width = (button_width * 3.0) + (button_spacing * 2.0);
        let start_x = -total_width / 2.0 + button_width / 2.0;
        let button_y = screen_layout.button_area_y; // Use dedicated button area
        
        let mut button_index = 0;
        for (mut transform, mut sprite, _button) in button_query.iter_mut() {
            let x_position = start_x + (button_index as f32 * (button_width + button_spacing));
            transform.translation = Vec3::new(x_position, button_y, 1.0);
            sprite.custom_size = Some(Vec2::new(button_width, button_height));
            
            button_index += 1;
        }
        
        // Update text font sizes
        let text_size = screen_layout.text_font_size(FontSizeClass::Medium);
        for (_, mut text_font) in text_query.iter_mut() {
            text_font.font_size = text_size;
        }
    }
}

/// Gets the position of user interaction (mouse click or touch)
fn get_interaction_position(
    mouse_input: &ButtonInput<MouseButton>,
    touches: &Touches,
    window: &Window,
) -> Option<Vec2> {
    // Check for mouse click
    if mouse_input.just_pressed(MouseButton::Left) {
        return window.cursor_position();
    }
    
    // Check for touch input (handle first touch only)
    touches.iter_just_pressed().next().map(|touch| touch.position())
}

/// Handles interaction with buttons at the given world position
fn handle_button_interaction(
    world_pos: Vec2,
    button_query: &Query<(&ActionButton, &Transform), With<ButtonSprite>>,
    screen_layout: &ScreenLayout,
    selected_card: &SelectedCard,
    play_events: &mut EventWriter<PlayCardEvent>,
    discard_events: &mut EventWriter<DiscardCardEvent>,
    simulate_day_events: &mut EventWriter<SimulateDayEvent>,
) {
    let button_width = screen_layout.w(20.0);
    let button_height = screen_layout.h(8.0);
    
    for (button, transform) in button_query.iter() {
        if !button.enabled {
            continue;
        }
        
        let button_center = transform.translation.truncate();
        let half_size = Vec2::new(button_width, button_height) / 2.0;
        
        if world_pos.x >= button_center.x - half_size.x &&
           world_pos.x <= button_center.x + half_size.x &&
           world_pos.y >= button_center.y - half_size.y &&
           world_pos.y <= button_center.y + half_size.y {
            
            match button.action {
                ButtonAction::Play => {
                    if let Some(index) = selected_card.get_selected() {
                        play_events.write(PlayCardEvent { hand_index: index });
                    }
                }
                ButtonAction::Discard => {
                    if let Some(index) = selected_card.get_selected() {
                        discard_events.write(DiscardCardEvent { hand_index: index });
                    }
                }
                ButtonAction::Pass => {
                    simulate_day_events.write(SimulateDayEvent);
                }
            }
            break;
        }
    }
}

/// Clear selected card after actions are performed
pub fn clear_selection_after_actions(
    mut selected_card: ResMut<SelectedCard>,
    play_events: EventReader<PlayCardEvent>,
    discard_events: EventReader<DiscardCardEvent>,
    simulate_day_events: EventReader<SimulateDayEvent>,
) {
    if !play_events.is_empty() || !discard_events.is_empty() || !simulate_day_events.is_empty() {
        selected_card.clear();
    }
}
