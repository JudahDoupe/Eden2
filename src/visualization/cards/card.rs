use bevy::prelude::*;
use crate::gameplay::cards::PlayCardEvent;
use crate::gameplay::Card;
use crate::gameplay::species::species::get_species;
use crate::visualization::ScreenLayout;

#[derive(Clone, Debug)]
pub struct CardDefinition {
    pub name: &'static str,
    pub color: Color,
}

/// Get card definition by name (for visualization)
/// This creates a card definition from the species definition
pub fn get_card_definition(name: &str) -> Option<CardDefinition> {
    get_species(name).map(|species_def| CardDefinition {
        name: species_def.name,
        color: species_def.color,
    })
}

/// UI Component for card entities
#[derive(Component)]
pub struct CardComponent {
    pub species: Card,
    pub hand_index: usize,
    pub is_selected: bool,
}

impl CardComponent {
    /// Get card definition for visualization (color, etc.)
    pub fn card_definition(&self) -> CardDefinition {
        get_card_definition(self.species.name()).expect("Species should have corresponding card definition")
    }
}

/// UI Component marker for card sprites
#[derive(Component)]
pub struct CardSprite;

/// UI Component marker for card text
#[derive(Component)]
pub struct CardText;

/// Update card visuals based on state
pub fn update_card_visuals(
    mut card_query: Query<(&CardComponent, &mut Sprite), With<CardSprite>>,
) {
    for (card, mut sprite) in card_query.iter_mut() {
        sprite.color = if card.is_selected {
            Color::srgb(1.0, 1.0, 0.8) // Light yellow when selected
        } else {
            card.card_definition().color
        };
    }
}

/// Handles clicking/touching cards to play them
pub fn handle_card_clicks(
    card_query: Query<(&CardComponent, &Transform), With<CardSprite>>,
    screen_layout: Res<ScreenLayout>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut species_play_events: EventWriter<PlayCardEvent>,
) {
    // Get required components
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = camera_query.single() else { return };
    
    // Check for user interaction (mouse or touch)
    let interaction_pos = get_interaction_position(&mouse_input, &touches, &window);
    
    if let Some(screen_pos) = interaction_pos {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            handle_card_interaction(world_pos, &card_query, &screen_layout, &mut species_play_events);
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

/// Handles interaction with cards at the given world position
fn handle_card_interaction(
    world_pos: Vec2,
    card_query: &Query<(&CardComponent, &Transform), With<CardSprite>>,
    screen_layout: &ScreenLayout,
    species_play_events: &mut EventWriter<PlayCardEvent>,
) {
    for (card, transform) in card_query.iter() {
        if is_point_in_card(world_pos, transform.translation.truncate(), screen_layout, card.hand_index) {
            species_play_events.write(PlayCardEvent {
                hand_index: card.hand_index,
            });
            break;
        }
    }
}

/// Check if a point is inside a card
fn is_point_in_card(point: Vec2, card_center: Vec2, screen_layout: &ScreenLayout, _hand_index: usize) -> bool {
    // We need to calculate the card size based on current hand size
    // This is a bit of a hack since we don't have access to GameState here
    // In a real implementation, you might want to store card size on the Card component
    let card_size = screen_layout.calculate_card_size(5); // Assume max hand size for now
    let half_size = card_size / 2.0;
    
    point.x >= card_center.x - half_size.x &&
    point.x <= card_center.x + half_size.x &&
    point.y >= card_center.y - half_size.y &&
    point.y <= card_center.y + half_size.y
}
