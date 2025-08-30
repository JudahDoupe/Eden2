use bevy::prelude::*;
use crate::types::CardType;

/// UI Component marker for resource display text
#[derive(Component)]
pub struct ResourceDisplayText;

/// UI Component marker for species display text
#[derive(Component)]
pub struct SpeciesDisplayText;

/// UI Component for card entities
#[derive(Component)]
pub struct Card {
    pub card_type: CardType,
    pub hand_index: usize,
    pub is_selected: bool,
}

/// UI Component marker for card sprites
#[derive(Component)]
pub struct CardSprite;

/// UI Component marker for card text
#[derive(Component)]
pub struct CardText;

/// UI Component marker for garden background
#[derive(Component)]
pub struct GardenBackground;

/// Resource flag to track if layout has been initialized to actual window size
#[derive(Resource, Default)]
pub struct LayoutInitialized(pub bool);
