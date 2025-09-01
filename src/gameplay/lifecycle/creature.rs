use crate::gameplay::species::Species;
use super::matter::MatterType;

/// Unique identifier for individual creatures
pub type CreatureId = u64;

/// Maturity stages of a creature's lifecycle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaturityStage {
    Juvenile,
    Mature,
    Dead,
}

/// Result of a creature's feeding attempt
#[derive(Clone, Debug)]
pub enum FeedingResult {
    FullyFed,
    PartiallyFed(f32), // percentage of needs met (0.0 to 1.0)
    Starving,
}

impl FeedingResult {
    /// Returns the feeding satisfaction level (0.0 to 1.0)
    pub fn satisfaction_level(&self) -> f32 {
        match self {
            FeedingResult::FullyFed => 1.0,
            FeedingResult::PartiallyFed(level) => *level,
            FeedingResult::Starving => 0.0,
        }
    }

    /// Checks if the creature is fed enough to survive
    pub fn is_fed(&self) -> bool {
        self.satisfaction_level() > 0.0
    }

    /// Checks if the creature is well-fed enough for growth/reproduction
    pub fn is_well_fed(&self, threshold: f32) -> bool {
        self.satisfaction_level() >= threshold
    }
}

/// Causes of creature death for statistics tracking
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeathCause {
    NaturalAge,
    Starvation,
    EnvironmentalStress,
}

/// Biomass composition of an individual creature
/// Tracks the specific amounts of different matter types in the creature
#[derive(Clone, Debug)]
pub struct CreatureBiomass {
    pub plant_matter: u32,
    pub animal_matter: u32,
}

impl CreatureBiomass {
    /// Create a new biomass composition
    pub fn new(plant_matter: u32, animal_matter: u32) -> Self {
        Self {
            plant_matter,
            animal_matter,
        }
    }

    /// Get total biomass across all matter types
    pub fn total(&self) -> u32 {
        self.plant_matter + self.animal_matter
    }

    /// Check if creature has any biomass
    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }

    /// Add biomass of a specific type
    pub fn add_matter(&mut self, matter_type: MatterType, amount: u32) {
        match matter_type {
            MatterType::PlantMatter => self.plant_matter += amount,
            MatterType::AnimalMatter => self.animal_matter += amount,
            _ => {
                // Other matter types can't be added to living creatures
                println!("Warning: Attempted to add non-biomass matter type {:?} to creature", matter_type);
            }
        }
    }

    /// Remove biomass of a specific type, returns actual amount removed
    pub fn remove_matter(&mut self, matter_type: MatterType, amount: u32) -> u32 {
        match matter_type {
            MatterType::PlantMatter => {
                let removed = amount.min(self.plant_matter);
                self.plant_matter -= removed;
                removed
            },
            MatterType::AnimalMatter => {
                let removed = amount.min(self.animal_matter);
                self.animal_matter -= removed;
                removed
            },
            _ => {
                println!("Warning: Attempted to remove non-biomass matter type {:?} from creature", matter_type);
                0
            }
        }
    }

    /// Get amount of a specific matter type
    pub fn get_matter(&self, matter_type: MatterType) -> u32 {
        match matter_type {
            MatterType::PlantMatter => self.plant_matter,
            MatterType::AnimalMatter => self.animal_matter,
            _ => 0,
        }
    }
}

/// Individual creature with full lifecycle tracking
/// Each creature is a unique entity with its own state and biomass
#[derive(Clone, Debug)]
pub struct IndividualCreature {
    pub id: CreatureId,
    pub species: Species,
    pub maturity_stage: MaturityStage,
    pub age_days: u32,
    pub days_since_last_fed: u32,
    pub days_in_bad_environment: u32,
    pub last_reproduction_day: Option<u32>,
    pub fed_status: FeedingResult,
    pub biomass: CreatureBiomass,
    
    // Tracking for ecosystem statistics
    pub introduction_day: u32,  // When creature was added to ecosystem
    pub introduction_order: u32, // Order within that day for deterministic competition
}

impl IndividualCreature {
    /// Create a new juvenile creature
    pub fn new(
        id: CreatureId,
        species: Species,
        initial_biomass: CreatureBiomass,
        introduction_day: u32,
        introduction_order: u32,
    ) -> Self {
        Self {
            id,
            species,
            maturity_stage: MaturityStage::Juvenile,
            age_days: 0,
            days_since_last_fed: 0,
            days_in_bad_environment: 0,
            last_reproduction_day: None,
            fed_status: FeedingResult::Starving,
            biomass: initial_biomass,
            introduction_day,
            introduction_order,
        }
    }

    /// Create a new mature creature (for breeding pairs, etc.)
    pub fn new_mature(
        id: CreatureId,
        species: Species,
        initial_biomass: CreatureBiomass,
        introduction_day: u32,
        introduction_order: u32,
    ) -> Self {
        let mut creature = Self::new(id, species, initial_biomass, introduction_day, introduction_order);
        creature.maturity_stage = MaturityStage::Mature;
        creature
    }

    /// Check if creature is alive
    pub fn is_alive(&self) -> bool {
        self.maturity_stage != MaturityStage::Dead
    }

    /// Check if creature is mature
    pub fn is_mature(&self) -> bool {
        self.maturity_stage == MaturityStage::Mature
    }

    /// Check if creature is juvenile
    pub fn is_juvenile(&self) -> bool {
        self.maturity_stage == MaturityStage::Juvenile
    }

    /// Advance creature age by one day
    pub fn age_one_day(&mut self) {
        if self.is_alive() {
            self.age_days += 1;
            
            // Update feeding counter
            if !self.fed_status.is_fed() {
                self.days_since_last_fed += 1;
            } else {
                self.days_since_last_fed = 0;
            }
        }
    }

    /// Mark creature as fed with specific result
    pub fn set_fed_status(&mut self, result: FeedingResult) {
        if result.is_fed() {
            self.days_since_last_fed = 0;
        }
        self.fed_status = result;
    }

    /// Attempt to mature from juvenile to adult
    pub fn attempt_growth(&mut self) -> bool {
        if self.is_juvenile() && self.fed_status.is_well_fed(0.7) {
            self.maturity_stage = MaturityStage::Mature;
            true
        } else {
            false
        }
    }

    /// Mark creature as dead with cause
    pub fn die(&mut self, _cause: DeathCause) {
        self.maturity_stage = MaturityStage::Dead;
    }

    /// Get the creature's biomass as dead matter when it dies
    pub fn get_death_matter(&self) -> (MatterType, u32) {
        match self.species.kingdom {
            crate::gameplay::species::Kingdom::Plant | 
            crate::gameplay::species::Kingdom::Fungi => (MatterType::DeadPlantMatter, self.biomass.plant_matter),
            crate::gameplay::species::Kingdom::Animal => (MatterType::DeadAnimalMatter, self.biomass.animal_matter),
        }
    }
}
