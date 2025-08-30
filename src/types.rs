use bevy::prelude::*;
use std::collections::HashMap;

// Simple resource types for the game
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ResourceType {
    Water,
    Sunlight,
    Nutrients,
}

impl ResourceType {
    pub fn name(&self) -> &'static str {
        match self {
            ResourceType::Water => "Water",
            ResourceType::Sunlight => "Sunlight",
            ResourceType::Nutrients => "Nutrients",
        }
    }

    pub fn all() -> Vec<ResourceType> {
        vec![
            ResourceType::Water,
            ResourceType::Sunlight,
            ResourceType::Nutrients,
        ]
    }
}

// Simple plant types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PlantType {
    Grass,
    Flower,
    Tree,
    Bush,
    Moss,
}

impl PlantType {
    pub fn name(&self) -> &'static str {
        match self {
            PlantType::Grass => "Grass",
            PlantType::Flower => "Flower",
            PlantType::Tree => "Tree",
            PlantType::Bush => "Bush",
            PlantType::Moss => "Moss",
        }
    }

    pub fn required_resources(&self) -> HashMap<ResourceType, i32> {
        let mut requirements = HashMap::new();
        match self {
            PlantType::Grass => {
                requirements.insert(ResourceType::Water, 1);
                requirements.insert(ResourceType::Sunlight, 1);
            },
            PlantType::Flower => {
                requirements.insert(ResourceType::Water, 1);
                requirements.insert(ResourceType::Sunlight, 2);
                requirements.insert(ResourceType::Nutrients, 1);
            },
            PlantType::Tree => {
                requirements.insert(ResourceType::Water, 3);
                requirements.insert(ResourceType::Sunlight, 2);
                requirements.insert(ResourceType::Nutrients, 2);
            },
            PlantType::Bush => {
                requirements.insert(ResourceType::Water, 2);
                requirements.insert(ResourceType::Sunlight, 1);
                requirements.insert(ResourceType::Nutrients, 1);
            },
            PlantType::Moss => {
                requirements.insert(ResourceType::Water, 2);
            },
        }
        requirements
    }

    pub fn produced_resources(&self) -> HashMap<ResourceType, i32> {
        let mut production = HashMap::new();
        match self {
            PlantType::Grass => {
                production.insert(ResourceType::Nutrients, 1);
            },
            PlantType::Flower => {
                production.insert(ResourceType::Nutrients, 1);
            },
            PlantType::Tree => {
                production.insert(ResourceType::Water, 1);
                production.insert(ResourceType::Nutrients, 2);
            },
            PlantType::Bush => {
                production.insert(ResourceType::Water, 1);
                production.insert(ResourceType::Nutrients, 1);
            },
            PlantType::Moss => {
                production.insert(ResourceType::Water, 1);
            },
        }
        production
    }

    pub fn color(&self) -> Color {
        match self {
            PlantType::Grass => Color::srgb(0.4, 0.7, 0.3),
            PlantType::Flower => Color::srgb(0.9, 0.7, 0.2),
            PlantType::Tree => Color::srgb(0.2, 0.5, 0.2),
            PlantType::Bush => Color::srgb(0.5, 0.6, 0.3),
            PlantType::Moss => Color::srgb(0.3, 0.4, 0.2),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CardType {
    Plant(PlantType),
}

impl CardType {
    pub fn name(&self) -> &'static str {
        match self {
            CardType::Plant(plant) => plant.name(),
        }
    }
    
    pub fn required_resources(&self) -> HashMap<ResourceType, i32> {
        match self {
            CardType::Plant(plant) => plant.required_resources(),
        }
    }
    
    pub fn produced_resources(&self) -> HashMap<ResourceType, i32> {
        match self {
            CardType::Plant(plant) => plant.produced_resources(),
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            CardType::Plant(plant) => plant.color(),
        }
    }
}
