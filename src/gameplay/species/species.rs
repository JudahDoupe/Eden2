use bevy::prelude::*;
use crate::gameplay::cards::Card;
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

// ===== SPECIES COLLECTION MANAGEMENT =====

/// A collection of species instances with utility methods
#[derive(Clone, Debug, Default)]
pub struct SpeciesCollection {
    pub species: Vec<SpeciesInstance>,
}

impl SpeciesCollection {
    /// Create a new empty species collection
    pub fn new() -> Self {
        Self {
            species: Vec::new(),
        }
    }

    /// Add a new species to the collection
    pub fn add_species(&mut self, species: Card, initial_population: u32) {
        // Convert Card to Species by getting the species definition
        let species_def = get_species(&species.name()).expect("Species definition not found").clone();
        self.species.push(SpeciesInstance::new(species_def, initial_population));
    }

    /// Remove species that cannot survive in the current environment
    pub fn remove_dead_species(&mut self, resources: &GardenResources) -> Vec<SpeciesInstance> {
        let mut removed_species = Vec::new();
        
        self.species.retain(|species| {
            if species.can_survive(resources) {
                true
            } else {
                removed_species.push(species.clone());
                false
            }
        });
        
        removed_species
    }

    /// Get the total count of all species
    pub fn total_count(&self) -> usize {
        self.species.len()
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.species.is_empty()
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

    /// Get a list of which species can survive in the current environment
    pub fn survival_status(&self, resources: &GardenResources) -> Vec<bool> {
        self.species.iter()
            .map(|species| species.can_survive(resources))
            .collect()
    }

    /// Check if the collection can afford to add a new species
    pub fn can_afford_species(&self, species: &Card, resources: &GardenResources) -> bool {
        let species_def = get_species(species.name()).expect("Species definition not found");
        resources.can_afford(&species_def.daily_consumption)
    }

    /// Get an iterator over the species
    pub fn iter(&self) -> std::slice::Iter<'_, SpeciesInstance> {
        self.species.iter()
    }

    /// Get a mutable iterator over the species
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, SpeciesInstance> {
        self.species.iter_mut()
    }
}

impl<'a> IntoIterator for &'a SpeciesCollection {
    type Item = &'a SpeciesInstance;
    type IntoIter = std::slice::Iter<'a, SpeciesInstance>;

    fn into_iter(self) -> Self::IntoIter {
        self.species.iter()
    }
}

impl<'a> IntoIterator for &'a mut SpeciesCollection {
    type Item = &'a mut SpeciesInstance;
    type IntoIter = std::slice::IterMut<'a, SpeciesInstance>;

    fn into_iter(self) -> Self::IntoIter {
        self.species.iter_mut()
    }
}

// ===== SPECIES EVENTS =====

/// Event for when a species is added to the garden
#[derive(Event)]
pub struct AddSpeciesEvent {
    pub species: Card,
}

/// Event for when species die during simulation
#[derive(Event)]
pub struct SpeciesDeathEvent {
    pub dead_species: Vec<SpeciesInstance>,
    pub reason: String,
}

// ===== LOGGING UTILITIES =====

/// Log resource consumption for all species
pub fn log_species_consumption(species_collection: &SpeciesCollection) {
    for species in &species_collection.species {
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
pub fn log_species_production(species_collection: &SpeciesCollection) {
    for species in &species_collection.species {
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
pub fn log_species_deaths(dead_species: &[SpeciesInstance], resources: &GardenResources) {
    for species in dead_species {
        let requirements = species.survival_requirements();
        
        for (resource_type, (min, max)) in requirements {
            let current_level = resources.get_resource(*resource_type);
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
