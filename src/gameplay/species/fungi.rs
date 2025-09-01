use bevy::prelude::*;
use super::{Kingdom, Species};
use super::lifecycle_config::*;
use crate::gameplay::lifecycle::MatterType;
use std::collections::HashMap;

pub fn get_fungi_tier_1() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Rot Fungi", 
        Species::new("Rot Fungi", Kingdom::Fungi, 1, 6, Color::srgb(0.4, 0.3, 0.2))
            .with_feeding_requirement(MatterType::DeadPlantMatter, 1)
            .with_feeding_requirement(MatterType::DeadAnimalMatter, 1)
            .with_biomass_conversion(BiomassConversion::Decomposition { 
                nutrient_output: 0.6, 
                biomass_gain: 0.3, 
                matter_type: MatterType::PlantMatter // Fungi biomass acts like plant matter
            })
            .with_growth_age(10) // Fast-growing decomposer
            .with_reproduction_cooldown(15) // Frequent spore production
            .with_lifespan(30, 60) // Short-lived but prolific
    );

    // Add Mushrooms - key decomposer species for easy identification
    species.insert("Mushrooms", 
        Species::new("Mushrooms", Kingdom::Fungi, 1, 5, Color::srgb(0.6, 0.4, 0.3))
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

    species.insert("Mycorrhizal Fungi", 
        Species::new("Mycorrhizal Fungi", Kingdom::Fungi, 2, 4, Color::srgb(0.5, 0.4, 0.3))
            .with_feeding_requirement(MatterType::SoilNutrients, 1)
            .with_feeding_requirement(MatterType::DeadPlantMatter, 1)
            .with_biomass_conversion(BiomassConversion::Decomposition { 
                nutrient_output: 0.8, 
                biomass_gain: 0.4, 
                matter_type: MatterType::PlantMatter
            })
            .with_growth_age(15) // Slower development for symbiotic fungi
            .with_reproduction_cooldown(25) // Less frequent reproduction
            .with_lifespan(60, 120) // Longer-lived specialist fungi
    );

    species
}

pub fn get_all_fungi_species() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();
    
    species.extend(get_fungi_tier_1());
    species.extend(get_fungi_tier_2());
    
    species
}
