# ECS-Based Garden Simulation Refactor

## Overview

The garden simulation has been refactored from a simple resource-based system to a full Entity-Component-System (ECS) architecture using Bevy's ECS capabilities. This provides better separation of concerns, reusable components, and more flexible simulation logic.

## Architecture

### Entities

1. **Garden Entity**: A single entity that represents the garden itself and holds the overall resource state
2. **Species Entities**: Each species in the garden is its own entity with multiple components defining its behavior

### Components

#### Garden Components
- `Garden`: Marker component identifying the garden entity
- `GardenResources`: Contains the HashMap of all garden resources (water, sunlight, nutrients, etc.)

#### Species Components
- `Species`: Core species data (species type, population)
- `DailyConsumption`: HashMap of resources consumed per day by this species
- `DailyProduction`: HashMap of resources produced per day by this species
- `SurvivalRequirements`: HashMap of resource ranges (min, max) required for survival

### Systems

#### Core Simulation Systems

1. **`spawn_garden`** (Startup): Creates the initial garden entity with default resources
2. **`handle_add_species`** (Update): Handles `AddSpeciesEvent` events to create new species entities
3. **`update_garden_simulation`** (FixedUpdate): Recalculates garden resources based on all species
4. **`update_species_survival`** (FixedUpdate): Checks survival requirements and handles growth/death
5. **`run_simulation_timer`** (Update): Provides debug output on a timer

#### Integration Systems

- **`handle_card_play`**: Modified to send `AddSpeciesEvent` instead of directly modifying garden state
- **`update_resource_display`**: Updated to read from the garden entity's `GardenResources` component
- **`update_species_display`**: Updated to query all species entities

### Benefits of the New System

1. **Modularity**: Each species is its own entity with composable components
2. **Reusability**: Components like `DailyConsumption` and `DailyProduction` can be easily reused
3. **Flexibility**: Easy to add new behaviors by adding new components
4. **Performance**: Bevy's ECS provides optimized queries and parallel execution
5. **Extensibility**: Easy to add new species effects, environmental factors, or complex interactions

### Resource Flow

1. **Card Play**: Player plays a species card → `CardPlayEvent` → `AddSpeciesEvent`
2. **Species Creation**: `handle_add_species` checks resources and creates species entity with components
3. **Simulation Update**: `update_garden_simulation` recalculates garden resources from all species
4. **Survival Check**: `update_species_survival` checks if species can survive in current conditions
5. **UI Update**: Display systems read current state and update UI

### Example Species Entity

When a Grass card is played, the following entity is created:

```rust
commands.spawn((
    Species {
        species_type: SpeciesType::Grass,
        population: 1,
    },
    DailyConsumption { 
        consumption: HashMap::from([
            (ResourceType::Sunlight, 2),
            (ResourceType::GroundWater, 1),
            (ResourceType::SoilNutrients, 1),
            (ResourceType::CO2, 1),
        ])
    },
    DailyProduction { 
        production: HashMap::from([
            (ResourceType::GreenVegetation, 2),
            (ResourceType::O2, 1),
        ])
    },
    SurvivalRequirements {
        requirements: HashMap::from([
            (ResourceType::Sunlight, (2, 8)),
            (ResourceType::GroundWater, (1, 6)),
            (ResourceType::SoilNutrients, (1, 5)),
            (ResourceType::CO2, (1, 10)),
        ])
    },
));
```

### Future Enhancements

The new ECS architecture makes it easy to add:

- **Seasonal Effects**: Add a `SeasonalModifier` component
- **Disease Systems**: Add `Disease` and `Immunity` components  
- **Age/Lifecycle**: Add `Age` component with lifecycle stages
- **Symbiotic Relationships**: Add `SymbioticPartner` component
- **Environmental Events**: Add event-driven weather, disasters, etc.
- **Complex Interactions**: Species that affect each other beyond just resource competition

## Migration Notes

The old `GardenState` resource is deprecated in favor of the ECS approach. The UI systems have been updated to work with the new entity queries instead of the old resource-based approach.
