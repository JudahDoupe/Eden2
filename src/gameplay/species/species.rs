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

    pub fn with_survival_requirement(mut self, resource: ResourceType, min: i32, max: i32) -> Self {
        self.survival_requirements.insert(resource, (min, max));
        self
    }

    pub fn with_daily_consumption(mut self, resource: ResourceType, amount: i32) -> Self {
        self.daily_consumption.insert(resource, amount);
        self
    }

    pub fn with_daily_production(mut self, resource: ResourceType, amount: i32) -> Self {
        self.daily_production.insert(resource, amount);
        self
    }
}


#[derive(Clone, Debug)]
pub struct Creature {
    pub species: Species,
}

impl Creature {
    pub fn new(species: Species) -> Self {
        Self {
            species,
        }
    }

    pub fn total_daily_consumption(&self) -> HashMap<ResourceType, i32> {
        let mut total_consumption = HashMap::new();
        let species_def = &self.species;
        
        for (resource_type, amount) in &species_def.daily_consumption {
            total_consumption.insert(*resource_type, *amount);
        }
        
        total_consumption
    }

    pub fn total_daily_production(&self) -> HashMap<ResourceType, i32> {
        let mut total_production = HashMap::new();
        let species_def = &self.species;
        
        for (resource_type, amount) in &species_def.daily_production {
            total_production.insert(*resource_type, *amount);
        }
        
        total_production
    }

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

    pub fn survival_requirements(&self) -> &HashMap<ResourceType, (i32, i32)> {
        &self.species.survival_requirements
    }
}

pub fn get_species(name: &str) -> Option<&Species> {
    static SPECIES_DEFINITIONS: std::sync::OnceLock<HashMap<&'static str, Species>> = std::sync::OnceLock::new();
    let species = SPECIES_DEFINITIONS.get_or_init(get_all_species);
    species.get(name)
}

pub fn get_all_species() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();
    species.extend(get_all_plant_species());
    species.extend(get_all_animal_species());
    species.extend(get_all_fungi_species());
    species
}

