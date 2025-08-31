use bevy::prelude::*;
use super::resources::{GardenResources, ResourceType};
use crate::gameplay::species::{SpeciesInstance, Species, Kingdom};
use std::collections::HashMap;

// ===== COMPONENTS AND RESOURCES =====

/// Core garden state with resources and species
#[derive(Resource)]
pub struct Garden {
    pub resources: GardenResources,
    pub species: Vec<SpeciesInstance>,
}

impl Default for Garden {
    fn default() -> Self {
        Self {
            resources: GardenResources::default(),
            species: Vec::new(),
        }
    }
}

impl Garden {
    pub fn get_resource(&self, resource_type: ResourceType) -> i32 {
        self.resources.get_resource(resource_type)
    }
    
    pub fn modify_resource(&mut self, resource_type: ResourceType, change: i32) {
        self.resources.modify_resource(resource_type, change);
    }

    pub fn can_afford(&self, requirements: &HashMap<ResourceType, i32>) -> bool {
        self.resources.can_afford(requirements)
    }

    /// Check if the garden can afford to add a new species
    pub fn can_afford_species(&self, species: &Species) -> bool {
        self.resources.can_afford(&species.daily_consumption)
    }

    /// Calculate total resource consumption for all species
    pub fn total_consumption(&self) -> HashMap<ResourceType, i32> {
        let mut total = HashMap::new();
        
        for species in &self.species {
            let species_consumption = species.total_daily_consumption();
            for (resource_type, amount) in species_consumption {
                *total.entry(resource_type).or_insert(0) += amount;
            }
        }
        
        total
    }

    /// Calculate total resource production for all species
    pub fn total_production(&self) -> HashMap<ResourceType, i32> {
        let mut total = HashMap::new();
        
        for species in &self.species {
            let species_production = species.total_daily_production();
            for (resource_type, amount) in species_production {
                *total.entry(resource_type).or_insert(0) += amount;
            }
        }
        
        total
    }

    /// Calculate net resource changes (production - consumption)
    pub fn net_resource_changes(&self) -> HashMap<ResourceType, i32> {
        let mut net_changes = HashMap::new();
        
        // Add production (positive)
        let production = self.total_production();
        for (resource_type, amount) in production {
            *net_changes.entry(resource_type).or_insert(0) += amount;
        }
        
        // Subtract consumption (negative)
        let consumption = self.total_consumption();
        for (resource_type, amount) in consumption {
            *net_changes.entry(resource_type).or_insert(0) -= amount;
        }
        
        net_changes
    }

    /// Remove species that cannot survive in the current environment
    pub fn remove_dead_species(&mut self) -> Vec<SpeciesInstance> {
        let mut removed_species = Vec::new();
        
        self.species.retain(|species| {
            if species.can_survive(&self.resources) {
                true
            } else {
                removed_species.push(species.clone());
                false
            }
        });
        
        removed_species
    }

    /// Get total population by kingdom
    pub fn population_by_kingdom(&self) -> (u32, u32, u32) {
        let mut plant_pop = 0;
        let mut animal_pop = 0;
        let mut fungi_pop = 0;

        for species in &self.species {
            match species.kingdom() {
                Kingdom::Plant => plant_pop += species.population,
                Kingdom::Animal => animal_pop += species.population,
                Kingdom::Fungi => fungi_pop += species.population,
            }
        }

        (plant_pop, animal_pop, fungi_pop)
    }

    /// Get the total count of all species
    pub fn total_species_count(&self) -> usize {
        self.species.len()
    }

    /// Check if there are any species in the garden
    pub fn is_empty(&self) -> bool {
        self.species.is_empty()
    }

    /// Log resource consumption for all species
    pub fn log_species_consumption(&self) {
        for species in &self.species {
            let consumption = species.total_daily_consumption();
            for (resource_type, amount) in consumption {
                if amount > 0 {
                    println!("Species {} consumed {} {}", 
                        species.name(), 
                        amount, 
                        resource_type.name()
                    );
                }
            }
        }
    }

    /// Log resource production for all species
    pub fn log_species_production(&self) {
        for species in &self.species {
            let production = species.total_daily_production();
            for (resource_type, amount) in production {
                if amount > 0 {
                    println!("Species {} produced {} {}", 
                        species.name(), 
                        amount, 
                        resource_type.name()
                    );
                }
            }
        }
    }

