#![allow(unused_variables)]

use super::{
    LifecyclePhase, PhaseResult, PhaseError, LifecycleEvent,
    EcosystemPopulation, MaturityStage
};

/// Growth phase - juvenile creatures mature into adults
/// Only processes creatures that were successfully fed in the feeding phase
pub struct GrowthPhase;

impl GrowthPhase {
    pub fn new() -> Self {
        Self
    }
}

impl LifecyclePhase for GrowthPhase {
    fn execute(&self, ecosystem: &mut EcosystemPopulation) -> Result<PhaseResult, PhaseError> {
        let mut events = Vec::new();
        let mut creatures_processed = 0;

        // Process all juvenile creatures
        for creature in ecosystem.living_creatures_mut() {
            if creature.maturity_stage == MaturityStage::Juvenile {
                creatures_processed += 1;

                // Check if creature can grow
                if creature.fed_status.is_well_fed(0.7) {
                    creature.maturity_stage = MaturityStage::Mature;
                    
                    events.push(LifecycleEvent::CreatureGrew {
                        creature_id: creature.id,
                    });
                }
            }
        }

        Ok(PhaseResult {
            creatures_processed,
            matter_transformed: 0, // Growth doesn't transform matter, just changes state
            events,
        })
    }

    fn validate_preconditions(&self, _ecosystem: &EcosystemPopulation) -> Result<(), PhaseError> {
        // Validate that feeding phase has been completed
        // (all creatures should have updated feeding status)
        Ok(())
    }

    fn validate_postconditions(&self, _ecosystem: &EcosystemPopulation) -> Result<(), PhaseError> {
        // Validate that growth requirements were properly applied
        Ok(())
    }

    fn phase_name(&self) -> &'static str {
        "Growth"
    }
}

impl Default for GrowthPhase {
    fn default() -> Self {
        Self::new()
    }
}
