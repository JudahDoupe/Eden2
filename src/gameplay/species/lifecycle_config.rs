use std::collections::HashMap;
use crate::gameplay::lifecycle::MatterType;

/// Requirements for growth phase
#[derive(Clone, Debug)]
pub struct GrowthRequirements {
    /// Minimum feeding threshold to enable growth
    pub minimum_feeding_threshold: f32,
    /// Environmental conditions needed for growth
    pub environmental_factors: HashMap<MatterType, (i32, i32)>,
    /// Age at which creature can mature (days)
    pub minimum_age: u32,
}

/// Requirements for feeding phase
#[derive(Clone, Debug)]
pub struct FeedingRequirements {
    /// Base matter/resource requirements per feeding cycle
    pub base_requirements: HashMap<MatterType, u32>,
    /// Multiplier for mature creatures (adults may need more food)
    pub maturity_multiplier: f32,
    /// Minimum percentage of needs that must be met to survive
    pub minimum_threshold: f32,
    /// What matter type this species converts input to when feeding
    pub biomass_conversion: BiomassConversion,
}

/// How feeding input converts to creature biomass
#[derive(Clone, Debug)]
pub enum BiomassConversion {
    /// Plants: nutrients -> plant matter biomass
    PlantGrowth { efficiency: f32 },
    /// Herbivores: plant matter -> animal matter biomass  
    PlantToAnimal { efficiency: f32 },
    /// Carnivores: animal matter -> animal matter biomass
    AnimalToAnimal { efficiency: f32 },
    /// Decomposers: dead matter -> soil nutrients + own biomass
    Decomposition { 
        nutrient_output: f32,
        biomass_gain: f32,
        matter_type: MatterType, // What type of biomass the decomposer gains
    },
}

/// Requirements for reproduction
#[derive(Clone, Debug)]
pub struct ReproductionRequirements {
    /// Minimum feeding threshold to enable reproduction
    pub minimum_feeding_threshold: f32,
    /// Environmental conditions needed for reproduction
    pub environmental_requirements: HashMap<MatterType, (i32, i32)>,
    /// Population constraints
    pub population_requirements: PopulationRequirement,
    /// How much biomass parent loses to create offspring
    pub biomass_cost: u32,
    /// How much biomass offspring starts with
    pub offspring_biomass: u32,
    /// Probability of successful reproduction (0.0 to 1.0)
    pub reproduction_probability: f32,
    /// Days that must pass between reproduction attempts
    pub cooldown_days: u32,
}

/// Population requirements for reproduction
#[derive(Clone, Debug)]
pub enum PopulationRequirement {
    None,
    MinimumPopulation(u32),
    RequiresPair, // Needs at least 2 mature individuals
    MaximumDensity(u32), // Won't reproduce if population exceeds this
}

/// Mortality factors that determine when creatures die
#[derive(Clone, Debug)]
pub struct MortalityFactors {
    /// Natural lifespan range (min_days, max_days)
    pub natural_lifespan: (u32, u32),
    /// Days without food before death
    pub starvation_tolerance: u32,
    /// Days in bad environmental conditions before death
    pub environmental_tolerance: u32,
}

/// What biomass type this species consists of
#[derive(Clone, Debug)]
pub enum BiomassComposition {
    /// Pure plant matter (plants, fungi)
    Plant,
    /// Pure animal matter (animals)
    Animal,
    /// Mixed composition (rare, for special species)
    Mixed { plant_ratio: f32, animal_ratio: f32 },
}

impl Default for FeedingRequirements {
    fn default() -> Self {
        Self {
            base_requirements: HashMap::new(),
            maturity_multiplier: 1.0,
            minimum_threshold: 0.5,
            biomass_conversion: BiomassConversion::PlantGrowth { efficiency: 1.0 },
        }
    }
}

impl Default for GrowthRequirements {
    fn default() -> Self {
        Self {
            minimum_feeding_threshold: 0.7,
            environmental_factors: HashMap::new(),
            minimum_age: 1,
        }
    }
}

impl Default for ReproductionRequirements {
    fn default() -> Self {
        Self {
            minimum_feeding_threshold: 0.8,
            environmental_requirements: HashMap::new(),
            population_requirements: PopulationRequirement::None,
            biomass_cost: 1,
            offspring_biomass: 1,
            reproduction_probability: 0.1,
            cooldown_days: 7,
        }
    }
}

impl Default for MortalityFactors {
    fn default() -> Self {
        Self {
            natural_lifespan: (30, 60),
            starvation_tolerance: 3,
            environmental_tolerance: 10,
        }
    }
}
