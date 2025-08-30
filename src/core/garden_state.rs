use bevy::prelude::*;
use crate::types::{ResourceType, SpeciesType, Kingdom};
use std::collections::HashMap;

/// Simple garden state with resources and species
#[derive(Resource)]
pub struct GardenState {
    pub resources: HashMap<ResourceType, i32>,
    pub species: Vec<SpeciesInstance>,
}

#[derive(Clone, Debug)]
pub struct SpeciesInstance {
    pub species_type: SpeciesType,
    pub population: u32,
}

impl Default for GardenState {
    fn default() -> Self {
        let mut resources = HashMap::new();
        resources.insert(ResourceType::GroundWater, 5);
        resources.insert(ResourceType::Sunlight, 5);
        resources.insert(ResourceType::SoilNutrients, 5);
        resources.insert(ResourceType::CO2, 10);
        resources.insert(ResourceType::O2, 10);
        resources.insert(ResourceType::GreenVegetation, 0);
        resources.insert(ResourceType::Fruit, 0);
        resources.insert(ResourceType::DeadMatter, 0);
        resources.insert(ResourceType::PlantPopulation, 0);
        resources.insert(ResourceType::AnimalPopulation, 0);
        resources.insert(ResourceType::FungiPopulation, 0);
        
        Self {
            resources,
            species: Vec::new(),
        }
    }
}

impl GardenState {
    pub fn get_resource(&self, resource_type: ResourceType) -> i32 {
        self.resources.get(&resource_type).copied().unwrap_or(0)
    }
    
    pub fn modify_resource(&mut self, resource_type: ResourceType, change: i32) {
        let current = self.get_resource(resource_type);
        let new_value = (current + change).max(0); // Don't go below 0
        self.resources.insert(resource_type, new_value);
    }

    pub fn update_population_counters(&mut self) {
        let mut plant_pop = 0;
        let mut animal_pop = 0;
        let mut fungi_pop = 0;

        for instance in &self.species {
            match instance.species_type.kingdom() {
                Kingdom::Plant => plant_pop += instance.population,
                Kingdom::Animal => animal_pop += instance.population,
                Kingdom::Fungi => fungi_pop += instance.population,
            }
        }

        self.resources.insert(ResourceType::PlantPopulation, plant_pop as i32);
        self.resources.insert(ResourceType::AnimalPopulation, animal_pop as i32);
        self.resources.insert(ResourceType::FungiPopulation, fungi_pop as i32);
    }
    
    pub fn can_afford(&self, requirements: &HashMap<ResourceType, i32>) -> bool {
        for (resource_type, amount) in requirements {
            if self.get_resource(*resource_type) < *amount {
                return false;
            }
        }
        true
    }
    
    pub fn add_species(&mut self, species_type: SpeciesType) -> bool {
        let requirements = species_type.daily_consumption();
        
        if self.can_afford(&requirements) {
            // Pay the costs
            for (resource_type, amount) in requirements {
                self.modify_resource(resource_type, -amount);
            }
            
            // Add the species
            self.species.push(SpeciesInstance {
                species_type,
                population: 1,
            });
            
            // Apply the benefits
            let production = species_type.daily_production();
            for (resource_type, amount) in production {
                self.modify_resource(resource_type, amount);
            }

            // Update population counters
            self.update_population_counters();
            
            true
        } else {
            false
        }
    }
}
