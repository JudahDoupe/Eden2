use bevy::prelude::*;
use crate::gameplay::lifecycle::MatterType;
use super::{Kingdom, Species, BiomassConversion};
use std::collections::HashMap;

pub fn get_plant_tier_1() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Clover", 
        Species::new("Clover", Kingdom::Plant, 2, 6, Color::srgb(0.2, 0.8, 0.4))
            // Lifecycle system - Nitrogen-fixing producer
            .with_feeding_requirement(MatterType::SoilNutrients, 1) // Needs some base nutrients
            .with_biomass_conversion(BiomassConversion::PlantGrowth { efficiency: 0.9 })
            .with_growth_age(7) // Moderate growth time
            .with_reproduction_cooldown(15) // Regular reproduction
            .with_lifespan(90, 180) // Seasonal plant
    );

    species
}

pub fn get_plant_tier_2() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Berry Bush", 
        Species::new("Berry Bush", Kingdom::Plant, 2, 6, Color::srgb(0.6, 0.3, 0.7))
            // Lifecycle system - Advanced plant producer
            .with_feeding_requirement(MatterType::SoilNutrients, 2)
            .with_biomass_conversion(BiomassConversion::PlantGrowth { efficiency: 1.0 })
            .with_growth_age(12) // Takes longer to establish
            .with_reproduction_cooldown(20) // Less frequent reproduction
            .with_lifespan(120, 240) // Longer-lived than grass
    );

    species
}

pub fn get_all_plant_species() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();
    
    species.extend(get_plant_tier_1());
    species.extend(get_plant_tier_2());
    
    species
}
