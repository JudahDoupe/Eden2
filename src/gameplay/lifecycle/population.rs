use bevy::prelude::*;
use std::collections::HashMap;
use super::{IndividualCreature, CreatureId, DeathCause, EcosystemMatter, MatterType};
use crate::gameplay::species::Species;

/// Main resource for tracking the entire ecosystem population
/// Manages individual creatures and ecosystem-wide statistics
#[derive(Resource, Clone, Debug)]
pub struct EcosystemPopulation {
    pub creatures: Vec<IndividualCreature>,
    pub next_creature_id: CreatureId,
    pub current_day: u32,
    
    // Population statistics
    pub living_population_by_species: HashMap<String, u32>,
    pub dead_population_by_species: HashMap<String, u32>,
    pub daily_births: HashMap<String, u32>,
    pub daily_deaths: HashMap<String, Vec<DeathCause>>,
    
    // Matter tracking
    pub ecosystem_matter: EcosystemMatter,
}

impl Default for EcosystemPopulation {
    fn default() -> Self {
        Self {
            creatures: Vec::new(),
            next_creature_id: 1,
            current_day: 0,
            living_population_by_species: HashMap::new(),
            dead_population_by_species: HashMap::new(),
            daily_births: HashMap::new(),
            daily_deaths: HashMap::new(),
            ecosystem_matter: EcosystemMatter::default(),
        }
    }
}

impl EcosystemPopulation {
    /// Add a new creature to the ecosystem
    pub fn add_creature(&mut self, species: Species, initial_biomass_amounts: (u32, u32)) -> CreatureId {
        let creature_id = self.next_creature_id;
        self.next_creature_id += 1;
        
        let introduction_order = self.creatures
            .iter()
            .filter(|c| c.introduction_day == self.current_day)
            .count() as u32;
        
        let biomass = super::CreatureBiomass::new(initial_biomass_amounts.0, initial_biomass_amounts.1);
        let creature = IndividualCreature::new(
            creature_id,
            species.clone(),
            biomass,
            self.current_day,
            introduction_order,
        );
        
        // Update population statistics
        let species_name = species.name.to_string();
        *self.living_population_by_species.entry(species_name.clone()).or_insert(0) += 1;
        *self.daily_births.entry(species_name).or_insert(0) += 1;
        
        self.creatures.push(creature);
        creature_id
    }

    /// Add a mature creature (for pairs, breeding stock, etc.)
    pub fn add_mature_creature(&mut self, species: Species, initial_biomass_amounts: (u32, u32)) -> CreatureId {
        let creature_id = self.add_creature(species, initial_biomass_amounts);
        
        // Find the creature and make it mature
        if let Some(creature) = self.creatures.iter_mut().find(|c| c.id == creature_id) {
            creature.maturity_stage = super::MaturityStage::Mature;
        }
        
        creature_id
    }

    /// Get all living creatures
    pub fn living_creatures(&self) -> impl Iterator<Item = &IndividualCreature> {
        self.creatures.iter().filter(|c| c.is_alive())
    }

    /// Get mutable references to all living creatures
    pub fn living_creatures_mut(&mut self) -> impl Iterator<Item = &mut IndividualCreature> {
        self.creatures.iter_mut().filter(|c| c.is_alive())
    }

    /// Get creatures by species name
    pub fn creatures_by_species<'a>(&'a self, species_name: &'a str) -> impl Iterator<Item = &'a IndividualCreature> {
        self.creatures.iter().filter(move |c| c.species.name == species_name && c.is_alive())
    }

    /// Get population count for a species
    pub fn population_count(&self, species_name: &str) -> u32 {
        self.living_population_by_species.get(species_name).copied().unwrap_or(0)
    }

    /// Get total living population
    pub fn total_population(&self) -> u32 {
        self.living_population_by_species.values().sum()
    }

    /// Get population by kingdom
    pub fn population_by_kingdom(&self) -> (u32, u32, u32) {
        let mut plant_pop = 0;
        let mut animal_pop = 0;
        let mut fungi_pop = 0;

        for creature in self.living_creatures() {
            match creature.species.kingdom {
                crate::gameplay::species::Kingdom::Plant => plant_pop += 1,
                crate::gameplay::species::Kingdom::Animal => animal_pop += 1,
                crate::gameplay::species::Kingdom::Fungi => fungi_pop += 1,
            }
        }

        (plant_pop, animal_pop, fungi_pop)
    }

    /// Calculate total living biomass by matter type
    pub fn total_living_biomass(&self) -> HashMap<MatterType, u32> {
        let mut totals = HashMap::new();
        
        for creature in self.living_creatures() {
            let plant_matter = totals.entry(MatterType::PlantMatter).or_insert(0);
            *plant_matter += creature.biomass.plant_matter;
            
            let animal_matter = totals.entry(MatterType::AnimalMatter).or_insert(0);
            *animal_matter += creature.biomass.animal_matter;
        }
        
        totals
    }

