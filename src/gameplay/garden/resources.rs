use bevy::prelude::*;
use std::collections::HashMap;

// ===== RESOURCE TYPES =====

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ResourceType {
    CO2,
    O2,
    PlantMatter,
    AnimalMatter,
    DeadMatter,
    SoilNutrients,
}

impl ResourceType {
    pub fn name(&self) -> &'static str {
        match self {
            ResourceType::CO2 => "CO2",
            ResourceType::O2 => "O2",
            ResourceType::PlantMatter => "Plant Matter",
            ResourceType::AnimalMatter => "Animal Matter",
            ResourceType::DeadMatter => "Dead Matter",
            ResourceType::SoilNutrients => "Soil Nutrients",
        }
    }

    pub fn all() -> Vec<ResourceType> {
        vec![
            ResourceType::CO2,
            ResourceType::O2,
            ResourceType::PlantMatter,
            ResourceType::AnimalMatter,
            ResourceType::DeadMatter,
            ResourceType::SoilNutrients,
        ]
    }
}

// ===== RESOURCE MANAGEMENT =====

/// Resource manager for the garden ecosystem
#[derive(Resource, Clone)]
pub struct GardenResources {
    pub resources: HashMap<ResourceType, i32>,
}

impl Default for GardenResources {
    fn default() -> Self {
        let mut resources = HashMap::new();
        resources.insert(ResourceType::CO2, 10);
        resources.insert(ResourceType::O2, 10);
        resources.insert(ResourceType::PlantMatter, 0);
        resources.insert(ResourceType::AnimalMatter, 0);
        resources.insert(ResourceType::DeadMatter, 0);
        resources.insert(ResourceType::SoilNutrients, 5);
        
        Self { resources }
    }
}

impl GardenResources {
    /// Get the current amount of a specific resource
    pub fn get_resource(&self, resource_type: ResourceType) -> i32 {
        self.resources.get(&resource_type).copied().unwrap_or(0)
    }
    
    /// Modify a resource by a certain amount (can be positive or negative)
    /// Resources cannot go below 0
    pub fn modify_resource(&mut self, resource_type: ResourceType, change: i32) {
        let current = self.get_resource(resource_type);
        let new_value = (current + change).max(0); // Don't go below 0
        self.resources.insert(resource_type, new_value);
    }

    /// Check if the garden can afford the given resource requirements
    pub fn can_afford(&self, requirements: &HashMap<ResourceType, i32>) -> bool {
        requirements.iter().all(|(resource_type, amount)| {
            self.get_resource(*resource_type) >= *amount
        })
    }

    /// Apply multiple resource changes at once
    pub fn apply_resource_changes(&mut self, changes: HashMap<ResourceType, i32>) {
        for (resource_type, change) in changes {
            self.modify_resource(resource_type, change);
        }
    }
}
