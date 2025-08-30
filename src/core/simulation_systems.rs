use bevy::prelude::*;
use crate::types::{SpeciesType, ResourceType, Kingdom};
use crate::ui::components::{Garden, GardenResources, Species, DailyConsumption, DailyProduction, SurvivalRequirements};

/// Event for when a species is added to the garden
#[derive(Event)]
pub struct AddSpeciesEvent {
    pub species_type: SpeciesType,
}

/// Event to trigger a simulation update (when cards are played)
#[derive(Event)]
pub struct TriggerSimulationEvent;

/// System to spawn the garden entity during startup
pub fn spawn_garden(mut commands: Commands) {
    commands.spawn((
        Garden,
        GardenResources::default(),
        Transform::default(),
        Visibility::default(),
    ));
}

/// System to handle adding new species to the garden
pub fn handle_add_species(
    mut commands: Commands,
    mut add_species_events: EventReader<AddSpeciesEvent>,
    garden_query: Query<&GardenResources, With<Garden>>,
) {
    let garden_resources = match garden_query.single() {
        Ok(resources) => resources,
        Err(_) => return, // No garden entity found
    };

    for event in add_species_events.read() {
        let species_type = event.species_type;
        
        // Check if garden can afford this species
        let consumption = species_type.daily_consumption();
        if !garden_resources.can_afford(&consumption) {
            println!("Insufficient resources to add species: {}", species_type.name());
            continue;
        }

        // Don't pay consumption cost upfront - let the simulation system handle it
        // This allows species to survive their first turn

        // Create the species entity with all required components
        let production = species_type.daily_production();
        let survival_requirements = species_type.survival_requirements();

        commands.spawn((
            Species {
                species_type,
                population: 1,
            },
            DailyConsumption { consumption },
            DailyProduction { production },
            SurvivalRequirements {
                requirements: survival_requirements,
            },
            Transform::default(),
            Visibility::default(),
        ));

        println!("Added species: {} to garden", species_type.name());
    }
}

/// System to trigger simulation when a card is played
/// This runs first and sends a single trigger event per card play
pub fn trigger_simulation_on_card_play(
    mut trigger_events: EventWriter<TriggerSimulationEvent>,
    mut add_species_events: EventReader<AddSpeciesEvent>,
) {
    // Only trigger simulation once even if multiple cards are played in the same frame
    let mut should_trigger = false;
    for _ in add_species_events.read() {
        should_trigger = true;
    }
    
    if should_trigger {
        trigger_events.write(TriggerSimulationEvent);
        println!("=== Starting Day Simulation ===");
    }
}

/// System that runs the complete daily simulation cycle
/// This handles all simulation steps in the correct order for one day
pub fn run_daily_simulation(
    mut commands: Commands,
    mut garden_query: Query<&mut GardenResources, With<Garden>>,
    mut species_query: Query<(Entity, &mut Species, &DailyConsumption, &DailyProduction, &SurvivalRequirements)>,
    mut trigger_events: EventReader<TriggerSimulationEvent>,
) {
    // Only run if there's a trigger event
    if trigger_events.read().next().is_none() {
        return;
    }

    let mut garden_resources = match garden_query.single_mut() {
        Ok(resources) => resources,
        Err(_) => return, // No garden entity found
    };

    println!("=== Running Daily Simulation ===");

    // Step 1: Reset renewable resources for the new day
    for resource_type in ResourceType::all() {
        if resource_type.is_renewable() {
            garden_resources.resources.insert(resource_type, resource_type.daily_renewable_amount());
        }
    }

    // Step 2: Reset produced resources to base levels
    let base_resources = GardenResources::default();
    for (resource_type, base_amount) in &base_resources.resources {
        if matches!(
            resource_type,
            ResourceType::GreenVegetation | 
            ResourceType::Fruit | 
            ResourceType::DeadMatter
        ) {
            garden_resources.resources.insert(*resource_type, *base_amount);
        }
    }

    // Step 3: Apply all species consumption and production
    for (_, species, consumption, production, _) in species_query.iter() {
        let population_multiplier = species.population as i32;

        // Apply consumption (negative effect)
        for (resource_type, amount) in &consumption.consumption {
            let total_consumption = amount * population_multiplier;
            garden_resources.modify_resource(*resource_type, -total_consumption);
        }

        // Apply production (positive effect)
        for (resource_type, amount) in &production.production {
            let total_production = amount * population_multiplier;
            garden_resources.modify_resource(*resource_type, total_production);
        }
    }

    // Step 4: Update population counters
    let mut plant_pop = 0;
    let mut animal_pop = 0;
    let mut fungi_pop = 0;

    for (_, species, _, _, _) in species_query.iter() {
        match species.species_type.kingdom() {
            Kingdom::Plant => plant_pop += species.population,
            Kingdom::Animal => animal_pop += species.population,
            Kingdom::Fungi => fungi_pop += species.population,
        }
    }

    garden_resources.resources.insert(ResourceType::PlantPopulation, plant_pop as i32);
    garden_resources.resources.insert(ResourceType::AnimalPopulation, animal_pop as i32);
    garden_resources.resources.insert(ResourceType::FungiPopulation, fungi_pop as i32);

    // Step 5: Check survival and handle population changes
    let mut species_to_remove = Vec::new();

    for (entity, mut species, _, _, survival_requirements) in species_query.iter_mut() {
        let mut can_survive = true;
        let mut failure_reasons = Vec::new();

        // Check if current garden state meets survival requirements
        for (resource_type, (min, max)) in &survival_requirements.requirements {
            let current_amount = garden_resources.get_resource(*resource_type);
            if current_amount < *min {
                can_survive = false;
                failure_reasons.push(format!(
                    "insufficient {} (need at least {}, had {})",
                    resource_type.name().to_lowercase(),
                    min,
                    current_amount
                ));
            } else if current_amount > *max {
                can_survive = false;
                failure_reasons.push(format!(
                    "too much {} (max {}, had {})",
                    resource_type.name().to_lowercase(),
                    max,
                    current_amount
                ));
            }
        }

        if !can_survive {
            // Species can't survive - reduce population or remove
            let failure_reason = failure_reasons.join(", ");
            
            if species.population > 1 {
                species.population -= 1;
                println!(
                    "{} population decreased to {} due to: {}",
                    species.species_type.name(),
                    species.population,
                    failure_reason
                );
            } else {
                species_to_remove.push(entity);
                println!(
                    "{} has died out due to: {}",
                    species.species_type.name(),
                    failure_reason
                );
            }
        } else if species.population < species.species_type.max_population() {
            // Species can thrive - potentially grow
            if rand::random::<f32>() < 0.1 { // 10% chance per day
                species.population += 1;
                println!(
                    "{} population increased to {}",
                    species.species_type.name(),
                    species.population
                );
            }
        }
    }

    // Remove extinct species
    for entity in species_to_remove {
        commands.entity(entity).despawn();
    }

    // Step 6: Print end-of-day summary
    println!("End of Day - Water: {}, Sunlight: {}, Nutrients: {}", 
        garden_resources.get_resource(ResourceType::GroundWater),
        garden_resources.get_resource(ResourceType::Sunlight),
        garden_resources.get_resource(ResourceType::SoilNutrients)
    );
    println!("Species count: {}", species_query.iter().count());
    println!("=========================");
}
