#![allow(unused_variables)]

use super::{
    LifecyclePhase, PhaseResult, PhaseError, LifecycleEvent,
    EcosystemPopulation, DeathCause, MaturityStage
};

/// Death phase - removes creatures that have died and converts their biomass
/// Processes all death conditions before any other lifecycle phases
pub struct DeathPhase;

impl DeathPhase {
    pub fn new() -> Self {
        Self
    }

    /// Check if a creature should die from various causes
    fn check_death_conditions(&self, creature: &crate::gameplay::lifecycle::IndividualCreature) -> Option<DeathCause> {
        // Already dead
        if !creature.is_alive() {
            return None;
        }

        // TODO: Implement species-specific mortality factors
        // For now, use simple rules:

        // Starvation death
        if creature.days_since_last_fed > 3 {
            return Some(DeathCause::Starvation);
        }

        // Natural aging (simplified)
        let max_age = match creature.species.kingdom {
            crate::gameplay::species::Kingdom::Plant => 100,    // Plants live longer
            crate::gameplay::species::Kingdom::Animal => 50,    // Animals have shorter lives
            crate::gameplay::species::Kingdom::Fungi => 75,     // Fungi intermediate
        };

        if creature.age_days > max_age {
            return Some(DeathCause::NaturalAge);
        }

        // Environmental stress (simplified)
        if creature.days_in_bad_environment > 10 {
            return Some(DeathCause::EnvironmentalStress);
        }

        None
    }
}

impl LifecyclePhase for DeathPhase {
    fn execute(&self, ecosystem: &mut EcosystemPopulation) -> Result<PhaseResult, PhaseError> {
        let mut events = Vec::new();
        let mut creatures_processed = 0;
        let mut matter_transformed = 0;

        // Check death conditions for all living creatures
        let mut creatures_to_kill = Vec::new();
        
        for creature in ecosystem.living_creatures() {
            creatures_processed += 1;

            if let Some(death_cause) = self.check_death_conditions(creature) {
                creatures_to_kill.push((creature.id, death_cause));
            }
        }

        // Apply death to marked creatures
        for (creature_id, death_cause) in creatures_to_kill {
            // Mark creature as dead
            if let Some(creature) = ecosystem.creatures.iter_mut().find(|c| c.id == creature_id) {
                let biomass_amount = creature.biomass.total();
                creature.die(death_cause.clone());
                matter_transformed += biomass_amount;

                events.push(LifecycleEvent::CreatureDied {
                    creature_id,
                    cause: death_cause,
                });
            }
        }

        // Remove dead creatures and convert their biomass to dead matter
        let removed_creatures = ecosystem.remove_dead_creatures();
        
        // Log matter transformations
        for (creature, _cause) in &removed_creatures {
            let (dead_matter_type, amount) = creature.get_death_matter();
            if amount > 0 {
                events.push(LifecycleEvent::MatterTransformed {
                    from_type: match creature.species.kingdom {
                        crate::gameplay::species::Kingdom::Plant | 
                        crate::gameplay::species::Kingdom::Fungi => super::MatterType::PlantMatter,
                        crate::gameplay::species::Kingdom::Animal => super::MatterType::AnimalMatter,
                    },
                    to_type: dead_matter_type,
                    amount,
                });
            }
        }

        Ok(PhaseResult {
            creatures_processed,
            matter_transformed,
            events,
        })
    }

    fn validate_preconditions(&self, _ecosystem: &EcosystemPopulation) -> Result<(), PhaseError> {
        // Death phase runs first, so no specific preconditions
        Ok(())
    }

    fn validate_postconditions(&self, ecosystem: &EcosystemPopulation) -> Result<(), PhaseError> {
        // Validate that no dead creatures remain in the population
        for creature in &ecosystem.creatures {
            if creature.maturity_stage == MaturityStage::Dead {
                return Err(PhaseError::SystemError(
                    "Dead creature found in population after death phase".to_string()
                ));
            }
        }

        Ok(())
    }

    fn phase_name(&self) -> &'static str {
        "Death"
    }
}

impl Default for DeathPhase {
    fn default() -> Self {
        Self::new()
    }
}
