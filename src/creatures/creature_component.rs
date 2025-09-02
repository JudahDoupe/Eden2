use bevy::prelude::*;
use crate::gameplay::lifecycle::{IndividualCreature, CreatureId};

/// Component wrapper for IndividualCreature
/// This allows us to link the gameplay model with the ECS visualization system
#[derive(Component)]
pub struct CreatureComponent {
    /// Reference to the gameplay creature ID
    pub creature_id: CreatureId,
    /// The species ID for visual representation
    pub species_id: String,
    /// Current lifecycle stage
    pub lifecycle_stage: String,
}

/// Handle to mark components as SVG-based for rendering
#[derive(Component, Clone)]
pub struct SvgHandle(pub Handle<Image>);

/// Component to add to SvgSprite entities
#[derive(Component, Reflect)]
pub struct SvgComponent;
