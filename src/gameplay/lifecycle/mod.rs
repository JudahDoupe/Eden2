pub mod creature;
pub mod matter;
pub mod population;
pub mod phases;
pub mod feeding;
pub mod growth;
pub mod reproduction;
pub mod death;

// Re-export key types for easier access
pub use creature::{IndividualCreature, CreatureBiomass, MaturityStage, FeedingResult, CreatureId, DeathCause};
pub use matter::{EcosystemMatter, MatterType, MatterConservationError};
pub use population::{EcosystemPopulation, handle_add_species_to_ecosystem_event, handle_simulate_day_event};
pub use phases::{LifecyclePhase, DailySimulation, PhaseResult, PhaseError, LifecycleEvent};
pub use feeding::FeedingPhaseImpl;

// Events for game integration
use bevy::prelude::*;

#[derive(Event)]
pub struct SimulateDayEvent;

#[derive(Event)]
pub struct AddSpeciesToEcosystemEvent {
    pub species: crate::gameplay::species::Species,
    pub starting_biomass: (u32, u32), // (plant_matter, animal_matter)
}
