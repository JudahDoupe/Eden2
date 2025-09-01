use bevy::prelude::*;
use super::{
    LifecyclePhase, PhaseResult, PhaseError, LifecycleEvent,
    EcosystemPopulation, MatterType, FeedingResult, CreatureId
};
use crate::gameplay::species::BiomassConversion;
use std::collections::HashMap;

/// Represents a creature's feeding demand
#[derive(Debug)]
struct FeedingDemand {
    creature_id: CreatureId,
    matter_demands: HashMap<MatterType, u32>,
    introduction_order: u32,
}

/// Result of resource allocation
#[derive(Debug)]
struct FeedingAllocation {
    creature_id: CreatureId,
    allocated_resources: HashMap<MatterType, u32>,
    satisfaction_level: f32,
}

/// Feeding phase - creatures compete for resources and convert matter
/// Processes creatures in trophic order: Decomposers → Producers → Consumers
pub struct FeedingPhaseImpl;

impl FeedingPhaseImpl {
    pub fn new() -> Self {
        Self
    }

    /// Calculate feeding demand for a creature
    fn calculate_feeding_demand(&self, creature: &crate::gameplay::lifecycle::IndividualCreature) -> FeedingDemand {
        let mut matter_demands = HashMap::new();
        
        // Get base requirements from species
        for (matter_type, base_amount) in &creature.species.feeding_requirements.base_requirements {
            let mut required_amount = *base_amount;
            
            // Apply maturity multiplier
            if creature.is_mature() {
                required_amount = (required_amount as f32 * creature.species.feeding_requirements.maturity_multiplier) as u32;
            }
            
            matter_demands.insert(*matter_type, required_amount);
        }

        FeedingDemand {
            creature_id: creature.id,
            matter_demands,
            introduction_order: creature.introduction_order,
        }
    }

    /// Allocate resources among competing creatures
    fn allocate_resources(
        &self,
        demands: Vec<FeedingDemand>,
        ecosystem_matter: &mut crate::gameplay::lifecycle::EcosystemMatter,
    ) -> Vec<FeedingAllocation> {
        let mut allocations = Vec::new();

        // Group demands by matter type to handle competition
        let mut matter_demands: HashMap<MatterType, Vec<(CreatureId, u32, u32)>> = HashMap::new();
        
        for demand in &demands {
            for (matter_type, amount) in &demand.matter_demands {
                matter_demands.entry(*matter_type)
                    .or_insert_with(Vec::new)
                    .push((demand.creature_id, *amount, demand.introduction_order));
            }
        }

        // Process each matter type allocation
        let mut creature_allocations: HashMap<CreatureId, HashMap<MatterType, u32>> = HashMap::new();

        for (matter_type, requesters) in matter_demands {
            let available_amount = ecosystem_matter.get_amount(matter_type);
            let total_demand: u32 = requesters.iter().map(|(_, amount, _)| amount).sum();

            if total_demand <= available_amount {
                // Enough for everyone
                for (creature_id, amount, _) in requesters {
                    creature_allocations.entry(creature_id)
                        .or_insert_with(HashMap::new)
                        .insert(matter_type, amount);
                    ecosystem_matter.consume_matter(matter_type, amount);
                }
            } else {
                // Not enough - allocate proportionally based on introduction order
                let mut sorted_requesters = requesters;
                sorted_requesters.sort_by_key(|(_, _, order)| *order);

                let mut remaining = available_amount;
                for (creature_id, requested, _) in sorted_requesters {
                    let allocated = requested.min(remaining);
                    if allocated > 0 {
                        creature_allocations.entry(creature_id)
                            .or_insert_with(HashMap::new)
                            .insert(matter_type, allocated);
                        ecosystem_matter.consume_matter(matter_type, allocated);
                        remaining -= allocated;
                    }
                }
            }
        }

        // Convert to allocations with satisfaction levels
        for demand in demands {
            let empty_map = HashMap::new();
            let allocated = creature_allocations.get(&demand.creature_id)
                .unwrap_or(&empty_map);
            
            let total_requested: u32 = demand.matter_demands.values().sum();
            let total_allocated: u32 = allocated.values().sum();
            
            let satisfaction_level = if total_requested > 0 {
                total_allocated as f32 / total_requested as f32
            } else {
                1.0
            };

            allocations.push(FeedingAllocation {
                creature_id: demand.creature_id,
                allocated_resources: allocated.clone(),
                satisfaction_level: satisfaction_level.min(1.0),
            });
        }

        allocations
    }

