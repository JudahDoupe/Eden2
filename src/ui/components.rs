use bevy::prelude::*;
use crate::types::ResourceType;
use std::collections::HashMap;

/// UI Component marker for resource display text
#[derive(Component)]
pub struct ResourceDisplayText;

/// UI Component marker for species display text
#[derive(Component)]
pub struct SpeciesDisplayText;

/// Component marking an entity as the garden itself
#[derive(Component)]
pub struct Garden;

/// Component representing the resource state of the garden
#[derive(Component)]
pub struct GardenResources {
    pub resources: HashMap<ResourceType, i32>,
}

impl Default for GardenResources {
    fn default() -> Self {
        let mut resources = HashMap::new();
        resources.insert(ResourceType::GroundWater, 5);
        resources.insert(ResourceType::Sunlight, 5);
        resources.insert(ResourceType::SoilNutrients, 5);
        resources.insert(ResourceType::CO2, 10);
        resources.insert(ResourceType::O2, 10);
        resources.insert(ResourceType::GreenVegetation, 0);
        resources.insert(ResourceType::Fruit, 0);
        resources.insert(ResourceType::DeadMatter, 0);
        resources.insert(ResourceType::PlantPopulation, 0);
        resources.insert(ResourceType::AnimalPopulation, 0);
        resources.insert(ResourceType::FungiPopulation, 0);
        
        Self { resources }
    }
}

impl GardenResources {
    pub fn get_resource(&self, resource_type: ResourceType) -> i32 {
        self.resources.get(&resource_type).copied().unwrap_or(0)
    }
    
    pub fn modify_resource(&mut self, resource_type: ResourceType, change: i32) {
        let current = self.get_resource(resource_type);
        let new_value = (current + change).max(0); // Don't go below 0
        self.resources.insert(resource_type, new_value);
    }

    pub fn can_afford(&self, requirements: &HashMap<ResourceType, i32>) -> bool {
        requirements.iter().all(|(resource_type, amount)| {
            self.get_resource(*resource_type) >= *amount
        })
    }
}

/// Component representing a species instance in the garden
#[derive(Component)]
pub struct Species {
    pub card: crate::types::Card,
    pub population: u32,
}

/// Component for daily resource consumption by a species
#[derive(Component)]
pub struct DailyConsumption {
    pub consumption: HashMap<ResourceType, i32>,
}

/// Component for daily resource production by a species
#[derive(Component)]
pub struct DailyProduction {
    pub production: HashMap<ResourceType, i32>,
}

/// Component for survival requirements of a species
#[derive(Component)]
pub struct SurvivalRequirements {
    pub requirements: HashMap<ResourceType, (i32, i32)>, // (min, max) ranges
}

/// UI Component for card entities
#[derive(Component)]
pub struct CardComponent {
    pub card: crate::types::Card,
    pub hand_index: usize,
    pub is_selected: bool,
}

/// UI Component marker for card sprites
#[derive(Component)]
pub struct CardSprite;

/// UI Component marker for card text
#[derive(Component)]
pub struct CardText;

/// UI Component marker for garden background
#[derive(Component)]
pub struct GardenBackground;

/// Resource flag to track if layout has been initialized to actual window size
#[derive(Resource, Default)]
pub struct LayoutInitialized(pub bool);
