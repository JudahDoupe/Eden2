use bevy::prelude::*;
use crate::gameplay::species::plants::{get_all_plant_species};
use crate::gameplay::species::animals::{get_all_animal_species};
use crate::gameplay::species::decomposers::{get_all_fungi_species};
use crate::gameplay::species::lifecycle_config::*;
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
    pub color: Color,
    
    // Lifecycle fields
    pub feeding_requirements: FeedingRequirements,
    pub growth_requirements: GrowthRequirements,
    pub reproduction_requirements: ReproductionRequirements,
    pub mortality_factors: MortalityFactors,
    pub biomass_composition: BiomassComposition,
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
            color,
            // Initialize lifecycle fields with defaults
            feeding_requirements: FeedingRequirements::default(),
            growth_requirements: GrowthRequirements::default(),
            reproduction_requirements: ReproductionRequirements::default(),
            mortality_factors: MortalityFactors::default(),
            biomass_composition: match kingdom {
                Kingdom::Plant | Kingdom::Fungi => BiomassComposition::Plant,
                Kingdom::Animal => BiomassComposition::Animal,
            },
        }
    }

    // Lifecycle configuration methods
    pub fn with_feeding_requirement(mut self, matter_type: crate::gameplay::lifecycle::MatterType, amount: u32) -> Self {
        self.feeding_requirements.base_requirements.insert(matter_type, amount);
        self
    }

    pub fn with_biomass_conversion(mut self, conversion: BiomassConversion) -> Self {
        self.feeding_requirements.biomass_conversion = conversion;
        self
    }

    pub fn with_growth_age(mut self, minimum_age: u32) -> Self {
        self.growth_requirements.minimum_age = minimum_age;
        self
    }

    pub fn with_reproduction_cooldown(mut self, days: u32) -> Self {
        self.reproduction_requirements.cooldown_days = days;
        self
    }

    pub fn with_lifespan(mut self, min_days: u32, max_days: u32) -> Self {
        self.mortality_factors.natural_lifespan = (min_days, max_days);
        self
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