    /// Apply feeding results to creatures
    fn apply_feeding_results(
        &self,
        allocations: Vec<FeedingAllocation>,
        ecosystem: &mut EcosystemPopulation,
    ) -> Result<Vec<LifecycleEvent>, PhaseError> {
        let mut events = Vec::new();

        for allocation in allocations {
            if let Some(creature) = ecosystem.creatures.iter_mut().find(|c| c.id == allocation.creature_id) {
                // Determine feeding result based on satisfaction
                let feeding_result = if allocation.satisfaction_level >= 1.0 {
                    FeedingResult::FullyFed
                } else if allocation.satisfaction_level >= creature.species.feeding_requirements.minimum_threshold {
                    FeedingResult::PartiallyFed(allocation.satisfaction_level)
                } else {
                    FeedingResult::Starving
                };

                creature.set_fed_status(feeding_result.clone());

                // Convert consumed matter to creature biomass based on species conversion
                if allocation.satisfaction_level > 0.0 {
                    self.apply_biomass_conversion(creature, &allocation.allocated_resources, &mut events, &mut ecosystem.ecosystem_matter)?;
                }

                events.push(LifecycleEvent::CreatureFed {
                    creature_id: allocation.creature_id,
                    satisfaction: allocation.satisfaction_level,
                });
            }
        }

        Ok(events)
    }

    /// Convert consumed matter to creature biomass
    fn apply_biomass_conversion(
        &self,
        creature: &mut crate::gameplay::lifecycle::IndividualCreature,
        consumed_resources: &HashMap<MatterType, u32>,
        events: &mut Vec<LifecycleEvent>,
        ecosystem_matter: &mut crate::gameplay::lifecycle::EcosystemMatter,
    ) -> Result<(), PhaseError> {
        for (matter_type, amount) in consumed_resources {
            match &creature.species.feeding_requirements.biomass_conversion {
                BiomassConversion::PlantGrowth { efficiency } => {
                    let gained_biomass = (*amount as f32 * efficiency) as u32;
                    creature.biomass.add_matter(MatterType::PlantMatter, gained_biomass);
                    
                    if gained_biomass > 0 {
                        events.push(LifecycleEvent::MatterTransformed {
                            from_type: *matter_type,
                            to_type: MatterType::PlantMatter,
                            amount: gained_biomass,
                        });
                    }
                },
                BiomassConversion::PlantToAnimal { efficiency } => {
                    let gained_biomass = (*amount as f32 * efficiency) as u32;
                    creature.biomass.add_matter(MatterType::AnimalMatter, gained_biomass);
                    
                    if gained_biomass > 0 {
                        events.push(LifecycleEvent::MatterTransformed {
                            from_type: *matter_type,
                            to_type: MatterType::AnimalMatter,
                            amount: gained_biomass,
                        });
                    }
                },
                BiomassConversion::AnimalToAnimal { efficiency } => {
                    let gained_biomass = (*amount as f32 * efficiency) as u32;
                    creature.biomass.add_matter(MatterType::AnimalMatter, gained_biomass);
                    
                    if gained_biomass > 0 {
                        events.push(LifecycleEvent::MatterTransformed {
                            from_type: *matter_type,
                            to_type: MatterType::AnimalMatter,
                            amount: gained_biomass,
                        });
                    }
                },
                BiomassConversion::Decomposition { nutrient_output, biomass_gain, matter_type: biomass_type } => {
                    // Decomposers produce soil nutrients and gain biomass
                    let nutrients_produced = (*amount as f32 * nutrient_output) as u32;
                    let biomass_gained = (*amount as f32 * biomass_gain) as u32;
                    
                    // Add nutrients back to ecosystem
                    if nutrients_produced > 0 {
                        ecosystem_matter.add_matter(MatterType::SoilNutrients, nutrients_produced);
                    }
                    
                    if biomass_gained > 0 {
                        creature.biomass.add_matter(*biomass_type, biomass_gained);
                        
                        events.push(LifecycleEvent::MatterTransformed {
                            from_type: *matter_type,
                            to_type: *biomass_type,
                            amount: biomass_gained,
                        });
                    }
                },
            }
        }

        Ok(())
    }

