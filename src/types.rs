use bevy::prelude::*;
use std::collections::HashMap;

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
}

// All species types from the documentation
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SpeciesType {
    // Plants
    Grass,
    BerryBushes,
    Wildflowers,
    OakSaplings,
    Clover,
    Ferns,
    Sunflowers,
    Moss,
    PineTrees,
    VegetablePlants,
    
    // Animals
    Rabbits,
    Earthworms,
    Honeybees,
    FieldMice,
    Butterflies,
    Ladybugs,
    Frogs,
    Squirrels,
    Snails,
    Birds,
    
    // Fungi
    RotFungi,
    Puffballs,
    GiantMushrooms,
    MycorrhizalFungi,
    ShelfFungi,
    MoldClusters,
    TruffleFungi,
    CoralFungi,
    YeastColonies,
    SlimeMolds,
}

impl SpeciesType {
    pub fn name(&self) -> &'static str {
        match self {
            // Plants
            SpeciesType::Grass => "Grass",
            SpeciesType::BerryBushes => "Berry Bushes",
            SpeciesType::Wildflowers => "Wildflowers",
            SpeciesType::OakSaplings => "Oak Saplings",
            SpeciesType::Clover => "Clover",
            SpeciesType::Ferns => "Ferns",
            SpeciesType::Sunflowers => "Sunflowers",
            SpeciesType::Moss => "Moss",
            SpeciesType::PineTrees => "Pine Trees",
            SpeciesType::VegetablePlants => "Vegetable Plants",
            
            // Animals
            SpeciesType::Rabbits => "Rabbits",
            SpeciesType::Earthworms => "Earthworms",
            SpeciesType::Honeybees => "Honeybees",
            SpeciesType::FieldMice => "Field Mice",
            SpeciesType::Butterflies => "Butterflies",
            SpeciesType::Ladybugs => "Ladybugs",
            SpeciesType::Frogs => "Frogs",
            SpeciesType::Squirrels => "Squirrels",
            SpeciesType::Snails => "Snails",
            SpeciesType::Birds => "Birds",
            
            // Fungi
            SpeciesType::RotFungi => "Rot Fungi",
            SpeciesType::Puffballs => "Puffballs",
            SpeciesType::GiantMushrooms => "Giant Mushrooms",
            SpeciesType::MycorrhizalFungi => "Mycorrhizal Fungi",
            SpeciesType::ShelfFungi => "Shelf Fungi",
            SpeciesType::MoldClusters => "Mold Clusters",
            SpeciesType::TruffleFungi => "Truffle Fungi",
            SpeciesType::CoralFungi => "Coral Fungi",
            SpeciesType::YeastColonies => "Yeast Colonies",
            SpeciesType::SlimeMolds => "Slime Molds",
        }
    }

    pub fn kingdom(&self) -> Kingdom {
        match self {
            SpeciesType::Grass | SpeciesType::BerryBushes | SpeciesType::Wildflowers | 
            SpeciesType::OakSaplings | SpeciesType::Clover | SpeciesType::Ferns | 
            SpeciesType::Sunflowers | SpeciesType::Moss | SpeciesType::PineTrees | 
            SpeciesType::VegetablePlants => Kingdom::Plant,
            
            SpeciesType::Rabbits | SpeciesType::Earthworms | SpeciesType::Honeybees | 
            SpeciesType::FieldMice | SpeciesType::Butterflies | SpeciesType::Ladybugs | 
            SpeciesType::Frogs | SpeciesType::Squirrels | SpeciesType::Snails | 
            SpeciesType::Birds => Kingdom::Animal,
            
            SpeciesType::RotFungi | SpeciesType::Puffballs | SpeciesType::GiantMushrooms | 
            SpeciesType::MycorrhizalFungi | SpeciesType::ShelfFungi | SpeciesType::MoldClusters | 
            SpeciesType::TruffleFungi | SpeciesType::CoralFungi | SpeciesType::YeastColonies | 
            SpeciesType::SlimeMolds => Kingdom::Fungi,
        }
    }

    pub fn unlock_round(&self) -> u32 {
        match self {
            // Round 1 unlocks
            SpeciesType::Grass | SpeciesType::Wildflowers | SpeciesType::Moss |
            SpeciesType::Rabbits | SpeciesType::Earthworms |
            SpeciesType::RotFungi | SpeciesType::MoldClusters => 1,
            
            // Round 2 unlocks
            SpeciesType::BerryBushes | SpeciesType::Clover | SpeciesType::Ferns |
            SpeciesType::Honeybees | SpeciesType::FieldMice | SpeciesType::Butterflies | SpeciesType::Snails |
            SpeciesType::Puffballs | SpeciesType::MycorrhizalFungi | SpeciesType::YeastColonies => 2,
            
            // Round 3 unlocks
            SpeciesType::OakSaplings | SpeciesType::Sunflowers | SpeciesType::VegetablePlants |
            SpeciesType::Ladybugs | SpeciesType::Frogs |
            SpeciesType::GiantMushrooms | SpeciesType::ShelfFungi | SpeciesType::CoralFungi => 3,
            
            // Round 4 unlocks
            SpeciesType::PineTrees |
            SpeciesType::Squirrels | SpeciesType::Birds |
            SpeciesType::TruffleFungi | SpeciesType::SlimeMolds => 4,
        }
    }

    pub fn max_population(&self) -> u32 {
        match self {
            // Plants
            SpeciesType::Grass => 5,
            SpeciesType::BerryBushes => 3,
            SpeciesType::Wildflowers => 4,
            SpeciesType::OakSaplings => 2,
            SpeciesType::Clover => 6,
            SpeciesType::Ferns => 4,
            SpeciesType::Sunflowers => 3,
            SpeciesType::Moss => 8,
            SpeciesType::PineTrees => 2,
            SpeciesType::VegetablePlants => 4,
            
            // Animals
            SpeciesType::Rabbits => 4,
            SpeciesType::Earthworms => 8,
            SpeciesType::Honeybees => 6,
            SpeciesType::FieldMice => 5,
            SpeciesType::Butterflies => 6,
            SpeciesType::Ladybugs => 4,
            SpeciesType::Frogs => 3,
            SpeciesType::Squirrels => 3,
            SpeciesType::Snails => 6,
            SpeciesType::Birds => 4,
            
            // Fungi
            SpeciesType::RotFungi => 6,
            SpeciesType::Puffballs => 4,
            SpeciesType::GiantMushrooms => 2,
            SpeciesType::MycorrhizalFungi => 5,
            SpeciesType::ShelfFungi => 4,
            SpeciesType::MoldClusters => 8,
            SpeciesType::TruffleFungi => 3,
            SpeciesType::CoralFungi => 4,
            SpeciesType::YeastColonies => 10,
            SpeciesType::SlimeMolds => 3,
        }
    }

    pub fn survival_requirements(&self) -> HashMap<ResourceType, (i32, i32)> {
        let mut requirements = HashMap::new();
        match self {
            // Plants
            SpeciesType::Grass => {
                requirements.insert(ResourceType::Sunlight, (2, 8));
                requirements.insert(ResourceType::GroundWater, (1, 6));
                requirements.insert(ResourceType::SoilNutrients, (1, 5));
                requirements.insert(ResourceType::CO2, (1, 10));
            },
            SpeciesType::BerryBushes => {
                requirements.insert(ResourceType::Sunlight, (3, 7));
                requirements.insert(ResourceType::GroundWater, (2, 6));
                requirements.insert(ResourceType::SoilNutrients, (2, 6));
                requirements.insert(ResourceType::O2, (2, 8));
            },
            SpeciesType::Wildflowers => {
                requirements.insert(ResourceType::Sunlight, (4, 8));
                requirements.insert(ResourceType::GroundWater, (1, 5));
                requirements.insert(ResourceType::SoilNutrients, (1, 4));
                requirements.insert(ResourceType::CO2, (1, 8));
            },
            SpeciesType::OakSaplings => {
                requirements.insert(ResourceType::Sunlight, (4, 9));
                requirements.insert(ResourceType::GroundWater, (3, 7));
                requirements.insert(ResourceType::SoilNutrients, (3, 8));
                requirements.insert(ResourceType::CO2, (2, 10));
            },
            SpeciesType::Clover => {
                requirements.insert(ResourceType::Sunlight, (2, 6));
                requirements.insert(ResourceType::GroundWater, (2, 5));
                requirements.insert(ResourceType::SoilNutrients, (0, 4));
                requirements.insert(ResourceType::CO2, (1, 8));
            },
            SpeciesType::Ferns => {
                requirements.insert(ResourceType::Sunlight, (1, 4));
                requirements.insert(ResourceType::GroundWater, (3, 8));
                requirements.insert(ResourceType::SoilNutrients, (2, 6));
                requirements.insert(ResourceType::CO2, (1, 8));
            },
            SpeciesType::Sunflowers => {
                requirements.insert(ResourceType::Sunlight, (6, 10));
                requirements.insert(ResourceType::GroundWater, (2, 6));
                requirements.insert(ResourceType::SoilNutrients, (3, 7));
                requirements.insert(ResourceType::CO2, (2, 10));
            },
            SpeciesType::Moss => {
                requirements.insert(ResourceType::Sunlight, (1, 3));
                requirements.insert(ResourceType::GroundWater, (4, 10));
                requirements.insert(ResourceType::SoilNutrients, (1, 3));
                requirements.insert(ResourceType::CO2, (1, 6));
            },
            SpeciesType::PineTrees => {
                requirements.insert(ResourceType::Sunlight, (3, 8));
                requirements.insert(ResourceType::GroundWater, (2, 6));
                requirements.insert(ResourceType::SoilNutrients, (2, 6));
                requirements.insert(ResourceType::CO2, (2, 10));
            },
            SpeciesType::VegetablePlants => {
                requirements.insert(ResourceType::Sunlight, (5, 8));
                requirements.insert(ResourceType::GroundWater, (3, 7));
                requirements.insert(ResourceType::SoilNutrients, (4, 8));
                requirements.insert(ResourceType::CO2, (2, 8));
            },
            
            // Animals
            SpeciesType::Rabbits => {
                requirements.insert(ResourceType::O2, (2, 8));
                requirements.insert(ResourceType::GreenVegetation, (2, 8));
                requirements.insert(ResourceType::GroundWater, (1, 6));
            },
            SpeciesType::Earthworms => {
                requirements.insert(ResourceType::O2, (1, 5));
                requirements.insert(ResourceType::DeadMatter, (1, 6));
                requirements.insert(ResourceType::GroundWater, (2, 8));
            },
            SpeciesType::Honeybees => {
                requirements.insert(ResourceType::O2, (2, 8));
                requirements.insert(ResourceType::Fruit, (1, 4));
                requirements.insert(ResourceType::Sunlight, (3, 8));
            },
            SpeciesType::FieldMice => {
                requirements.insert(ResourceType::O2, (2, 7));
                requirements.insert(ResourceType::Fruit, (2, 6));
                requirements.insert(ResourceType::GreenVegetation, (1, 4));
            },
            SpeciesType::Butterflies => {
                requirements.insert(ResourceType::O2, (2, 8));
                requirements.insert(ResourceType::Fruit, (1, 3));
                requirements.insert(ResourceType::Sunlight, (4, 9));
            },
            SpeciesType::Ladybugs => {
                requirements.insert(ResourceType::O2, (2, 8));
                requirements.insert(ResourceType::GreenVegetation, (1, 5));
            },
            SpeciesType::Frogs => {
                requirements.insert(ResourceType::O2, (2, 7));
                requirements.insert(ResourceType::GroundWater, (4, 9));
                requirements.insert(ResourceType::AnimalPopulation, (2, 10));
            },
            SpeciesType::Squirrels => {
                requirements.insert(ResourceType::O2, (2, 8));
                requirements.insert(ResourceType::Fruit, (2, 6));
                requirements.insert(ResourceType::GreenVegetation, (3, 8));
            },
            SpeciesType::Snails => {
                requirements.insert(ResourceType::O2, (1, 6));
                requirements.insert(ResourceType::DeadMatter, (1, 4));
                requirements.insert(ResourceType::GroundWater, (3, 8));
            },
            SpeciesType::Birds => {
                requirements.insert(ResourceType::O2, (3, 9));
                requirements.insert(ResourceType::Fruit, (2, 6));
                requirements.insert(ResourceType::AnimalPopulation, (1, 8));
            },
            
            // Fungi
            SpeciesType::RotFungi => {
                requirements.insert(ResourceType::DeadMatter, (2, 8));
                requirements.insert(ResourceType::GroundWater, (2, 7));
                requirements.insert(ResourceType::O2, (0, 4));
            },
            SpeciesType::Puffballs => {
                requirements.insert(ResourceType::DeadMatter, (1, 5));
                requirements.insert(ResourceType::SoilNutrients, (2, 6));
                requirements.insert(ResourceType::GroundWater, (1, 5));
            },
            SpeciesType::GiantMushrooms => {
                requirements.insert(ResourceType::DeadMatter, (3, 8));
                requirements.insert(ResourceType::SoilNutrients, (3, 7));
                requirements.insert(ResourceType::GroundWater, (3, 7));
                requirements.insert(ResourceType::Sunlight, (0, 3));
            },
            SpeciesType::MycorrhizalFungi => {
                requirements.insert(ResourceType::PlantPopulation, (2, 10));
                requirements.insert(ResourceType::SoilNutrients, (1, 6));
                requirements.insert(ResourceType::GroundWater, (2, 6));
            },
            SpeciesType::ShelfFungi => {
                requirements.insert(ResourceType::DeadMatter, (2, 6));
                requirements.insert(ResourceType::GreenVegetation, (3, 8));
                requirements.insert(ResourceType::GroundWater, (1, 5));
            },
            SpeciesType::MoldClusters => {
                requirements.insert(ResourceType::DeadMatter, (1, 6));
                requirements.insert(ResourceType::GroundWater, (3, 8));
                requirements.insert(ResourceType::O2, (0, 5));
            },
            SpeciesType::TruffleFungi => {
                requirements.insert(ResourceType::SoilNutrients, (4, 8));
                requirements.insert(ResourceType::GroundWater, (3, 7));
                requirements.insert(ResourceType::PlantPopulation, (3, 8));
            },
            SpeciesType::CoralFungi => {
                requirements.insert(ResourceType::DeadMatter, (2, 6));
                requirements.insert(ResourceType::SoilNutrients, (2, 5));
                requirements.insert(ResourceType::FungiPopulation, (2, 8));
            },
            SpeciesType::YeastColonies => {
                requirements.insert(ResourceType::Fruit, (1, 6));
                requirements.insert(ResourceType::GroundWater, (2, 6));
                requirements.insert(ResourceType::O2, (0, 3));
            },
            SpeciesType::SlimeMolds => {
                requirements.insert(ResourceType::DeadMatter, (3, 8));
                requirements.insert(ResourceType::GroundWater, (4, 9));
                requirements.insert(ResourceType::O2, (1, 6));
            },
        }
        requirements
    }

    pub fn daily_consumption(&self) -> HashMap<ResourceType, i32> {
        let mut consumption = HashMap::new();
        match self {
            // Plants
            SpeciesType::Grass => {
                consumption.insert(ResourceType::CO2, 1);
                consumption.insert(ResourceType::Sunlight, 1);
                consumption.insert(ResourceType::GroundWater, 1);
            },
            SpeciesType::BerryBushes => {
                consumption.insert(ResourceType::CO2, 1);
                consumption.insert(ResourceType::Sunlight, 2);
                consumption.insert(ResourceType::SoilNutrients, 2);
            },
            SpeciesType::Wildflowers => {
                consumption.insert(ResourceType::CO2, 1);
                consumption.insert(ResourceType::Sunlight, 1);
                consumption.insert(ResourceType::GroundWater, 1);
            },
            SpeciesType::OakSaplings => {
                consumption.insert(ResourceType::CO2, 2);
                consumption.insert(ResourceType::Sunlight, 2);
                consumption.insert(ResourceType::SoilNutrients, 3);
            },
            SpeciesType::Clover => {
                consumption.insert(ResourceType::CO2, 2);
                consumption.insert(ResourceType::Sunlight, 1);
                consumption.insert(ResourceType::GroundWater, 1);
            },
            SpeciesType::Ferns => {
                consumption.insert(ResourceType::CO2, 1);
                consumption.insert(ResourceType::Sunlight, 1);
                consumption.insert(ResourceType::GroundWater, 3);
            },
            SpeciesType::Sunflowers => {
                consumption.insert(ResourceType::CO2, 2);
                consumption.insert(ResourceType::Sunlight, 3);
                consumption.insert(ResourceType::SoilNutrients, 2);
            },
            SpeciesType::Moss => {
                consumption.insert(ResourceType::CO2, 1);
                consumption.insert(ResourceType::Sunlight, 1);
            },
            SpeciesType::PineTrees => {
                consumption.insert(ResourceType::CO2, 3);
                consumption.insert(ResourceType::Sunlight, 2);
                consumption.insert(ResourceType::SoilNutrients, 2);
            },
            SpeciesType::VegetablePlants => {
                consumption.insert(ResourceType::CO2, 1);
                consumption.insert(ResourceType::Sunlight, 2);
                consumption.insert(ResourceType::SoilNutrients, 3);
            },
            
            // Animals
            SpeciesType::Rabbits => {
                consumption.insert(ResourceType::O2, 1);
                consumption.insert(ResourceType::GreenVegetation, 2);
                consumption.insert(ResourceType::GroundWater, 1);
            },
            SpeciesType::Earthworms => {
                consumption.insert(ResourceType::O2, 1);
                consumption.insert(ResourceType::DeadMatter, 2);
                consumption.insert(ResourceType::GroundWater, 1);
            },
            SpeciesType::Honeybees => {
                consumption.insert(ResourceType::O2, 1);
                consumption.insert(ResourceType::Fruit, 1);
            },
            SpeciesType::FieldMice => {
                consumption.insert(ResourceType::O2, 1);
                consumption.insert(ResourceType::Fruit, 1);
                consumption.insert(ResourceType::GreenVegetation, 1);
            },
            SpeciesType::Butterflies => {
                consumption.insert(ResourceType::O2, 1);
                consumption.insert(ResourceType::Fruit, 1);
            },
            SpeciesType::Ladybugs => {
                consumption.insert(ResourceType::O2, 1);
            },
            SpeciesType::Frogs => {
                consumption.insert(ResourceType::O2, 1);
                consumption.insert(ResourceType::GroundWater, 1);
            },
            SpeciesType::Squirrels => {
                consumption.insert(ResourceType::O2, 1);
                consumption.insert(ResourceType::Fruit, 2);
            },
            SpeciesType::Snails => {
                consumption.insert(ResourceType::O2, 1);
                consumption.insert(ResourceType::DeadMatter, 1);
                consumption.insert(ResourceType::GroundWater, 2);
            },
            SpeciesType::Birds => {
                consumption.insert(ResourceType::O2, 1);
                consumption.insert(ResourceType::Fruit, 1);
            },
            
            // Fungi
            SpeciesType::RotFungi => {
                consumption.insert(ResourceType::DeadMatter, 2);
                consumption.insert(ResourceType::GroundWater, 1);
            },
            SpeciesType::Puffballs => {
                consumption.insert(ResourceType::DeadMatter, 1);
                consumption.insert(ResourceType::SoilNutrients, 1);
            },
            SpeciesType::GiantMushrooms => {
                consumption.insert(ResourceType::DeadMatter, 3);
                consumption.insert(ResourceType::SoilNutrients, 2);
            },
            SpeciesType::MycorrhizalFungi => {
                consumption.insert(ResourceType::SoilNutrients, 1);
                consumption.insert(ResourceType::GroundWater, 1);
            },
            SpeciesType::ShelfFungi => {
                consumption.insert(ResourceType::DeadMatter, 1);
                consumption.insert(ResourceType::GreenVegetation, 1);
            },
            SpeciesType::MoldClusters => {
                consumption.insert(ResourceType::DeadMatter, 1);
                consumption.insert(ResourceType::GroundWater, 1);
            },
            SpeciesType::TruffleFungi => {
                consumption.insert(ResourceType::SoilNutrients, 2);
                consumption.insert(ResourceType::GroundWater, 1);
            },
            SpeciesType::CoralFungi => {
                consumption.insert(ResourceType::DeadMatter, 1);
                consumption.insert(ResourceType::SoilNutrients, 1);
            },
            SpeciesType::YeastColonies => {
                consumption.insert(ResourceType::Fruit, 1);
                consumption.insert(ResourceType::GroundWater, 1);
            },
            SpeciesType::SlimeMolds => {
                consumption.insert(ResourceType::DeadMatter, 2);
                consumption.insert(ResourceType::GroundWater, 2);
            },
        }
        consumption
    }

    pub fn daily_production(&self) -> HashMap<ResourceType, i32> {
        let mut production = HashMap::new();
        match self {
            // Plants
            SpeciesType::Grass => {
                production.insert(ResourceType::O2, 2);
                production.insert(ResourceType::GreenVegetation, 1);
            },
            SpeciesType::BerryBushes => {
                production.insert(ResourceType::O2, 1);
                production.insert(ResourceType::Fruit, 2);
                production.insert(ResourceType::GreenVegetation, 1);
            },
            SpeciesType::Wildflowers => {
                production.insert(ResourceType::O2, 1);
                production.insert(ResourceType::GreenVegetation, 1);
            },
            SpeciesType::OakSaplings => {
                production.insert(ResourceType::O2, 4);
                production.insert(ResourceType::GreenVegetation, 2);
            },
            SpeciesType::Clover => {
                production.insert(ResourceType::O2, 2);
                production.insert(ResourceType::SoilNutrients, 2);
                production.insert(ResourceType::GreenVegetation, 1);
            },
            SpeciesType::Ferns => {
                production.insert(ResourceType::O2, 1);
                production.insert(ResourceType::GreenVegetation, 1);
            },
            SpeciesType::Sunflowers => {
                production.insert(ResourceType::O2, 3);
                production.insert(ResourceType::GreenVegetation, 1);
                production.insert(ResourceType::Fruit, 2);
            },
            SpeciesType::Moss => {
                production.insert(ResourceType::O2, 1);
            },
            SpeciesType::PineTrees => {
                production.insert(ResourceType::O2, 4);
                production.insert(ResourceType::GreenVegetation, 1);
            },
            SpeciesType::VegetablePlants => {
                production.insert(ResourceType::O2, 1);
                production.insert(ResourceType::Fruit, 3);
            },
            
            // Animals
            SpeciesType::Rabbits => {
                production.insert(ResourceType::CO2, 1);
                production.insert(ResourceType::DeadMatter, 1);
            },
            SpeciesType::Earthworms => {
                production.insert(ResourceType::CO2, 1);
                production.insert(ResourceType::SoilNutrients, 3);
            },
            SpeciesType::Honeybees => {
                production.insert(ResourceType::CO2, 1);
            },
            SpeciesType::FieldMice => {
                production.insert(ResourceType::CO2, 1);
                production.insert(ResourceType::DeadMatter, 1);
            },
            SpeciesType::Butterflies => {
                production.insert(ResourceType::CO2, 1);
            },
            SpeciesType::Ladybugs => {
                production.insert(ResourceType::CO2, 1);
            },
            SpeciesType::Frogs => {
                production.insert(ResourceType::CO2, 2);
                production.insert(ResourceType::DeadMatter, 2);
            },
            SpeciesType::Squirrels => {
                production.insert(ResourceType::CO2, 1);
                production.insert(ResourceType::DeadMatter, 1);
            },
            SpeciesType::Snails => {
                production.insert(ResourceType::CO2, 1);
                production.insert(ResourceType::SoilNutrients, 1);
            },
            SpeciesType::Birds => {
                production.insert(ResourceType::CO2, 1);
                production.insert(ResourceType::DeadMatter, 1);
            },
            
            // Fungi
            SpeciesType::RotFungi => {
                production.insert(ResourceType::CO2, 1);
                production.insert(ResourceType::SoilNutrients, 2);
            },
            SpeciesType::Puffballs => {
                production.insert(ResourceType::CO2, 1);
                production.insert(ResourceType::SoilNutrients, 1);
            },
            SpeciesType::GiantMushrooms => {
                production.insert(ResourceType::CO2, 2);
                production.insert(ResourceType::SoilNutrients, 4);
                production.insert(ResourceType::Fruit, 1);
            },
            SpeciesType::MycorrhizalFungi => {
                production.insert(ResourceType::CO2, 1);
            },
            SpeciesType::ShelfFungi => {
                production.insert(ResourceType::CO2, 1);
                production.insert(ResourceType::SoilNutrients, 2);
            },
            SpeciesType::MoldClusters => {
                production.insert(ResourceType::CO2, 1);
                production.insert(ResourceType::SoilNutrients, 2);
            },
            SpeciesType::TruffleFungi => {
                production.insert(ResourceType::CO2, 1);
                production.insert(ResourceType::SoilNutrients, 1);
                production.insert(ResourceType::Fruit, 2);
            },
            SpeciesType::CoralFungi => {
                production.insert(ResourceType::CO2, 1);
                production.insert(ResourceType::SoilNutrients, 2);
            },
            SpeciesType::YeastColonies => {
                production.insert(ResourceType::CO2, 2);
                production.insert(ResourceType::SoilNutrients, 1);
            },
            SpeciesType::SlimeMolds => {
                production.insert(ResourceType::CO2, 1);
                production.insert(ResourceType::SoilNutrients, 3);
            },
        }
        production
    }

    pub fn color(&self) -> Color {
        match self {
            // Plants
            SpeciesType::Grass => Color::srgb(0.4, 0.7, 0.3),
            SpeciesType::BerryBushes => Color::srgb(0.6, 0.3, 0.7),
            SpeciesType::Wildflowers => Color::srgb(0.9, 0.7, 0.2),
            SpeciesType::OakSaplings => Color::srgb(0.2, 0.5, 0.2),
            SpeciesType::Clover => Color::srgb(0.3, 0.8, 0.3),
            SpeciesType::Ferns => Color::srgb(0.2, 0.6, 0.4),
            SpeciesType::Sunflowers => Color::srgb(1.0, 0.8, 0.0),
            SpeciesType::Moss => Color::srgb(0.3, 0.4, 0.2),
            SpeciesType::PineTrees => Color::srgb(0.1, 0.4, 0.2),
            SpeciesType::VegetablePlants => Color::srgb(0.5, 0.7, 0.3),
            
            // Animals
            SpeciesType::Rabbits => Color::srgb(0.6, 0.5, 0.4),
            SpeciesType::Earthworms => Color::srgb(0.5, 0.3, 0.2),
            SpeciesType::Honeybees => Color::srgb(1.0, 0.8, 0.2),
            SpeciesType::FieldMice => Color::srgb(0.4, 0.3, 0.2),
            SpeciesType::Butterflies => Color::srgb(0.8, 0.6, 0.9),
            SpeciesType::Ladybugs => Color::srgb(0.8, 0.2, 0.2),
            SpeciesType::Frogs => Color::srgb(0.2, 0.6, 0.3),
            SpeciesType::Squirrels => Color::srgb(0.6, 0.4, 0.2),
            SpeciesType::Snails => Color::srgb(0.7, 0.6, 0.5),
            SpeciesType::Birds => Color::srgb(0.4, 0.6, 0.8),
            
            // Fungi
            SpeciesType::RotFungi => Color::srgb(0.4, 0.3, 0.2),
            SpeciesType::Puffballs => Color::srgb(0.8, 0.8, 0.7),
            SpeciesType::GiantMushrooms => Color::srgb(0.6, 0.4, 0.3),
            SpeciesType::MycorrhizalFungi => Color::srgb(0.5, 0.4, 0.3),
            SpeciesType::ShelfFungi => Color::srgb(0.7, 0.5, 0.3),
            SpeciesType::MoldClusters => Color::srgb(0.3, 0.5, 0.3),
            SpeciesType::TruffleFungi => Color::srgb(0.3, 0.2, 0.1),
            SpeciesType::CoralFungi => Color::srgb(0.9, 0.7, 0.5),
            SpeciesType::YeastColonies => Color::srgb(0.7, 0.7, 0.6),
            SpeciesType::SlimeMolds => Color::srgb(0.6, 0.8, 0.4),
        }
    }

    pub fn all() -> Vec<SpeciesType> {
        vec![
            // Plants
            SpeciesType::Grass,
            SpeciesType::BerryBushes,
            SpeciesType::Wildflowers,
            SpeciesType::OakSaplings,
            SpeciesType::Clover,
            SpeciesType::Ferns,
            SpeciesType::Sunflowers,
            SpeciesType::Moss,
            SpeciesType::PineTrees,
            SpeciesType::VegetablePlants,
            
            // Animals
            SpeciesType::Rabbits,
            SpeciesType::Earthworms,
            SpeciesType::Honeybees,
            SpeciesType::FieldMice,
            SpeciesType::Butterflies,
            SpeciesType::Ladybugs,
            SpeciesType::Frogs,
            SpeciesType::Squirrels,
            SpeciesType::Snails,
            SpeciesType::Birds,
            
            // Fungi
            SpeciesType::RotFungi,
            SpeciesType::Puffballs,
            SpeciesType::GiantMushrooms,
            SpeciesType::MycorrhizalFungi,
            SpeciesType::ShelfFungi,
            SpeciesType::MoldClusters,
            SpeciesType::TruffleFungi,
            SpeciesType::CoralFungi,
            SpeciesType::YeastColonies,
            SpeciesType::SlimeMolds,
        ]
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CardType {
    Species(SpeciesType),
}

impl CardType {
    pub fn name(&self) -> &'static str {
        match self {
            CardType::Species(species) => species.name(),
        }
    }
    
    pub fn survival_requirements(&self) -> HashMap<ResourceType, (i32, i32)> {
        match self {
            CardType::Species(species) => species.survival_requirements(),
        }
    }
    
    pub fn daily_consumption(&self) -> HashMap<ResourceType, i32> {
        match self {
            CardType::Species(species) => species.daily_consumption(),
        }
    }
    
    pub fn daily_production(&self) -> HashMap<ResourceType, i32> {
        match self {
            CardType::Species(species) => species.daily_production(),
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            CardType::Species(species) => species.color(),
        }
    }

    pub fn unlock_round(&self) -> u32 {
        match self {
            CardType::Species(species) => species.unlock_round(),
        }
    }

    pub fn max_population(&self) -> u32 {
        match self {
            CardType::Species(species) => species.max_population(),
        }
    }
}
