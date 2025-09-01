use bevy::prelude::*;
use crate::gameplay::lifecycle::MatterType;
use super::{Kingdom, Species, BiomassConversion};
use std::collections::HashMap;

pub fn get_animal_tier_1() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Rabbits", 
        Species::new("Rabbits", Kingdom::Animal, 1, 6, Color::srgb(0.6, 0.5, 0.4))
            // Lifecycle system - Primary consumers (herbivores)
            .with_feeding_requirement(MatterType::PlantMatter, 2)
            .with_biomass_conversion(BiomassConversion::PlantToAnimal { efficiency: 0.7 })
            .with_growth_age(14) // Rabbits mature quickly
            .with_reproduction_cooldown(20) // Fast breeding
            .with_lifespan(180, 360) // 6-12 months
    );

    // Add Earthworms - key decomposer species
    species.insert("Earthworms", 
        Species::new("Earthworms", Kingdom::Animal, 1, 8, Color::srgb(0.5, 0.3, 0.2))
            // Lifecycle system - Animal decomposer
            .with_feeding_requirement(MatterType::DeadPlantMatter, 2)
            .with_feeding_requirement(MatterType::DeadAnimalMatter, 1)
            .with_biomass_conversion(BiomassConversion::Decomposition { 
                nutrient_output: 0.8, // Very efficient at creating soil nutrients
                biomass_gain: 0.3, // Slow growth
                matter_type: MatterType::AnimalMatter // Earthworms are animal matter
            })
            .with_growth_age(8) // Quick to mature
            .with_reproduction_cooldown(5) // Frequent reproduction
            .with_lifespan(60, 120) // 2-4 months
    );

    species
}

pub fn get_animal_tier_2() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Frogs", 
        Species::new("Frogs", Kingdom::Animal, 2, 4, Color::srgb(0.2, 0.6, 0.3))
            // Lifecycle system - Secondary consumers (carnivores)
            .with_feeding_requirement(MatterType::AnimalMatter, 2)
            .with_biomass_conversion(BiomassConversion::AnimalToAnimal { efficiency: 0.6 })
            .with_growth_age(25) // Longer development time
            .with_reproduction_cooldown(40) // Less frequent breeding
            .with_lifespan(240, 480) // 8-16 months
    );

    species
}

pub fn get_all_animal_species() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();
    
    species.extend(get_animal_tier_1());
    species.extend(get_animal_tier_2());
    
    species
}
