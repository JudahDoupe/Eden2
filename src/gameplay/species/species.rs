use bevy::prelude::*;
use crate::gameplay::garden::resources::{GardenResources, ResourceType};
use crate::gameplay::species::plants::{get_all_plant_species};
use crate::gameplay::species::animals::{get_all_animal_species};
use crate::gameplay::species::fungi::{get_all_fungi_species};
use std::collections::HashMap;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Kingdom {
    Plant,
    Animal,
    Fungi,
}

impl Kingdom {
    pub fn name(&self) -> &'static str {
        match self {
            Kingdom::Plant => "Plant",
            Kingdom::Animal => "Animal",
            Kingdom::Fungi => "Fungi",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Species {
    pub name: &'static str,
    pub kingdom: Kingdom,
    pub unlock_round: u32,
    pub max_population: u32,
    pub survival_requirements: HashMap<ResourceType, (i32, i32)>,
    pub daily_consumption: HashMap<ResourceType, i32>,
    pub daily_production: HashMap<ResourceType, i32>,
    pub color: Color,
}

impl Species {
    pub fn new(
        name: &'static str,
        kingdom: Kingdom,
        unlock_round: u32,
        max_population: u32,
        color: Color,
    ) -> Self {
        Self {
            name,
            kingdom,
            unlock_round,
            max_population,
            survival_requirements: HashMap::new(),
            daily_consumption: HashMap::new(),
            daily_production: HashMap::new(),
            color,
        }
    }

    /// Add a survival requirement range (min, max)
    pub fn survival_requirement(mut self, resource: ResourceType, min: i32, max: i32) -> Self {
        self.survival_requirements.insert(resource, (min, max));
        self
    }

    /// Add daily consumption
    pub fn consumes(mut self, resource: ResourceType, amount: i32) -> Self {
        self.daily_consumption.insert(resource, amount);
        self
    }

    /// Add daily production
    pub fn produces(mut self, resource: ResourceType, amount: i32) -> Self {
        self.daily_production.insert(resource, amount);
        self
    }
}

// ===== SPECIES DATA STRUCTURES =====

/// Represents a species instance living in the garden
#[derive(Clone, Debug)]
pub struct SpeciesInstance {
    pub species: Species,
    pub population: u32,
}

impl SpeciesInstance {
    /// Create a new species instance with initial population
    pub fn new(species: Species, initial_population: u32) -> Self {
        Self {
            species,
            population: initial_population,
        }
    }

    /// Get the kingdom this species belongs to
    pub fn kingdom(&self) -> Kingdom {
        self.species.kingdom
    }

    /// Get the species name
    pub fn name(&self) -> &str {
        self.species.name
    }

    /// Get the daily resource consumption for this species (total for all population)
    pub fn total_daily_consumption(&self) -> HashMap<ResourceType, i32> {
        let mut total_consumption = HashMap::new();
        let species_def = &self.species;
        
        for (resource_type, amount) in &species_def.daily_consumption {
            let total = amount * self.population as i32;
            total_consumption.insert(*resource_type, total);
        }
        
        total_consumption
    }

    /// Get the daily resource production for this species (total for all population)
    pub fn total_daily_production(&self) -> HashMap<ResourceType, i32> {
        let mut total_production = HashMap::new();
        let species_def = &self.species;
        
        for (resource_type, amount) in &species_def.daily_production {
            let total = amount * self.population as i32;
            total_production.insert(*resource_type, total);
        }
        
        total_production
    }

    /// Check if this species can survive in the current resource environment
    pub fn can_survive(&self, resources: &GardenResources) -> bool {
        let species_def = &self.species;
        
        for (resource_type, (min, max)) in &species_def.survival_requirements {
            let current_level = resources.get_resource(*resource_type);
            if current_level < *min || current_level > *max {
                return false;
            }
        }
        
        true
    }

    /// Get the survival requirements for this species
    pub fn survival_requirements(&self) -> &HashMap<ResourceType, (i32, i32)> {
        &self.species.survival_requirements
    }
}

/// All species definitions organized by kingdom and tier - this aggregates all the individual functions!
pub fn get_all_species() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    // Plants
    species.extend(get_all_plant_species());

    // Animals
    species.extend(get_all_animal_species());

    // Fungi
    species.extend(get_all_fungi_species());

    species
}

/// Get species definition by name
pub fn get_species(name: &str) -> Option<&Species> {
    static SPECIES_DEFINITIONS: std::sync::OnceLock<HashMap<&'static str, Species>> = std::sync::OnceLock::new();
    let species = SPECIES_DEFINITIONS.get_or_init(get_all_species);
    species.get(name)
}
