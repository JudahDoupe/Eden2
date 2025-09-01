#![allow(unused_variables)]

use super::{
    LifecyclePhase, PhaseResult, PhaseError, LifecycleEvent,
    EcosystemPopulation, MaturityStage, CreatureBiomass
};

/// Reproduction phase - mature, well-fed creatures create offspring
/// Parent biomass is converted to offspring biomass (matter conservation)
pub struct ReproductionPhase;

impl ReproductionPhase {
    pub fn new() -> Self {
        Self
    }

    /// Check if a creature can reproduce based on basic requirements
    fn can_reproduce(&self, creature: &crate::gameplay::lifecycle::IndividualCreature) -> bool {
        // Must be mature
        if creature.maturity_stage != MaturityStage::Mature {
            return false;
        }

        // Must be well-fed
        if !creature.fed_status.is_well_fed(0.8) {
            return false;
        }

        // Must have sufficient biomass
        if creature.biomass.total() < 2 {
            return false;
        }

        // TODO: Check species-specific requirements
        // - Reproduction cooldown
        // - Environmental requirements  
        // - Population requirements (pairs, max density, etc.)

        true
    }

    /// Calculate offspring biomass from parent
    fn calculate_offspring_biomass(&self, parent: &crate::gameplay::lifecycle::IndividualCreature) -> (CreatureBiomass, CreatureBiomass) {
        // For now, simple 50/50 split
        // TODO: Implement species-specific reproduction costs and offspring sizes
        
        let total_plant = parent.biomass.plant_matter;
        let total_animal = parent.biomass.animal_matter;
        
        let offspring_plant = total_plant / 2;
        let offspring_animal = total_animal / 2;
        
        let remaining_plant = total_plant - offspring_plant;
        let remaining_animal = total_animal - offspring_animal;
        
        let offspring_biomass = CreatureBiomass::new(offspring_plant, offspring_animal);
        let parent_remaining = CreatureBiomass::new(remaining_plant, remaining_animal);
        
        (parent_remaining, offspring_biomass)
    }
}

impl LifecyclePhase for ReproductionPhase {
    fn execute(&self, ecosystem: &mut EcosystemPopulation) -> Result<PhaseResult, PhaseError> {
        let mut events = Vec::new();
        let mut creatures_processed = 0;
        let mut new_offspring = Vec::new();

        // Collect reproduction candidates to avoid borrowing issues
        let mut reproduction_candidates = Vec::new();
        for creature in ecosystem.living_creatures() {
            if self.can_reproduce(creature) {
                reproduction_candidates.push(creature.id);
            }
        }

        // Process each reproduction candidate
        for parent_id in reproduction_candidates {
            creatures_processed += 1;

            // Find parent creature
            let (parent_biomass, offspring_biomass) = {
                let parent = ecosystem.creatures.iter()
                    .find(|c| c.id == parent_id && c.is_alive())
                    .ok_or_else(|| PhaseError::SystemError("Parent creature not found".to_string()))?;
                
                self.calculate_offspring_biomass(parent)
            };

            // Update parent biomass
            if let Some(parent) = ecosystem.creatures.iter_mut().find(|c| c.id == parent_id) {
                parent.biomass = parent_biomass;
                parent.last_reproduction_day = Some(ecosystem.current_day);
            }

            // Create offspring (but don't add to ecosystem yet to avoid borrowing issues)
            let parent_species = ecosystem.creatures.iter()
                .find(|c| c.id == parent_id)
                .map(|c| c.species.clone())
                .ok_or_else(|| PhaseError::SystemError("Parent species not found".to_string()))?;

            new_offspring.push((parent_species, offspring_biomass, parent_id));
        }

        // Add all offspring to ecosystem
        for (species, biomass, parent_id) in new_offspring {
            let offspring_id = ecosystem.add_creature(species, (biomass.plant_matter, biomass.animal_matter));
            
            events.push(LifecycleEvent::CreatureReproduced {
                parent_id,
                offspring_id,
            });
        }

        Ok(PhaseResult {
            creatures_processed,
            matter_transformed: 0, // Matter is conserved in reproduction
            events,
        })
    }

    fn validate_preconditions(&self, _ecosystem: &EcosystemPopulation) -> Result<(), PhaseError> {
        // Validate that growth phase has been completed
        Ok(())
    }

    fn validate_postconditions(&self, _ecosystem: &EcosystemPopulation) -> Result<(), PhaseError> {
        // Validate that matter was conserved during reproduction
        // Validate that new offspring have proper biomass
        Ok(())
    }

    fn phase_name(&self) -> &'static str {
        "Reproduction"
    }
}

impl Default for ReproductionPhase {
    fn default() -> Self {
        Self::new()
    }
}
