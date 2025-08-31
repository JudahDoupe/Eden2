use bevy::prelude::*;
use crate::core::card_definitions::{get_card_definition, get_all_card_names};

// Resource types based on the ecosystem simulation
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ResourceType {
    // Basic resources
    GroundWater,
    Sunlight,
    SoilNutrients,
    // Atmospheric gases
    CO2,
    O2,
    // Living matter
    GreenVegetation,
    Fruit,
    DeadMatter,
    // Population counters
    PlantPopulation,
    AnimalPopulation,
    FungiPopulation,
}

impl ResourceType {
    pub fn name(&self) -> &'static str {
        match self {
            ResourceType::GroundWater => "Ground Water",
            ResourceType::Sunlight => "Sunlight",
            ResourceType::SoilNutrients => "Soil Nutrients",
            ResourceType::CO2 => "CO₂",
            ResourceType::O2 => "O₂",
            ResourceType::GreenVegetation => "Green Vegetation",
            ResourceType::Fruit => "Fruit",
            ResourceType::DeadMatter => "Dead Matter",
            ResourceType::PlantPopulation => "Plant Population",
            ResourceType::AnimalPopulation => "Animal Population",
            ResourceType::FungiPopulation => "Fungi Population",
        }
    }

    pub fn all() -> Vec<ResourceType> {
        vec![
            ResourceType::GroundWater,
            ResourceType::Sunlight,
            ResourceType::SoilNutrients,
            ResourceType::CO2,
            ResourceType::O2,
            ResourceType::GreenVegetation,
            ResourceType::Fruit,
            ResourceType::DeadMatter,
            ResourceType::PlantPopulation,
            ResourceType::AnimalPopulation,
            ResourceType::FungiPopulation,
        ]
    }

    /// Returns true if this resource should reset to its base value each day
    /// (renewable resources like sunlight, vs finite resources like nutrients)
    pub fn is_renewable(&self) -> bool {
        match self {
            ResourceType::Sunlight => true,
            ResourceType::CO2 => true,
            ResourceType::O2 => true,
            _ => false,
        }
    }

    /// Returns the daily renewable amount for renewable resources
    pub fn daily_renewable_amount(&self) -> i32 {
        match self {
            ResourceType::Sunlight => 5,
            ResourceType::CO2 => 10,
            ResourceType::O2 => 10,
            _ => 0,
        }
    }
}

/// Card identifier - simply references card definitions by name
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Card {
    name: String,
}

impl Card {
    /// Create a new Card from a name string
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// Get the card name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the full card definition
    pub fn definition(&self) -> &crate::core::card_definitions::CardDefinition {
        get_card_definition(&self.name).expect("Card definition not found")
    }

    /// Get all available cards from the card definitions
    pub fn all() -> Vec<Card> {
        get_all_card_names().iter()
            .map(|name| Card::new(name))
            .collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Kingdom {
    Plant,
    Animal,
    Fungi,
}

impl Kingdom {
    pub fn name(&self) -> &'static str {
        match self {
            Kingdom::Plant => "Plant",
            Kingdom::Animal => "Animal",
            Kingdom::Fungi => "Fungi",
        }
    }
}
