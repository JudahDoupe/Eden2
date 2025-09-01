use bevy::prelude::*;
use crate::gameplay::garden::resources::ResourceType;
use super::{Kingdom, Species};
use std::collections::HashMap;

pub fn get_plant_tier_1() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Grass", 
        Species::new("Grass", Kingdom::Plant, 1, 5, Color::srgb(0.4, 0.7, 0.3))
            .with_survival_requirement(ResourceType::Sunlight, 2, 8)
            .with_survival_requirement(ResourceType::GroundWater, 1, 6)
            .with_survival_requirement(ResourceType::SoilNutrients, 1, 5)
            .with_survival_requirement(ResourceType::CO2, 1, 10)
            .with_daily_consumption(ResourceType::CO2, 1)
            .with_daily_consumption(ResourceType::Sunlight, 1)
            .with_daily_consumption(ResourceType::GroundWater, 1)
            .wioth_daily_production(ResourceType::O2, 2)
            .wioth_daily_production(ResourceType::GreenVegetation, 1)
    );

    species.insert("Wildflowers", 
        Species::new("Wildflowers", Kingdom::Plant, 1, 4, Color::srgb(0.9, 0.7, 0.2))
            .with_survival_requirement(ResourceType::Sunlight, 4, 8)
            .with_survival_requirement(ResourceType::GroundWater, 1, 5)
            .with_survival_requirement(ResourceType::SoilNutrients, 1, 4)
            .with_survival_requirement(ResourceType::CO2, 1, 8)
            .with_daily_consumption(ResourceType::CO2, 1)
            .with_daily_consumption(ResourceType::Sunlight, 1)
            .with_daily_consumption(ResourceType::GroundWater, 1)
            .wioth_daily_production(ResourceType::O2, 1)
            .wioth_daily_production(ResourceType::GreenVegetation, 1)
    );

    species.insert("Moss", 
        Species::new("Moss", Kingdom::Plant, 1, 8, Color::srgb(0.3, 0.4, 0.2))
            .with_survival_requirement(ResourceType::Sunlight, 1, 3)
            .with_survival_requirement(ResourceType::GroundWater, 4, 10)
            .with_survival_requirement(ResourceType::SoilNutrients, 1, 3)
            .with_survival_requirement(ResourceType::CO2, 1, 6)
            .with_daily_consumption(ResourceType::CO2, 1)
            .with_daily_consumption(ResourceType::Sunlight, 1)
            .wioth_daily_production(ResourceType::O2, 1)
    );

    species
}

pub fn get_plant_tier_2() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Berry Bushes", 
        Species::new("Berry Bushes", Kingdom::Plant, 2, 3, Color::srgb(0.6, 0.3, 0.7))
            .with_survival_requirement(ResourceType::Sunlight, 3, 7)
            .with_survival_requirement(ResourceType::GroundWater, 2, 6)
            .with_survival_requirement(ResourceType::SoilNutrients, 2, 6)
            .with_survival_requirement(ResourceType::O2, 2, 8)
            .with_daily_consumption(ResourceType::CO2, 1)
            .with_daily_consumption(ResourceType::Sunlight, 2)
            .with_daily_consumption(ResourceType::SoilNutrients, 2)
            .wioth_daily_production(ResourceType::O2, 1)
            .wioth_daily_production(ResourceType::Fruit, 2)
    );

    species.insert("Clover", 
        Species::new("Clover", Kingdom::Plant, 2, 6, Color::srgb(0.3, 0.8, 0.3))
    );

    species.insert("Ferns", 
        Species::new("Ferns", Kingdom::Plant, 2, 4, Color::srgb(0.2, 0.6, 0.4))
    );

    species
}

pub fn get_plant_tier_3() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Oak Saplings", 
        Species::new("Oak Saplings", Kingdom::Plant, 3, 2, Color::srgb(0.2, 0.5, 0.2))
    );

    species.insert("Sunflowers", 
        Species::new("Sunflowers", Kingdom::Plant, 3, 3, Color::srgb(1.0, 0.8, 0.0))
    );

    species.insert("Vegetable Plants", 
        Species::new("Vegetable Plants", Kingdom::Plant, 3, 4, Color::srgb(0.5, 0.7, 0.3))
    );

    species
}

pub fn get_plant_tier_4() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Pine Trees", 
        Species::new("Pine Trees", Kingdom::Plant, 4, 2, Color::srgb(0.1, 0.4, 0.2))
    );

    species
}

pub fn get_all_plant_species() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();
    
    species.extend(get_plant_tier_1());
    species.extend(get_plant_tier_2());
    species.extend(get_plant_tier_3());
    species.extend(get_plant_tier_4());
    
    species
}