    /// Process feeding for creatures of a specific trophic level
    fn process_trophic_level(
        &self,
        creature_ids: Vec<CreatureId>,
        ecosystem: &mut EcosystemPopulation,
    ) -> Result<Vec<LifecycleEvent>, PhaseError> {
        let mut events = Vec::new();

        // Calculate demands for all creatures in this trophic level
        let mut demands = Vec::new();
        for creature_id in &creature_ids {
            if let Some(creature) = ecosystem.creatures.iter().find(|c| c.id == *creature_id && c.is_alive()) {
                demands.push(self.calculate_feeding_demand(creature));
            }
        }

        if demands.is_empty() {
            return Ok(events);
        }

        // Allocate resources among competing creatures
        let allocations = self.allocate_resources(demands, &mut ecosystem.ecosystem_matter);

        // Apply feeding results
        let feeding_events = self.apply_feeding_results(allocations, ecosystem)?;
        events.extend(feeding_events);

        Ok(events)
    }
}

impl LifecyclePhase for FeedingPhaseImpl {
    fn execute(&self, ecosystem: &mut EcosystemPopulation) -> Result<PhaseResult, PhaseError> {
        let mut all_events = Vec::new();
        let mut creatures_processed = 0;
        let mut matter_transformed = 0;

        // Collect creatures grouped by trophic level to avoid borrowing issues
        let mut trophic_groups: Vec<(u8, Vec<CreatureId>)> = Vec::new();
        let mut current_trophic_level = None;
        let mut current_group = Vec::new();

        // Get creatures in trophic order
        for creature in ecosystem.creatures_by_trophic_order() {
            let trophic_level = match creature.species.kingdom {
                crate::gameplay::species::Kingdom::Fungi => 0,
                crate::gameplay::species::Kingdom::Animal => {
                    if creature.species.name.contains("Earthworm") || creature.species.name.contains("Worm") { 0 } else { 2 }
                },
                crate::gameplay::species::Kingdom::Plant => 1,
            };

            match current_trophic_level {
                None => {
                    current_trophic_level = Some(trophic_level);
                    current_group.push(creature.id);
                }
                Some(level) if level == trophic_level => {
                    current_group.push(creature.id);
                }
                Some(level) => {
                    // Finish current group and start new one
                    trophic_groups.push((level, current_group.clone()));
                    current_group.clear();
                    current_group.push(creature.id);
                    current_trophic_level = Some(trophic_level);
                }
            }
        }

        // Add final group
        if !current_group.is_empty() {
            if let Some(level) = current_trophic_level {
                trophic_groups.push((level, current_group));
            }
        }

        // Process each trophic group in order
        for (_level, creature_ids) in trophic_groups {
            let group_events = self.process_trophic_level(creature_ids.clone(), ecosystem)?;
            creatures_processed += creature_ids.len() as u32;
            
            // Count matter transformation events
            matter_transformed += group_events.iter()
                .filter(|e| matches!(e, LifecycleEvent::MatterTransformed { .. }))
                .count() as u32;
            
            all_events.extend(group_events);
        }

        println!("Info: Feeding phase completed: {} creatures processed, {} matter transformations", 
                 creatures_processed, matter_transformed);

        Ok(PhaseResult {
            creatures_processed,
            matter_transformed,
            events: all_events,
        })
    }

    fn validate_preconditions(&self, _ecosystem: &EcosystemPopulation) -> Result<(), PhaseError> {
        // Validate that death phase has been completed
        // (no creatures should be marked as dead but still in population)
        Ok(())
    }

    fn validate_postconditions(&self, _ecosystem: &EcosystemPopulation) -> Result<(), PhaseError> {
        // Validate matter conservation
        // All creatures should have updated feeding status
        Ok(())
    }

    fn phase_name(&self) -> &'static str {
        "Feeding"
    }
}

impl Default for FeedingPhaseImpl {
    fn default() -> Self {
        Self::new()
    }
}
