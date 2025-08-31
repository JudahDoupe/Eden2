use bevy::prelude::*;
use crate::gameplay::cards::Card;
use super::resources::{GardenResources, ResourceType};
use crate::gameplay::species::{SpeciesCollection, AddSpeciesEvent, log_species_consumption, log_species_production, log_species_deaths};
use std::collections::HashMap;

// ===== COMPONENTS AND RESOURCES =====

/// Core garden state with resources and species
#[derive(Resource)]
pub struct GardenState {
    pub resources: GardenResources,
    pub species: SpeciesCollection,
}

impl Default for GardenState {
    fn default() -> Self {
        Self {
            resources: GardenResources::default(),
            species: SpeciesCollection::new(),
        }
    }
}

impl GardenState {
    pub fn get_resource(&self, resource_type: ResourceType) -> i32 {
        self.resources.get_resource(resource_type)
    }
    
    pub fn modify_resource(&mut self, resource_type: ResourceType, change: i32) {
        self.resources.modify_resource(resource_type, change);
    }

    pub fn can_afford(&self, requirements: &HashMap<ResourceType, i32>) -> bool {
        self.resources.can_afford(requirements)
    }

    pub fn add_species(&mut self, species: Card) -> bool {
        // Check if garden can afford this species
        if !self.species.can_afford_species(&species, &self.resources) {
            println!("Insufficient resources to add species: {}", species.name());
            return false;
        }

        // Add the species
        self.species.add_species(species.clone(), 1);

        println!("Added species: {} to garden", species.name());
        true
    }

    /// Update population counts in resources based on current species
    fn update_population_counts(&mut self) {
        let (plant_pop, animal_pop, fungi_pop) = self.species.population_by_kingdom();
        self.resources.resources.insert(ResourceType::PlantPopulation, plant_pop as i32);
        self.resources.resources.insert(ResourceType::AnimalPopulation, animal_pop as i32);
        self.resources.resources.insert(ResourceType::FungiPopulation, fungi_pop as i32);
    }

    /// Run a complete daily simulation cycle
    pub fn run_daily_simulation(&mut self) {
        println!("=== Running Daily Garden Simulation ===");
        
        // Step 1: Calculate and apply resource changes
        let resource_changes = self.species.net_resource_changes();
        
        // Log what happened
        log_species_consumption(&self.species);
        log_species_production(&self.species);
        
        // Apply all resource changes
        self.resources.apply_resource_changes(resource_changes);
        
        // Step 2: Check survival requirements and remove dead species
        let dead_species = self.species.remove_dead_species(&self.resources);
        
        // Log deaths
        log_species_deaths(&dead_species, &self.resources);
        
        // Step 3: Update population counts
        self.update_population_counts();
        
        // Print final state
        self.resources.print_resources();
        println!("Active species: {}", self.species.total_count());
    }
}

// ===== EVENTS =====

/// Event for when a species is selected to be added to the garden
#[derive(Event)]
pub struct PlayCardEvent {
    pub hand_index: usize,
}

/// Event to trigger a simulation update (when species are added)
#[derive(Event)]
pub struct SimulateDayEvent;

// ===== SYSTEMS =====

/// Handles species play events by validating resources and applying effects
pub fn handle_species_play(
    mut game_state: ResMut<crate::gameplay::GameState>,
    mut species_play_events: EventReader<PlayCardEvent>,
    mut add_species_events: EventWriter<AddSpeciesEvent>,
) {
    for event in species_play_events.read() {
        if let Some(played_species) = try_play_species(&mut game_state, &mut add_species_events, event.hand_index) {
            println!("Successfully played: {}", played_species.name());
        }
    }
}

/// Attempts to play a species from the hand, returns the played species if successful
fn try_play_species(
    game_state: &mut crate::gameplay::GameState,
    add_species_events: &mut EventWriter<AddSpeciesEvent>,
    hand_index: usize,
) -> Option<Card> {
    // Validate hand index
    if hand_index >= game_state.hand.len() {
        return None;
    }

    let species = game_state.hand.get_card(hand_index)?.clone();
    
    // Send event to add species - the simulation system will handle resource validation
    add_species_events.write(AddSpeciesEvent { species: species.clone() });
    
    // Remove species from hand regardless - the add species system will handle validation
    game_state.play_species(hand_index)
}

/// System to handle adding new species to the garden
pub fn handle_add_species(
    mut garden_state: ResMut<GardenState>,
    mut add_species_events: EventReader<AddSpeciesEvent>,
    mut trigger_events: EventWriter<SimulateDayEvent>,
) {
    let mut should_trigger_simulation = false;
    
    for event in add_species_events.read() {
        if garden_state.add_species(event.species.clone()) {
            should_trigger_simulation = true;
        }
    }
    
    if should_trigger_simulation {
        trigger_events.write(SimulateDayEvent);
    }
}

/// System to trigger simulation when a species is added
/// This runs first and sends a single trigger event per species addition
pub fn trigger_simulation_on_species_play(
    mut trigger_events: EventWriter<SimulateDayEvent>,
    mut add_species_events: EventReader<AddSpeciesEvent>,
) {
    // Only trigger simulation once even if multiple species are added in the same frame
    let mut should_trigger = false;
    for _ in add_species_events.read() {
        should_trigger = true;
    }
    
    if should_trigger {
        trigger_events.write(SimulateDayEvent);
        println!("=== Starting Day Simulation ===");
    }
}

/// System that runs the complete daily simulation cycle
pub fn run_daily_simulation(
    mut garden_state: ResMut<GardenState>,
    mut trigger_events: EventReader<SimulateDayEvent>,
) {
    // Only run if there's a trigger event
    if trigger_events.read().next().is_none() {
        return;
    }

    garden_state.run_daily_simulation();
}
