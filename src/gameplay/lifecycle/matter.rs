use bevy::prelude::*;
use std::collections::HashMap;

/// Extended matter types for the ecosystem
/// Includes both living biomass and environmental resources
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MatterType {
    // Living biomass (in creatures)
    PlantMatter,
    AnimalMatter,
    
    // Dead matter (available for decomposition)
    DeadPlantMatter,
    DeadAnimalMatter,
    
    // Environmental resources
    SoilNutrients,
    Sunlight,
    GroundWater,
    CO2,
    O2,
}

impl MatterType {
    pub fn name(&self) -> &'static str {
        match self {
            MatterType::PlantMatter => "Plant Matter",
            MatterType::AnimalMatter => "Animal Matter",
            MatterType::DeadPlantMatter => "Dead Plant Matter",
            MatterType::DeadAnimalMatter => "Dead Animal Matter",
            MatterType::SoilNutrients => "Soil Nutrients",
            MatterType::Sunlight => "Sunlight",
            MatterType::GroundWater => "Ground Water",
            MatterType::CO2 => "CO2",
            MatterType::O2 => "O2",
        }
    }

    pub fn all() -> Vec<MatterType> {
        vec![
            MatterType::PlantMatter,
            MatterType::AnimalMatter,
            MatterType::DeadPlantMatter,
            MatterType::DeadAnimalMatter,
            MatterType::SoilNutrients,
            MatterType::Sunlight,
            MatterType::GroundWater,
            MatterType::CO2,
            MatterType::O2,
        ]
    }

    /// Check if this matter type represents living biomass
    pub fn is_living_biomass(&self) -> bool {
        matches!(self, MatterType::PlantMatter | MatterType::AnimalMatter)
    }

    /// Check if this matter type represents dead matter
    pub fn is_dead_matter(&self) -> bool {
        matches!(self, MatterType::DeadPlantMatter | MatterType::DeadAnimalMatter)
    }

    /// Check if this is an environmental resource
    pub fn is_environmental_resource(&self) -> bool {
        matches!(self, 
            MatterType::SoilNutrients | 
            MatterType::Sunlight | 
            MatterType::GroundWater | 
            MatterType::CO2 | 
            MatterType::O2
        )
    }
}

/// Error type for matter conservation violations
#[derive(Debug)]
pub struct MatterConservationError {
    pub expected_total: u32,
    pub actual_total: u32,
    pub phase: String,
    pub details: String,
}

impl std::fmt::Display for MatterConservationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matter conservation violation in {}: expected {}, got {} ({})",
            self.phase, self.expected_total, self.actual_total, self.details
        )
    }
}

impl std::error::Error for MatterConservationError {}

/// Tracks all matter pools in the ecosystem
/// This includes environmental resources and dead matter available for consumption
#[derive(Clone, Debug, Resource)]
pub struct EcosystemMatter {
    pub matter_pools: HashMap<MatterType, u32>,
}

impl Default for EcosystemMatter {
    fn default() -> Self {
        let mut matter_pools = HashMap::new();
        
        // Initialize with starting environmental resources
        matter_pools.insert(MatterType::SoilNutrients, 30);
        matter_pools.insert(MatterType::Sunlight, 100);  // Renewable daily
        matter_pools.insert(MatterType::GroundWater, 50); // Renewable daily
        matter_pools.insert(MatterType::CO2, 50);
        matter_pools.insert(MatterType::O2, 50);
        
        // No dead matter initially
        matter_pools.insert(MatterType::DeadPlantMatter, 0);
        matter_pools.insert(MatterType::DeadAnimalMatter, 0);
        
        // Living biomass is tracked in creatures, not here
        matter_pools.insert(MatterType::PlantMatter, 0);
        matter_pools.insert(MatterType::AnimalMatter, 0);
        
        Self { matter_pools }
    }
}

impl EcosystemMatter {
    /// Get amount of a specific matter type
    pub fn get_amount(&self, matter_type: MatterType) -> u32 {
        self.matter_pools.get(&matter_type).copied().unwrap_or(0)
    }

    /// Set amount of a specific matter type
    pub fn set_amount(&mut self, matter_type: MatterType, amount: u32) {
        self.matter_pools.insert(matter_type, amount);
    }

    /// Add matter to a pool
    pub fn add_matter(&mut self, matter_type: MatterType, amount: u32) {
        let current = self.get_amount(matter_type);
        self.set_amount(matter_type, current + amount);
    }

    /// Remove matter from a pool, returns actual amount removed
    pub fn consume_matter(&mut self, matter_type: MatterType, amount: u32) -> u32 {
        let current = self.get_amount(matter_type);
        let consumed = amount.min(current);
        self.set_amount(matter_type, current - consumed);
        consumed
    }

    /// Check if enough matter is available
    pub fn can_consume(&self, matter_type: MatterType, amount: u32) -> bool {
        self.get_amount(matter_type) >= amount
    }

    /// Apply daily environmental inputs
    pub fn apply_daily_inputs(&mut self) {
        // Renewable resources get daily replenishment
        self.set_amount(MatterType::Sunlight, 100);  // Full daily sunlight
        self.add_matter(MatterType::GroundWater, 10); // Daily precipitation
        self.add_matter(MatterType::CO2, 5);          // Atmospheric exchange
        self.add_matter(MatterType::O2, 5);           // Atmospheric exchange
    }

    /// Calculate total environmental matter (excluding living biomass)
    pub fn total_environmental_matter(&self) -> u32 {
        MatterType::all()
            .iter()
            .filter(|mt| mt.is_environmental_resource() || mt.is_dead_matter())
            .map(|mt| self.get_amount(*mt))
            .sum()
    }

    /// Validate matter conservation between two states
    pub fn validate_conservation(
        &self,
        previous: &EcosystemMatter,
        external_inputs: u32,
        expected_losses: u32,
        phase: &str,
    ) -> Result<(), MatterConservationError> {
        let previous_total = previous.total_environmental_matter();
        let current_total = self.total_environmental_matter();
        let expected_total = previous_total + external_inputs - expected_losses;

        if current_total != expected_total {
            return Err(MatterConservationError {
                expected_total,
                actual_total: current_total,
                phase: phase.to_string(),
                details: format!(
                    "Previous: {}, Inputs: {}, Losses: {}", 
                    previous_total, external_inputs, expected_losses
                ),
            });
        }

        Ok(())
    }

    /// Get matter breakdown for debugging
    pub fn get_matter_summary(&self) -> HashMap<String, u32> {
        let mut summary = HashMap::new();
        
        for matter_type in MatterType::all() {
            let amount = self.get_amount(matter_type);
            if amount > 0 {
                summary.insert(matter_type.name().to_string(), amount);
            }
        }
        
        summary
    }
}
