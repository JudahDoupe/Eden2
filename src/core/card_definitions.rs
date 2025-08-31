use bevy::prelude::*;
use std::collections::HashMap;
use crate::types::{ResourceType, Kingdom};

/// Complete card definition containing all properties for a species card
#[derive(Clone, Debug)]
pub struct CardDefinition {
    pub name: &'static str,
    pub kingdom: Kingdom,
    pub unlock_round: u32,
    pub max_population: u32,
    pub survival_requirements: HashMap<ResourceType, (i32, i32)>,
    pub daily_consumption: HashMap<ResourceType, i32>,
    pub daily_production: HashMap<ResourceType, i32>,
    pub color: Color,
}

impl CardDefinition {
    /// Helper method to create a new card definition with required fields
    pub fn new(
        name: &'static str,
        kingdom: Kingdom,
        unlock_round: u32,
        max_population: u32,
        color: Color,
    ) -> Self {
        Self {
            name,
            kingdom,
            unlock_round,
            max_population,
            survival_requirements: HashMap::new(),
            daily_consumption: HashMap::new(),
            daily_production: HashMap::new(),
            color,
        }
    }

    /// Add a survival requirement range (min, max)
    pub fn survival_requirement(mut self, resource: ResourceType, min: i32, max: i32) -> Self {
        self.survival_requirements.insert(resource, (min, max));
        self
    }

    /// Add daily consumption
    pub fn consumes(mut self, resource: ResourceType, amount: i32) -> Self {
        self.daily_consumption.insert(resource, amount);
        self
    }

    /// Add daily production
    pub fn produces(mut self, resource: ResourceType, amount: i32) -> Self {
        self.daily_production.insert(resource, amount);
        self
    }
}

