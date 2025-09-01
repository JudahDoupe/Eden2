use bevy::prelude::*;
use super::resources::{GardenResources, ResourceType};
use crate::gameplay::species::{Creature, Species, Kingdom};
use std::collections::HashMap;

// ===== COMPONENTS AND RESOURCES =====

/// Core garden state with resources and species
#[derive(Resource)]
pub struct Garden {
    pub resources: GardenResources,
    pub creatures: Vec<Creature>,
}

impl Default for Garden {
    fn default() -> Self {
        Self {
            resources: GardenResources::default(),
            creatures: Vec::new(),
        }
    }
}

impl Garden {
    pub fn total_consumption(&self) -> HashMap<ResourceType, i32> {
        let mut total = HashMap::new();
        
        for species in &self.creatures {
            let species_consumption = species.total_daily_consumption();
            for (resource_type, amount) in species_consumption {
                *total.entry(resource_type).or_insert(0) += amount;
            }
        }
        
        total
    }

    pub fn total_production(&self) -> HashMap<ResourceType, i32> {
        let mut total = HashMap::new();
        
        for species in &self.creatures {
            let species_production = species.total_daily_production();
            for (resource_type, amount) in species_production {
                *total.entry(resource_type).or_insert(0) += amount;
            }
        }
        
        total
    }

    pub fn net_resource_changes(&self) -> HashMap<ResourceType, i32> {
        let mut net_changes = HashMap::new();
        
        let production = self.total_production();
        for (resource_type, amount) in production {
            *net_changes.entry(resource_type).or_insert(0) += amount;
        }
        
        let consumption = self.total_consumption();
        for (resource_type, amount) in consumption {
            *net_changes.entry(resource_type).or_insert(0) -= amount;
        }
        
        net_changes
    }

    pub fn remove_dead_species(&mut self) -> Vec<Creature> {
        let mut removed_species = Vec::new();
        
        self.creatures.retain(|species| {
            if species.can_survive(&self.resources) {
                true
            } else {
                removed_species.push(species.clone());
                false
            }
        });
        
        removed_species
    }

    pub fn population_by_kingdom(&self) -> (u32, u32, u32) {
        let mut plant_pop = 0;
        let mut animal_pop = 0;
        let mut fungi_pop = 0;

        for creature in &self.creatures {
            match creature.species.kingdom {
                Kingdom::Plant => plant_pop += 1,
                Kingdom::Animal => animal_pop += 1,
                Kingdom::Fungi => fungi_pop += 1,
            }
        }

        (plant_pop, animal_pop, fungi_pop)
    }

    pub fn add_species(&mut self, species: Species) {
        self.creatures.push(Creature::new(species.clone()));
    }

    pub fn current_species(&self) -> Vec<&Species> {
        let mut unique_species = Vec::new();
        let mut seen_names = std::collections::HashSet::new();
        
        for creature in &self.creatures {
            if seen_names.insert(creature.species.name) {
                unique_species.push(&creature.species);
            }
        }
        
        unique_species
    }

    pub fn species_population(&self, species_name: &str) -> u32 {
        self.creatures
            .iter()
            .filter(|creature| creature.species.name == species_name)
            .count() as u32
    }

    pub fn total_species_count(&self) -> usize {
        self.creatures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.creatures.is_empty()
    }

    pub fn run_daily_simulation(&mut self) {        
        let resource_changes = self.net_resource_changes();
        
        self.resources.apply_resource_changes(resource_changes);
        
        self.remove_dead_species();
    }
}

// ===== EVENTS =====

#[derive(Event)]
pub struct AddSpeciesToGardenEvent {
    pub species: Species,
}

#[derive(Event)]
pub struct SimulateDayEvent;

// ===== SYSTEMS =====

pub fn handle_add_species_to_garden_event(
    mut garden_state: ResMut<Garden>,
    mut add_species_events: EventReader<AddSpeciesToGardenEvent>,
    mut trigger_events: EventWriter<SimulateDayEvent>,
) {    
    for event in add_species_events.read() {
        garden_state.add_species(event.species.clone());
            trigger_events.write(SimulateDayEvent);
    }
}

pub fn handle_simulate_day_event(
    mut garden_state: ResMut<Garden>,
    mut simulation_events: EventReader<SimulateDayEvent>,
) {
    for _event in simulation_events.read() {
        garden_state.run_daily_simulation();
    }
}
