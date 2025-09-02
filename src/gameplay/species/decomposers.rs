use bevy::prelude::*;
use super::{Kingdom, Species};
use super::lifecycle_config::*;
use crate::gameplay::lifecycle::MatterType;
use std::collections::HashMap;

pub fn get_fungi_tier_1() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    // Add Mushrooms - key decomposer species for easy identification
    species.insert("Mushroom", 
        Species::new("Mushroom", Kingdom::Fungi, 1, 5, Color::srgb(0.6, 0.4, 0.3))
            .with_feeding_requirement(MatterType::DeadPlantMatter, 1)
            .with_feeding_requirement(MatterType::DeadAnimalMatter, 1)
            .with_biomass_conversion(BiomassConversion::Decomposition { 
                nutrient_output: 0.5, // Moderate efficiency
                biomass_gain: 0.4, // Better growth than rot fungi
                matter_type: MatterType::PlantMatter
            })
            .with_growth_age(12) // Moderate growth time
            .with_reproduction_cooldown(18) // Regular spore production
            .with_lifespan(45, 90) // Moderate lifespan
    );

    species
}

pub fn get_fungi_tier_2() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Earthworm", 
        Species::new("Earthworm", Kingdom::Animal, 1, 8, Color::srgb(0.5, 0.3, 0.2))
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

pub fn get_all_fungi_species() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();
    
    species.extend(get_fungi_tier_1());
    species.extend(get_fungi_tier_2());
    
    species
}
