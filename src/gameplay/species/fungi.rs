use bevy::prelude::*;
use crate::gameplay::garden::resources::ResourceType;
use super::{Kingdom, Species};
use std::collections::HashMap;

pub fn get_fungi_tier_1() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Rot Fungi", 
        Species::new("Rot Fungi", Kingdom::Fungi, 1, 6, Color::srgb(0.4, 0.3, 0.2))
            .with_survival_requirement(ResourceType::DeadMatter, 2, 8)
            .with_survival_requirement(ResourceType::O2, 0, 4)
            .with_daily_consumption(ResourceType::DeadMatter, 2)
            .with_daily_production(ResourceType::CO2, 1)
            .with_daily_production(ResourceType::SoilNutrients, 2)
    );

    species.insert("Mold Clusters", 
        Species::new("Mold Clusters", Kingdom::Fungi, 1, 8, Color::srgb(0.3, 0.5, 0.3))
            .with_survival_requirement(ResourceType::DeadMatter, 1, 6)
            .with_survival_requirement(ResourceType::O2, 0, 5)
            .with_daily_consumption(ResourceType::DeadMatter, 1)
            .with_daily_production(ResourceType::CO2, 1)
            .with_daily_production(ResourceType::SoilNutrients, 2)
    );

    species
}

pub fn get_fungi_tier_2() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Puffballs", 
        Species::new("Puffballs", Kingdom::Fungi, 2, 4, Color::srgb(0.8, 0.8, 0.7))
            .with_survival_requirement(ResourceType::DeadMatter, 1, 5)
            .with_survival_requirement(ResourceType::SoilNutrients, 2, 6)
            .with_daily_consumption(ResourceType::DeadMatter, 1)
            .with_daily_consumption(ResourceType::SoilNutrients, 1)
            .with_daily_production(ResourceType::CO2, 1)
            .with_daily_production(ResourceType::SoilNutrients, 1)
    );

    species.insert("Mycorrhizal Fungi", 
        Species::new("Mycorrhizal Fungi", Kingdom::Fungi, 2, 5, Color::srgb(0.5, 0.4, 0.3))
            .with_survival_requirement(ResourceType::PlantMatter, 2, 10)
            .with_survival_requirement(ResourceType::SoilNutrients, 1, 6)
            .with_daily_consumption(ResourceType::SoilNutrients, 1)
            .with_daily_production(ResourceType::CO2, 1)
    );

    species.insert("Yeast Colonies", 
        Species::new("Yeast Colonies", Kingdom::Fungi, 2, 10, Color::srgb(0.7, 0.7, 0.6))
            .with_survival_requirement(ResourceType::PlantMatter, 1, 6)
            .with_survival_requirement(ResourceType::O2, 0, 3)
            .with_daily_consumption(ResourceType::PlantMatter, 1)
            .with_daily_production(ResourceType::CO2, 2)
            .with_daily_production(ResourceType::SoilNutrients, 1)
    );

    species
}

pub fn get_fungi_tier_3() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Giant Mushrooms", 
        Species::new("Giant Mushrooms", Kingdom::Fungi, 3, 2, Color::srgb(0.6, 0.4, 0.3))
            .with_survival_requirement(ResourceType::DeadMatter, 3, 8)
            .with_survival_requirement(ResourceType::SoilNutrients, 3, 7)
            .with_survival_requirement(ResourceType::O2, 0, 3)
            .with_daily_consumption(ResourceType::DeadMatter, 3)
            .with_daily_consumption(ResourceType::SoilNutrients, 2)
            .with_daily_production(ResourceType::CO2, 2)
            .with_daily_production(ResourceType::SoilNutrients, 4)
    );

    species.insert("Shelf Fungi", 
        Species::new("Shelf Fungi", Kingdom::Fungi, 3, 4, Color::srgb(0.7, 0.5, 0.3))
            .with_survival_requirement(ResourceType::DeadMatter, 2, 6)
            .with_survival_requirement(ResourceType::PlantMatter, 3, 8)
            .with_daily_consumption(ResourceType::DeadMatter, 1)
            .with_daily_consumption(ResourceType::PlantMatter, 1)
            .with_daily_production(ResourceType::CO2, 1)
            .with_daily_production(ResourceType::SoilNutrients, 2)
    );

    species.insert("Coral Fungi", 
        Species::new("Coral Fungi", Kingdom::Fungi, 3, 4, Color::srgb(0.9, 0.7, 0.5))
            .with_survival_requirement(ResourceType::DeadMatter, 2, 6)
            .with_survival_requirement(ResourceType::SoilNutrients, 2, 5)
            .with_daily_consumption(ResourceType::DeadMatter, 1)
            .with_daily_consumption(ResourceType::SoilNutrients, 1)
            .with_daily_production(ResourceType::CO2, 1)
            .with_daily_production(ResourceType::SoilNutrients, 2)
    );

    species
}

pub fn get_fungi_tier_4() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Truffle Fungi", 
        Species::new("Truffle Fungi", Kingdom::Fungi, 4, 3, Color::srgb(0.3, 0.2, 0.1))
            .with_survival_requirement(ResourceType::SoilNutrients, 4, 8)
            .with_survival_requirement(ResourceType::PlantMatter, 3, 8)
            .with_daily_consumption(ResourceType::SoilNutrients, 2)
            .with_daily_production(ResourceType::CO2, 1)
            .with_daily_production(ResourceType::SoilNutrients, 1)
    );

    species.insert("Slime Molds", 
        Species::new("Slime Molds", Kingdom::Fungi, 4, 3, Color::srgb(0.6, 0.8, 0.4))
            .with_survival_requirement(ResourceType::DeadMatter, 3, 8)
            .with_survival_requirement(ResourceType::O2, 1, 6)
            .with_daily_consumption(ResourceType::DeadMatter, 2)
            .with_daily_production(ResourceType::CO2, 1)
            .with_daily_production(ResourceType::SoilNutrients, 3)
    );

    species
}

pub fn get_all_fungi_species() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();
    
    species.extend(get_fungi_tier_1());
    species.extend(get_fungi_tier_2());
    species.extend(get_fungi_tier_3());
    species.extend(get_fungi_tier_4());
    
    species
}