/// All card definitions in one place - this is the only place you need to edit to add new cards!
pub fn get_all_card_definitions() -> HashMap<&'static str, CardDefinition> {
    let mut cards = HashMap::new();

    // =============================================================================
    // PLANTS - Round 1
    // =============================================================================
    cards.insert("Grass", 
        CardDefinition::new("Grass", Kingdom::Plant, 1, 5, Color::srgb(0.4, 0.7, 0.3))
            .survival_requirement(ResourceType::Sunlight, 2, 8)
            .survival_requirement(ResourceType::GroundWater, 1, 6)
            .survival_requirement(ResourceType::SoilNutrients, 1, 5)
            .survival_requirement(ResourceType::CO2, 1, 10)
            .consumes(ResourceType::CO2, 1)
            .consumes(ResourceType::Sunlight, 1)
            .consumes(ResourceType::GroundWater, 1)
            .produces(ResourceType::O2, 2)
            .produces(ResourceType::GreenVegetation, 1)
    );

    cards.insert("Wildflowers", 
        CardDefinition::new("Wildflowers", Kingdom::Plant, 1, 4, Color::srgb(0.9, 0.7, 0.2))
            .survival_requirement(ResourceType::Sunlight, 4, 8)
            .survival_requirement(ResourceType::GroundWater, 1, 5)
            .survival_requirement(ResourceType::SoilNutrients, 1, 4)
            .survival_requirement(ResourceType::CO2, 1, 8)
            .consumes(ResourceType::CO2, 1)
            .consumes(ResourceType::Sunlight, 1)
            .consumes(ResourceType::GroundWater, 1)
            .produces(ResourceType::O2, 1)
            .produces(ResourceType::GreenVegetation, 1)
    );

    cards.insert("Moss", 
        CardDefinition::new("Moss", Kingdom::Plant, 1, 8, Color::srgb(0.3, 0.4, 0.2))
            .survival_requirement(ResourceType::Sunlight, 1, 3)
            .survival_requirement(ResourceType::GroundWater, 4, 10)
            .survival_requirement(ResourceType::SoilNutrients, 1, 3)
            .survival_requirement(ResourceType::CO2, 1, 6)
            .consumes(ResourceType::CO2, 1)
            .consumes(ResourceType::Sunlight, 1)
            .produces(ResourceType::O2, 1)
    );

    // =============================================================================
    // PLANTS - Round 2
    // =============================================================================
    cards.insert("Berry Bushes", 
        CardDefinition::new("Berry Bushes", Kingdom::Plant, 2, 3, Color::srgb(0.6, 0.3, 0.7))
            .survival_requirement(ResourceType::Sunlight, 3, 7)
            .survival_requirement(ResourceType::GroundWater, 2, 6)
            .survival_requirement(ResourceType::SoilNutrients, 2, 6)
            .survival_requirement(ResourceType::O2, 2, 8)
            .consumes(ResourceType::CO2, 1)
            .consumes(ResourceType::Sunlight, 2)
            .consumes(ResourceType::SoilNutrients, 2)
            .produces(ResourceType::O2, 1)
            .produces(ResourceType::Fruit, 2)
            .produces(ResourceType::GreenVegetation, 1)
    );

    cards.insert("Clover", 
        CardDefinition::new("Clover", Kingdom::Plant, 2, 6, Color::srgb(0.3, 0.8, 0.3))
            .survival_requirement(ResourceType::Sunlight, 2, 6)
            .survival_requirement(ResourceType::GroundWater, 2, 5)
            .survival_requirement(ResourceType::SoilNutrients, 0, 4)
            .survival_requirement(ResourceType::CO2, 1, 8)
            .consumes(ResourceType::CO2, 2)
            .consumes(ResourceType::Sunlight, 1)
            .consumes(ResourceType::GroundWater, 1)
            .produces(ResourceType::O2, 2)
            .produces(ResourceType::SoilNutrients, 2)
            .produces(ResourceType::GreenVegetation, 1)
    );

    cards.insert("Ferns", 
        CardDefinition::new("Ferns", Kingdom::Plant, 2, 4, Color::srgb(0.2, 0.6, 0.4))
            .survival_requirement(ResourceType::Sunlight, 1, 4)
            .survival_requirement(ResourceType::GroundWater, 3, 8)
            .survival_requirement(ResourceType::SoilNutrients, 2, 6)
            .survival_requirement(ResourceType::CO2, 1, 8)
            .consumes(ResourceType::CO2, 1)
            .consumes(ResourceType::Sunlight, 1)
            .consumes(ResourceType::GroundWater, 3)
            .produces(ResourceType::O2, 1)
            .produces(ResourceType::GreenVegetation, 1)
    );

    // =============================================================================
    // PLANTS - Round 3
    // =============================================================================
    cards.insert("Oak Saplings", 
        CardDefinition::new("Oak Saplings", Kingdom::Plant, 3, 2, Color::srgb(0.2, 0.5, 0.2))
            .survival_requirement(ResourceType::Sunlight, 4, 9)
            .survival_requirement(ResourceType::GroundWater, 3, 7)
            .survival_requirement(ResourceType::SoilNutrients, 3, 8)
            .survival_requirement(ResourceType::CO2, 2, 10)
            .consumes(ResourceType::CO2, 2)
            .consumes(ResourceType::Sunlight, 2)
            .consumes(ResourceType::SoilNutrients, 3)
            .produces(ResourceType::O2, 4)
            .produces(ResourceType::GreenVegetation, 2)
    );

    cards.insert("Sunflowers", 
        CardDefinition::new("Sunflowers", Kingdom::Plant, 3, 3, Color::srgb(1.0, 0.8, 0.0))
            .survival_requirement(ResourceType::Sunlight, 6, 10)
            .survival_requirement(ResourceType::GroundWater, 2, 6)
            .survival_requirement(ResourceType::SoilNutrients, 3, 7)
            .survival_requirement(ResourceType::CO2, 2, 10)
            .consumes(ResourceType::CO2, 2)
            .consumes(ResourceType::Sunlight, 3)
            .consumes(ResourceType::SoilNutrients, 2)
            .produces(ResourceType::O2, 3)
            .produces(ResourceType::GreenVegetation, 1)
            .produces(ResourceType::Fruit, 2)
    );

    cards.insert("Vegetable Plants", 
        CardDefinition::new("Vegetable Plants", Kingdom::Plant, 3, 4, Color::srgb(0.5, 0.7, 0.3))
            .survival_requirement(ResourceType::Sunlight, 5, 8)
            .survival_requirement(ResourceType::GroundWater, 3, 7)
            .survival_requirement(ResourceType::SoilNutrients, 4, 8)
            .survival_requirement(ResourceType::CO2, 2, 8)
            .consumes(ResourceType::CO2, 1)
            .consumes(ResourceType::Sunlight, 2)
            .consumes(ResourceType::SoilNutrients, 3)
            .produces(ResourceType::O2, 1)
            .produces(ResourceType::Fruit, 3)
    );

    // =============================================================================
    // PLANTS - Round 4
    // =============================================================================
    cards.insert("Pine Trees", 
        CardDefinition::new("Pine Trees", Kingdom::Plant, 4, 2, Color::srgb(0.1, 0.4, 0.2))
            .survival_requirement(ResourceType::Sunlight, 3, 8)
            .survival_requirement(ResourceType::GroundWater, 2, 6)
            .survival_requirement(ResourceType::SoilNutrients, 2, 6)
            .survival_requirement(ResourceType::CO2, 2, 10)
            .consumes(ResourceType::CO2, 3)
            .consumes(ResourceType::Sunlight, 2)
            .consumes(ResourceType::SoilNutrients, 2)
            .produces(ResourceType::O2, 4)
            .produces(ResourceType::GreenVegetation, 1)
    );

    // =============================================================================
    // ANIMALS - Round 1
    // =============================================================================
    cards.insert("Rabbits", 
        CardDefinition::new("Rabbits", Kingdom::Animal, 1, 4, Color::srgb(0.6, 0.5, 0.4))
            .survival_requirement(ResourceType::O2, 2, 8)
            .survival_requirement(ResourceType::GreenVegetation, 2, 8)
            .survival_requirement(ResourceType::GroundWater, 1, 6)
            .consumes(ResourceType::O2, 1)
            .consumes(ResourceType::GreenVegetation, 2)
            .consumes(ResourceType::GroundWater, 1)
            .produces(ResourceType::CO2, 1)
            .produces(ResourceType::DeadMatter, 1)
    );

    cards.insert("Earthworms", 
        CardDefinition::new("Earthworms", Kingdom::Animal, 1, 8, Color::srgb(0.5, 0.3, 0.2))
            .survival_requirement(ResourceType::O2, 1, 5)
            .survival_requirement(ResourceType::DeadMatter, 1, 6)
            .survival_requirement(ResourceType::GroundWater, 2, 8)
            .consumes(ResourceType::O2, 1)
            .consumes(ResourceType::DeadMatter, 2)
            .consumes(ResourceType::GroundWater, 1)
            .produces(ResourceType::CO2, 1)
            .produces(ResourceType::SoilNutrients, 3)
    );

    // =============================================================================
    // ANIMALS - Round 2
    // =============================================================================
    cards.insert("Honeybees", 
        CardDefinition::new("Honeybees", Kingdom::Animal, 2, 6, Color::srgb(1.0, 0.8, 0.2))
            .survival_requirement(ResourceType::O2, 2, 8)
            .survival_requirement(ResourceType::Fruit, 1, 4)
            .survival_requirement(ResourceType::Sunlight, 3, 8)
            .consumes(ResourceType::O2, 1)
            .consumes(ResourceType::Fruit, 1)
            .produces(ResourceType::CO2, 1)
    );

    cards.insert("Field Mice", 
        CardDefinition::new("Field Mice", Kingdom::Animal, 2, 5, Color::srgb(0.4, 0.3, 0.2))
            .survival_requirement(ResourceType::O2, 2, 7)
            .survival_requirement(ResourceType::Fruit, 2, 6)
            .survival_requirement(ResourceType::GreenVegetation, 1, 4)
            .consumes(ResourceType::O2, 1)
            .consumes(ResourceType::Fruit, 1)
            .consumes(ResourceType::GreenVegetation, 1)
            .produces(ResourceType::CO2, 1)
            .produces(ResourceType::DeadMatter, 1)
    );

    cards.insert("Butterflies", 
        CardDefinition::new("Butterflies", Kingdom::Animal, 2, 6, Color::srgb(0.8, 0.6, 0.9))
            .survival_requirement(ResourceType::O2, 2, 8)
            .survival_requirement(ResourceType::Fruit, 1, 3)
            .survival_requirement(ResourceType::Sunlight, 4, 9)
            .consumes(ResourceType::O2, 1)
            .consumes(ResourceType::Fruit, 1)
            .produces(ResourceType::CO2, 1)
    );

    cards.insert("Snails", 
        CardDefinition::new("Snails", Kingdom::Animal, 2, 6, Color::srgb(0.7, 0.6, 0.5))
            .survival_requirement(ResourceType::O2, 1, 6)
            .survival_requirement(ResourceType::DeadMatter, 1, 4)
            .survival_requirement(ResourceType::GroundWater, 3, 8)
            .consumes(ResourceType::O2, 1)
            .consumes(ResourceType::DeadMatter, 1)
            .consumes(ResourceType::GroundWater, 2)
            .produces(ResourceType::CO2, 1)
            .produces(ResourceType::SoilNutrients, 1)
    );

    // =============================================================================
    // ANIMALS - Round 3
    // =============================================================================
    cards.insert("Ladybugs", 
        CardDefinition::new("Ladybugs", Kingdom::Animal, 3, 4, Color::srgb(0.8, 0.2, 0.2))
            .survival_requirement(ResourceType::O2, 2, 8)
            .survival_requirement(ResourceType::GreenVegetation, 1, 5)
            .consumes(ResourceType::O2, 1)
            .produces(ResourceType::CO2, 1)
    );

    cards.insert("Frogs", 
        CardDefinition::new("Frogs", Kingdom::Animal, 3, 3, Color::srgb(0.2, 0.6, 0.3))
            .survival_requirement(ResourceType::O2, 2, 7)
            .survival_requirement(ResourceType::GroundWater, 4, 9)
            .survival_requirement(ResourceType::AnimalPopulation, 2, 10)
            .consumes(ResourceType::O2, 1)
            .consumes(ResourceType::GroundWater, 1)
            .produces(ResourceType::CO2, 2)
            .produces(ResourceType::DeadMatter, 2)
    );

    // =============================================================================
    // ANIMALS - Round 4
    // =============================================================================
    cards.insert("Squirrels", 
        CardDefinition::new("Squirrels", Kingdom::Animal, 4, 3, Color::srgb(0.6, 0.4, 0.2))
            .survival_requirement(ResourceType::O2, 2, 8)
            .survival_requirement(ResourceType::Fruit, 2, 6)
            .survival_requirement(ResourceType::GreenVegetation, 3, 8)
            .consumes(ResourceType::O2, 1)
            .consumes(ResourceType::Fruit, 2)
            .produces(ResourceType::CO2, 1)
            .produces(ResourceType::DeadMatter, 1)
    );

    cards.insert("Birds", 
        CardDefinition::new("Birds", Kingdom::Animal, 4, 4, Color::srgb(0.4, 0.6, 0.8))
            .survival_requirement(ResourceType::O2, 3, 9)
            .survival_requirement(ResourceType::Fruit, 2, 6)
            .survival_requirement(ResourceType::AnimalPopulation, 1, 8)
            .consumes(ResourceType::O2, 1)
            .consumes(ResourceType::Fruit, 1)
            .produces(ResourceType::CO2, 1)
            .produces(ResourceType::DeadMatter, 1)
    );

    // =============================================================================
    // FUNGI - Round 1
    // =============================================================================
    cards.insert("Rot Fungi", 
        CardDefinition::new("Rot Fungi", Kingdom::Fungi, 1, 6, Color::srgb(0.4, 0.3, 0.2))
            .survival_requirement(ResourceType::DeadMatter, 2, 8)
            .survival_requirement(ResourceType::GroundWater, 2, 7)
            .survival_requirement(ResourceType::O2, 0, 4)
            .consumes(ResourceType::DeadMatter, 2)
            .consumes(ResourceType::GroundWater, 1)
            .produces(ResourceType::CO2, 1)
            .produces(ResourceType::SoilNutrients, 2)
    );

    cards.insert("Mold Clusters", 
        CardDefinition::new("Mold Clusters", Kingdom::Fungi, 1, 8, Color::srgb(0.3, 0.5, 0.3))
            .survival_requirement(ResourceType::DeadMatter, 1, 6)
            .survival_requirement(ResourceType::GroundWater, 3, 8)
            .survival_requirement(ResourceType::O2, 0, 5)
            .consumes(ResourceType::DeadMatter, 1)
            .consumes(ResourceType::GroundWater, 1)
            .produces(ResourceType::CO2, 1)
            .produces(ResourceType::SoilNutrients, 2)
    );

    // =============================================================================
    // FUNGI - Round 2
    // =============================================================================
    cards.insert("Puffballs", 
        CardDefinition::new("Puffballs", Kingdom::Fungi, 2, 4, Color::srgb(0.8, 0.8, 0.7))
            .survival_requirement(ResourceType::DeadMatter, 1, 5)
            .survival_requirement(ResourceType::SoilNutrients, 2, 6)
            .survival_requirement(ResourceType::GroundWater, 1, 5)
            .consumes(ResourceType::DeadMatter, 1)
            .consumes(ResourceType::SoilNutrients, 1)
            .produces(ResourceType::CO2, 1)
            .produces(ResourceType::SoilNutrients, 1)
    );

    cards.insert("Mycorrhizal Fungi", 
        CardDefinition::new("Mycorrhizal Fungi", Kingdom::Fungi, 2, 5, Color::srgb(0.5, 0.4, 0.3))
            .survival_requirement(ResourceType::PlantPopulation, 2, 10)
            .survival_requirement(ResourceType::SoilNutrients, 1, 6)
            .survival_requirement(ResourceType::GroundWater, 2, 6)
            .consumes(ResourceType::SoilNutrients, 1)
            .consumes(ResourceType::GroundWater, 1)
            .produces(ResourceType::CO2, 1)
    );

    cards.insert("Yeast Colonies", 
        CardDefinition::new("Yeast Colonies", Kingdom::Fungi, 2, 10, Color::srgb(0.7, 0.7, 0.6))
            .survival_requirement(ResourceType::Fruit, 1, 6)
            .survival_requirement(ResourceType::GroundWater, 2, 6)
            .survival_requirement(ResourceType::O2, 0, 3)
            .consumes(ResourceType::Fruit, 1)
            .consumes(ResourceType::GroundWater, 1)
            .produces(ResourceType::CO2, 2)
            .produces(ResourceType::SoilNutrients, 1)
    );

    // =============================================================================
    // FUNGI - Round 3
    // =============================================================================
    cards.insert("Giant Mushrooms", 
        CardDefinition::new("Giant Mushrooms", Kingdom::Fungi, 3, 2, Color::srgb(0.6, 0.4, 0.3))
            .survival_requirement(ResourceType::DeadMatter, 3, 8)
            .survival_requirement(ResourceType::SoilNutrients, 3, 7)
            .survival_requirement(ResourceType::GroundWater, 3, 7)
            .survival_requirement(ResourceType::Sunlight, 0, 3)
            .consumes(ResourceType::DeadMatter, 3)
            .consumes(ResourceType::SoilNutrients, 2)
            .produces(ResourceType::CO2, 2)
            .produces(ResourceType::SoilNutrients, 4)
            .produces(ResourceType::Fruit, 1)
    );

    cards.insert("Shelf Fungi", 
        CardDefinition::new("Shelf Fungi", Kingdom::Fungi, 3, 4, Color::srgb(0.7, 0.5, 0.3))
            .survival_requirement(ResourceType::DeadMatter, 2, 6)
            .survival_requirement(ResourceType::GreenVegetation, 3, 8)
            .survival_requirement(ResourceType::GroundWater, 1, 5)
            .consumes(ResourceType::DeadMatter, 1)
            .consumes(ResourceType::GreenVegetation, 1)
            .produces(ResourceType::CO2, 1)
            .produces(ResourceType::SoilNutrients, 2)
    );

    cards.insert("Coral Fungi", 
        CardDefinition::new("Coral Fungi", Kingdom::Fungi, 3, 4, Color::srgb(0.9, 0.7, 0.5))
            .survival_requirement(ResourceType::DeadMatter, 2, 6)
            .survival_requirement(ResourceType::SoilNutrients, 2, 5)
            .survival_requirement(ResourceType::FungiPopulation, 2, 8)
            .consumes(ResourceType::DeadMatter, 1)
            .consumes(ResourceType::SoilNutrients, 1)
            .produces(ResourceType::CO2, 1)
            .produces(ResourceType::SoilNutrients, 2)
    );

    // =============================================================================
    // FUNGI - Round 4
    // =============================================================================
    cards.insert("Truffle Fungi", 
        CardDefinition::new("Truffle Fungi", Kingdom::Fungi, 4, 3, Color::srgb(0.3, 0.2, 0.1))
            .survival_requirement(ResourceType::SoilNutrients, 4, 8)
            .survival_requirement(ResourceType::GroundWater, 3, 7)
            .survival_requirement(ResourceType::PlantPopulation, 3, 8)
            .consumes(ResourceType::SoilNutrients, 2)
            .consumes(ResourceType::GroundWater, 1)
            .produces(ResourceType::CO2, 1)
            .produces(ResourceType::SoilNutrients, 1)
            .produces(ResourceType::Fruit, 2)
    );

    cards.insert("Slime Molds", 
        CardDefinition::new("Slime Molds", Kingdom::Fungi, 4, 3, Color::srgb(0.6, 0.8, 0.4))
            .survival_requirement(ResourceType::DeadMatter, 3, 8)
            .survival_requirement(ResourceType::GroundWater, 4, 9)
            .survival_requirement(ResourceType::O2, 1, 6)
            .consumes(ResourceType::DeadMatter, 2)
            .consumes(ResourceType::GroundWater, 2)
            .produces(ResourceType::CO2, 1)
            .produces(ResourceType::SoilNutrients, 3)
    );

    cards
}

/// Get card definition by name
pub fn get_card_definition(name: &str) -> Option<&CardDefinition> {
    static CARD_DEFINITIONS: std::sync::OnceLock<HashMap<&'static str, CardDefinition>> = std::sync::OnceLock::new();
    let cards = CARD_DEFINITIONS.get_or_init(get_all_card_definitions);
    cards.get(name)
}

/// Get all card names in the order they should appear
pub fn get_all_card_names() -> Vec<&'static str> {
    vec![
        // Plants
        "Grass", "Wildflowers", "Moss",
        "Berry Bushes", "Clover", "Ferns",
        "Oak Saplings", "Sunflowers", "Vegetable Plants",
        "Pine Trees",
        // Animals
        "Rabbits", "Earthworms",
        "Honeybees", "Field Mice", "Butterflies", "Snails",
        "Ladybugs", "Frogs",
        "Squirrels", "Birds",
        // Fungi
        "Rot Fungi", "Mold Clusters",
        "Puffballs", "Mycorrhizal Fungi", "Yeast Colonies",
        "Giant Mushrooms", "Shelf Fungi", "Coral Fungi",
        "Truffle Fungi", "Slime Molds",
    ]
}