    /// Log which species died and why
    pub fn log_species_deaths(&self, dead_species: &[SpeciesInstance]) {
        for species in dead_species {
            let requirements = species.survival_requirements();
            
            for (resource_type, (min, max)) in requirements {
                let current_level = self.resources.get_resource(*resource_type);
                if current_level < *min || current_level > *max {
                    println!("Species {} died - {} level {} outside survival range [{}, {}]", 
                        species.name(), 
                        resource_type.name(), 
                        current_level, 
                        min, 
                        max
                    );
                    break; // Only log the first reason found
                }
            }
        }
    }

    pub fn add_species(&mut self, species: Species) -> bool {
        // Check if garden can afford this species
        if !self.can_afford_species(&species) {
            println!("Insufficient resources to add species: {}", species.name);
            return false;
        }

        // Add the species
        self.species.push(SpeciesInstance::new(species.clone(), 1));

        println!("Added species: {} to garden", species.name);
        true
    }

    /// Update population counts in resources based on current species
    fn update_population_counts(&mut self) {
        let (plant_pop, animal_pop, fungi_pop) = self.population_by_kingdom();
        self.resources.resources.insert(ResourceType::PlantPopulation, plant_pop as i32);
        self.resources.resources.insert(ResourceType::AnimalPopulation, animal_pop as i32);
        self.resources.resources.insert(ResourceType::FungiPopulation, fungi_pop as i32);
    }

    /// Run a complete daily simulation cycle
    pub fn run_daily_simulation(&mut self) {
        println!("=== Running Daily Garden Simulation ===");
        
        // Step 1: Calculate and apply resource changes
        let resource_changes = self.net_resource_changes();
        
        // Log what happened
        self.log_species_consumption();
        self.log_species_production();
        
        // Apply all resource changes
        self.resources.apply_resource_changes(resource_changes);
        
        // Step 2: Check survival requirements and remove dead species
        let dead_species = self.remove_dead_species();
        
        // Log deaths
        self.log_species_deaths(&dead_species);
        
        // Step 3: Update population counts
        self.update_population_counts();
        
        // Print final state
        self.resources.print_resources();
        println!("Active species: {}", self.total_species_count());
    }
}

// ===== EVENTS =====

/// Event for when a species is added to the garden
#[derive(Event)]
pub struct AddSpeciesToGardenEvent {
    pub species: Species,
}

/// Event for when species die during simulation
#[derive(Event)]
pub struct SpeciesDeathEvent {
    pub dead_species: Vec<SpeciesInstance>,
    pub reason: String,
}

/// Event to trigger a simulation update (when species are added)
#[derive(Event)]
pub struct SimulateDayEvent;

// ===== SYSTEMS =====

pub fn handle_add_species_to_garden_event(
    mut garden_state: ResMut<Garden>,
    mut add_species_events: EventReader<AddSpeciesToGardenEvent>,
    mut trigger_events: EventWriter<SimulateDayEvent>,
) {    
    for event in add_species_events.read() {
        if garden_state.add_species(event.species.clone()) {
            trigger_events.write(SimulateDayEvent);
        }
    }
}

/// System to trigger simulation when a species is added
/// This runs first and sends a single trigger event per species addition
pub fn trigger_simulation_on_species_play(
    mut trigger_events: EventWriter<SimulateDayEvent>,
    mut add_species_events: EventReader<AddSpeciesToGardenEvent>,
) {
    // Only trigger simulation once even if multiple species are added in the same frame
    let mut should_trigger = false;
    for _ in add_species_events.read() {
        should_trigger = true;
    }
    
    if should_trigger {
        trigger_events.write(SimulateDayEvent);
        println!("=== Starting Day Simulation ===");
    }
}

/// System that runs the complete daily simulation cycle
pub fn run_daily_simulation(
    mut garden_state: ResMut<Garden>,
    mut trigger_events: EventReader<SimulateDayEvent>,
) {
    // Only run if there's a trigger event
    if trigger_events.read().next().is_none() {
        return;
    }

    garden_state.run_daily_simulation();
}