    /// Remove dead creatures and convert their biomass to dead matter
    pub fn remove_dead_creatures(&mut self) -> Vec<(IndividualCreature, DeathCause)> {
        let mut removed = Vec::new();
        
        // Find dead creatures and process their matter
        let mut to_remove = Vec::new();
        for (index, creature) in self.creatures.iter().enumerate() {
            if !creature.is_alive() {
                // Convert biomass to dead matter
                let (dead_matter_type, amount) = creature.get_death_matter();
                if amount > 0 {
                    self.ecosystem_matter.add_matter(dead_matter_type, amount);
                }
                
                // Update statistics
                let species_name = creature.species.name.to_string();
                *self.living_population_by_species.entry(species_name.clone()).or_insert(0) = 
                    self.living_population_by_species.get(&species_name).unwrap_or(&0).saturating_sub(1);
                *self.dead_population_by_species.entry(species_name.clone()).or_insert(0) += 1;
                
                to_remove.push(index);
                removed.push((creature.clone(), DeathCause::NaturalAge)); // Default cause for now
            }
        }
        
        // Remove in reverse order to maintain indices
        for &index in to_remove.iter().rev() {
            self.creatures.remove(index);
        }
        
        removed
    }

    /// Advance to next day
    pub fn advance_day(&mut self) {
        self.current_day += 1;
        
        // Clear daily statistics
        self.daily_births.clear();
        self.daily_deaths.clear();
        
        // Age all living creatures
        for creature in self.living_creatures_mut() {
            creature.age_one_day();
        }
        
        // Apply daily environmental inputs
        self.ecosystem_matter.apply_daily_inputs();
    }

    /// Get creatures sorted by trophic level and introduction order
    /// This is crucial for proper feeding phase processing
    pub fn creatures_by_trophic_order(&self) -> Vec<&IndividualCreature> {
        let mut creatures: Vec<&IndividualCreature> = self.living_creatures().collect();
        
        // Sort by trophic level first, then by introduction order
        creatures.sort_by(|a, b| {
            let trophic_a = self.get_trophic_level(a);
            let trophic_b = self.get_trophic_level(b);
            
            trophic_a.cmp(&trophic_b)
                .then_with(|| a.introduction_day.cmp(&b.introduction_day))
                .then_with(|| a.introduction_order.cmp(&b.introduction_order))
        });
        
        creatures
    }

    /// Get trophic level for feeding order (lower numbers feed first)
    fn get_trophic_level(&self, creature: &IndividualCreature) -> u8 {
        match creature.species.kingdom {
            // Decomposers feed first
            crate::gameplay::species::Kingdom::Fungi => 0,
            // TODO: Distinguish decomposer animals (earthworms) from regular animals
            // For now, treat all animals as consumers
            crate::gameplay::species::Kingdom::Animal => {
                // Check if this is a decomposer animal (earthworm, etc.)
                if creature.species.name.contains("Earthworm") || creature.species.name.contains("Worm") {
                    0  // Decomposer
                } else {
                    2  // Consumer (herbivore/carnivore determined by what they eat)
                }
            },
            // Plants are primary producers
            crate::gameplay::species::Kingdom::Plant => 1,
        }
    }

    /// Get ecosystem summary for debugging/display
    pub fn get_ecosystem_summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();
        
        summary.insert("Total Population".to_string(), self.total_population().to_string());
        summary.insert("Current Day".to_string(), self.current_day.to_string());
        summary.insert("Next Creature ID".to_string(), self.next_creature_id.to_string());
        
        let (plants, animals, fungi) = self.population_by_kingdom();
        summary.insert("Plants".to_string(), plants.to_string());
        summary.insert("Animals".to_string(), animals.to_string());
        summary.insert("Fungi".to_string(), fungi.to_string());
        
        // Add matter summary
        for (matter_name, amount) in self.ecosystem_matter.get_matter_summary() {
            summary.insert(format!("Matter: {}", matter_name), amount.to_string());
        }
        
        summary
    }
}

// ===== EVENT HANDLERS =====

/// Handle adding species to the ecosystem
pub fn handle_add_species_to_ecosystem_event(
    mut ecosystem: ResMut<EcosystemPopulation>,
    mut add_species_events: EventReader<super::AddSpeciesToEcosystemEvent>,
) {
    for event in add_species_events.read() {
        println!("Adding {} to ecosystem with biomass ({}, {})", 
                 event.species.name, 
                 event.starting_biomass.0, 
                 event.starting_biomass.1);
        
        ecosystem.add_creature(
            event.species.clone(), 
            event.starting_biomass
        );
    }
}

/// Handle daily simulation events
pub fn handle_simulate_day_event(
    mut ecosystem: ResMut<EcosystemPopulation>,
    mut simulate_events: EventReader<super::SimulateDayEvent>,
) {
    for _event in simulate_events.read() {
        let daily_simulation = super::DailySimulation::new();
        
        match daily_simulation.simulate_day(&mut ecosystem) {
            Ok(result) => {
                let summary = result.get_summary();
                println!("Day {} simulation completed: {} births, {} deaths, {} total creatures", 
                         summary.day, 
                         summary.births,
                         summary.deaths,
                         summary.final_population);
            }
            Err(error) => {
                eprintln!("Day {} simulation failed: {}", ecosystem.current_day, error);
            }
        }
    }
}
