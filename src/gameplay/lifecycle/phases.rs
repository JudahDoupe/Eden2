use bevy::prelude::*;
use super::{EcosystemPopulation, MatterConservationError};

/// Trait for individual lifecycle phases
/// Each phase processes all relevant creatures before moving to the next phase
pub trait LifecyclePhase {
    /// Execute this phase on the ecosystem
    fn execute(&self, ecosystem: &mut EcosystemPopulation) -> Result<PhaseResult, PhaseError>;
    
    /// Validate that the ecosystem is in a valid state for this phase
    fn validate_preconditions(&self, ecosystem: &EcosystemPopulation) -> Result<(), PhaseError>;
    
    /// Validate that the phase completed successfully
    fn validate_postconditions(&self, ecosystem: &EcosystemPopulation) -> Result<(), PhaseError>;
    
    /// Get the name of this phase for logging/debugging
    fn phase_name(&self) -> &'static str;
}

/// Result of executing a lifecycle phase
#[derive(Debug)]
pub struct PhaseResult {
    pub creatures_processed: u32,
    pub matter_transformed: u32,
    pub events: Vec<LifecycleEvent>,
}

/// Events that can occur during lifecycle phases
#[derive(Debug, Clone)]
pub enum LifecycleEvent {
    CreatureFed { creature_id: super::CreatureId, satisfaction: f32 },
    CreatureGrew { creature_id: super::CreatureId },
    CreatureReproduced { parent_id: super::CreatureId, offspring_id: super::CreatureId },
    CreatureDied { creature_id: super::CreatureId, cause: super::DeathCause },
    MatterTransformed { from_type: super::MatterType, to_type: super::MatterType, amount: u32 },
}

/// Errors that can occur during lifecycle phases
#[derive(Debug)]
pub enum PhaseError {
    MatterConservationViolation(MatterConservationError),
    InvalidCreatureState(String),
    InsufficientResources(String),
    SystemError(String),
}

impl From<MatterConservationError> for PhaseError {
    fn from(error: MatterConservationError) -> Self {
        PhaseError::MatterConservationViolation(error)
    }
}

impl std::fmt::Display for PhaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhaseError::MatterConservationViolation(e) => write!(f, "Matter conservation error: {}", e),
            PhaseError::InvalidCreatureState(msg) => write!(f, "Invalid creature state: {}", msg),
            PhaseError::InsufficientResources(msg) => write!(f, "Insufficient resources: {}", msg),
            PhaseError::SystemError(msg) => write!(f, "System error: {}", msg),
        }
    }
}

impl std::error::Error for PhaseError {}

/// Orchestrates the complete daily simulation cycle
/// Ensures phases are executed in the correct order with proper validation
pub struct DailySimulation {
    pub phases: Vec<Box<dyn LifecyclePhase>>,
}

impl DailySimulation {
    /// Create a new daily simulation with default phase order
    pub fn new() -> Self {
        Self {
            phases: vec![
                Box::new(super::death::DeathPhase::new()),
                Box::new(super::feeding::FeedingPhaseImpl::new()),
                Box::new(super::growth::GrowthPhase::new()),
                Box::new(super::reproduction::ReproductionPhase::new()),
            ],
        }
    }

    /// Execute one complete daily cycle
    pub fn simulate_day(&self, ecosystem: &mut EcosystemPopulation) -> Result<DailyResult, PhaseError> {
        let mut total_events = Vec::new();
        let mut phase_results = Vec::new();

        println!("Starting daily simulation for day {}", ecosystem.current_day + 1);

        // Execute each phase in order
        for phase in &self.phases {
            // Validate preconditions
            phase.validate_preconditions(ecosystem)?;

            // Execute phase
            let result = phase.execute(ecosystem)?;
            println!("Completed {} phase: {} creatures processed", 
                  phase.phase_name(), result.creatures_processed);

            // Validate postconditions
            phase.validate_postconditions(ecosystem)?;

            total_events.extend(result.events.clone());
            phase_results.push((phase.phase_name().to_string(), result));
        }

        // Advance to next day
        ecosystem.advance_day();

        Ok(DailyResult {
            day: ecosystem.current_day - 1, // Previous day that was just simulated
            phase_results,
            total_events,
            final_population: ecosystem.total_population(),
        })
    }
}

impl Default for DailySimulation {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a complete daily simulation
#[derive(Debug)]
pub struct DailyResult {
    pub day: u32,
    pub phase_results: Vec<(String, PhaseResult)>,
    pub total_events: Vec<LifecycleEvent>,
    pub final_population: u32,
}

impl DailyResult {
    /// Get summary statistics for the day
    pub fn get_summary(&self) -> DailySummary {
        let mut births = 0;
        let mut deaths = 0;
        let mut growth_events = 0;
        let mut feeding_events = 0;

        for event in &self.total_events {
            match event {
                LifecycleEvent::CreatureReproduced { .. } => births += 1,
                LifecycleEvent::CreatureDied { .. } => deaths += 1,
                LifecycleEvent::CreatureGrew { .. } => growth_events += 1,
                LifecycleEvent::CreatureFed { .. } => feeding_events += 1,
                _ => {}
            }
        }

        DailySummary {
            day: self.day,
            births,
            deaths,
            growth_events,
            feeding_events,
            final_population: self.final_population,
        }
    }
}

/// Summary statistics for a day's simulation
#[derive(Debug)]
pub struct DailySummary {
    pub day: u32,
    pub births: u32,
    pub deaths: u32,
    pub growth_events: u32,
    pub feeding_events: u32,
    pub final_population: u32,
}
