use bevy::prelude::*;
use crate::gameplay::garden::resources::ResourceType;
use super::{Kingdom, Species};
use std::collections::HashMap;

pub fn get_animal_tier_1() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Rabbits", 
        Species::new("Rabbits", Kingdom::Animal, 1, 4, Color::srgb(0.6, 0.5, 0.4))
            .with_survival_requirement(ResourceType::O2, 2, 8)
            .with_survival_requirement(ResourceType::PlantMatter, 2, 8)
            .with_daily_consumption(ResourceType::O2, 1)
            .with_daily_consumption(ResourceType::PlantMatter, 2)
            .with_daily_production(ResourceType::CO2, 1)
            .with_daily_production(ResourceType::AnimalMatter, 1)
            .with_daily_production(ResourceType::DeadMatter, 1)
    );

    species.insert("Earthworms", 
        Species::new("Earthworms", Kingdom::Animal, 1, 8, Color::srgb(0.5, 0.3, 0.2))
            .with_survival_requirement(ResourceType::O2, 1, 5)
            .with_survival_requirement(ResourceType::DeadMatter, 1, 6)
            .with_daily_consumption(ResourceType::O2, 1)
            .with_daily_consumption(ResourceType::DeadMatter, 2)
            .with_daily_production(ResourceType::CO2, 1)
            .with_daily_production(ResourceType::SoilNutrients, 3)
    );

    species
}

pub fn get_animal_tier_2() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Honeybees", 
        Species::new("Honeybees", Kingdom::Animal, 2, 6, Color::srgb(1.0, 0.8, 0.2))
            .with_survival_requirement(ResourceType::O2, 2, 8)
            .with_survival_requirement(ResourceType::PlantMatter, 1, 4)
            .with_daily_consumption(ResourceType::O2, 1)
            .with_daily_consumption(ResourceType::PlantMatter, 1)
            .with_daily_production(ResourceType::CO2, 1)
    );

    species.insert("Field Mice", 
        Species::new("Field Mice", Kingdom::Animal, 2, 5, Color::srgb(0.4, 0.3, 0.2))
            .with_survival_requirement(ResourceType::O2, 2, 7)
            .with_survival_requirement(ResourceType::PlantMatter, 3, 6)
            .with_daily_consumption(ResourceType::O2, 1)
            .with_daily_consumption(ResourceType::PlantMatter, 2)
            .with_daily_production(ResourceType::CO2, 1)
            .with_daily_production(ResourceType::AnimalMatter, 1)
            .with_daily_production(ResourceType::DeadMatter, 1)
    );

    species.insert("Butterflies", 
        Species::new("Butterflies", Kingdom::Animal, 2, 6, Color::srgb(0.8, 0.6, 0.9))
            .with_survival_requirement(ResourceType::O2, 2, 8)
            .with_survival_requirement(ResourceType::PlantMatter, 1, 3)
            .with_daily_consumption(ResourceType::O2, 1)
            .with_daily_consumption(ResourceType::PlantMatter, 1)
            .with_daily_production(ResourceType::CO2, 1)
    );

    species.insert("Snails", 
        Species::new("Snails", Kingdom::Animal, 2, 6, Color::srgb(0.7, 0.6, 0.5))
            .with_survival_requirement(ResourceType::O2, 1, 6)
            .with_survival_requirement(ResourceType::DeadMatter, 1, 4)
            .with_daily_consumption(ResourceType::O2, 1)
            .with_daily_consumption(ResourceType::DeadMatter, 1)
            .with_daily_production(ResourceType::CO2, 1)
            .with_daily_production(ResourceType::SoilNutrients, 1)
    );

    species
}

pub fn get_animal_tier_3() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Ladybugs", 
        Species::new("Ladybugs", Kingdom::Animal, 3, 4, Color::srgb(0.8, 0.2, 0.2))
            .with_survival_requirement(ResourceType::O2, 2, 8)
            .with_survival_requirement(ResourceType::PlantMatter, 1, 5)
            .with_daily_consumption(ResourceType::O2, 1)
            .with_daily_production(ResourceType::CO2, 1)
    );

    species.insert("Frogs", 
        Species::new("Frogs", Kingdom::Animal, 3, 3, Color::srgb(0.2, 0.6, 0.3))
            .with_survival_requirement(ResourceType::O2, 2, 7)
            .with_survival_requirement(ResourceType::AnimalMatter, 2, 10)
            .with_daily_consumption(ResourceType::O2, 1)
            .with_daily_production(ResourceType::CO2, 2)
            .with_daily_production(ResourceType::DeadMatter, 2)
    );

    species
}

pub fn get_animal_tier_4() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Squirrels", 
        Species::new("Squirrels", Kingdom::Animal, 4, 3, Color::srgb(0.6, 0.4, 0.2))
            .with_survival_requirement(ResourceType::O2, 2, 8)
            .with_survival_requirement(ResourceType::PlantMatter, 5, 8)
            .with_daily_consumption(ResourceType::O2, 1)
            .with_daily_consumption(ResourceType::PlantMatter, 2)
            .with_daily_production(ResourceType::CO2, 1)
            .with_daily_production(ResourceType::AnimalMatter, 1)
            .with_daily_production(ResourceType::DeadMatter, 1)
    );

    species.insert("Birds", 
        Species::new("Birds", Kingdom::Animal, 4, 4, Color::srgb(0.4, 0.6, 0.8))
            .with_survival_requirement(ResourceType::O2, 3, 9)
            .with_survival_requirement(ResourceType::PlantMatter, 2, 6)
            .with_survival_requirement(ResourceType::AnimalMatter, 1, 8)
            .with_daily_consumption(ResourceType::O2, 1)
            .with_daily_consumption(ResourceType::PlantMatter, 1)
            .with_daily_production(ResourceType::CO2, 1)
            .with_daily_production(ResourceType::DeadMatter, 1)
    );

    species
}

pub fn get_all_animal_species() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();
    
    species.extend(get_animal_tier_1());
    species.extend(get_animal_tier_2());
    species.extend(get_animal_tier_3());
    species.extend(get_animal_tier_4());
    
    species
}
