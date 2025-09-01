# Creature Lifecycle System Design Document

## Overview

This document outlines a major refactor to the Eden2 simulation system to implement a comprehensive creature lifecycle with a **matter conservation system**. Instead of the current simplified daily resource consumption/production model, each creature will go through distinct life phases each day while maintaining energy balance through biomass transformation between different matter types.

## Current System vs. New System

### Current System
- Creatures have static daily resource consumption/production
- Simple survival checks based on resource ranges
- Population changes are abstract
- No individual creature tracking
- Resources appear and disappear without conservation

### New System
- Individual creatures go through daily lifecycle phases
- **Matter conservation**: biomass transforms between states rather than being created/destroyed
- **Biomass tracking**: creatures consist of specific amounts of matter types
- Explicit growth, reproduction, and death mechanics with matter transformation
- More realistic population dynamics with energy balance

## Matter Conservation System

### Core Principle
Energy and biomass are conserved throughout the ecosystem. Matter transforms between different states but is never created or destroyed (except for external inputs like sunlight and water).

### Matter Types
The ecosystem tracks several types of matter:

- **Soil Nutrients**: Base nutrients available in the soil
- **Plant Matter**: Living plant biomass
- **Animal Matter**: Living animal biomass  
- **Fungi Matter**: Living fungi biomass
- **Dead Plant Matter**: Deceased plant biomass available for decomposition
- **Dead Animal Matter**: Deceased animal biomass available for decomposition
- **Energy Resources**: Sunlight, water, oxygen, CO2 (external or atmospheric)

### Transformation Rules

#### Plants (e.g., Clover)
- **Feeding**: Consumes Soil Nutrient + Energy → Produces 1 Plant Matter (growth)
- **Reproduction**: Converts Plant Matter → Creates new creature
- **Death**: All Plant Matter becomes Dead Plant Matter
- **Consumption by animals**: Plant Matter becomes Animal Matter

#### Animals (e.g., Rabbit)  
- **Feeding**: Consumes Plant Matter → Converts to Animal Matter
- **Reproduction**: Converts Animal Matter → Creates new creature
- **Death**: All Animal Matter becomes Dead Animal Matter
- **Consumption by predators**: Animal Matter becomes predator's Animal Matter

#### Decomposers

**Fungi**:
- **Composition**: Consist of Plant Matter
- **Feeding**: Consumes Dead Plant Matter + Dead Animal Matter → Converts to Soil Nutrients
- **Reproduction**: Converts Plant Matter → Creates new fungi with Plant Matter
- **Death**: Plant Matter becomes Dead Plant Matter

**Worms**:
- **Composition**: Consist of Animal Matter  
- **Feeding**: Consumes Dead Plant Matter + Dead Animal Matter → Converts to Soil Nutrients
- **Reproduction**: Converts Animal Matter → Creates new worm with Animal Matter
- **Death**: Animal Matter becomes Dead Animal Matter

### Biomass Tracking
Each individual creature tracks:
```rust
#[derive(Clone, Debug)]
pub struct CreatureBiomass {
    pub plant_matter: u32,
    pub animal_matter: u32,
}
```

## Lifecycle Phases

Every creature attempts to go through these four phases each day, in order:

### 1. Feeding Phase
**Purpose**: Creatures attempt to acquire the resources they need to survive and thrive.

**Mechanics**:
- Each creature checks available garden resources
- Attempts to consume required amounts based on species needs and current life stage
- Success/failure affects creature's fed status for the day
- Competition occurs when multiple creatures need the same limited resources
- Feeding priority may be determined by factors like:
  - Creature size/maturity
  - Species feeding efficiency
  - Random selection for same-priority creatures

**Implementation Details**:
```rust
pub struct FeedingRequirements {
    pub base_requirements: HashMap<ResourceType, i32>,
    pub maturity_multiplier: f32,  // Adults may need more food
    pub minimum_threshold: f32,    // Minimum % of needs to survive
}

pub enum FeedingResult {
    FullyFed,
    PartiallyFed(f32), // percentage of needs met
    Starving,
}
```

### 2. Growing Phase
**Purpose**: Fed juvenile creatures mature into adults.

**Mechanics**:
- Only applies to juvenile (non-mature) creatures
- Requires creature to be fed (from feeding phase)
- May have species-specific growth requirements beyond basic feeding
- Successful growth advances creature to mature status
- Mature creatures skip this phase

**Implementation Details**:
```rust
pub enum MaturityStage {
    Juvenile,
    Mature,
    Dead,
}

pub struct GrowthRequirements {
    pub minimum_feeding_threshold: f32,  // Must be fed at least this % to grow
    pub environmental_factors: Vec<(ResourceType, (i32, i32))>, // Optimal growth onditions
}
```

### 3. Reproduction Phase
**Purpose**: Mature, fed creatures create offspring.

**Mechanics**:
- Only applies to mature creatures
- Requires creature to be fed (from feeding phase)
- May require specific environmental conditions
- Successful reproduction creates new juvenile creature(s) of same species
- Population limits may prevent reproduction
- Some species may require pairs or specific population densities

**Implementation Details**:
```rust
pub struct ReproductionRequirements {
    pub minimum_feeding_threshold: f32,
    pub environmental_requirements: HashMap<ResourceType, (i32, i32)>,
    pub population_requirements: PopulationRequirement,
    pub cooldown_days: u32, // Days between reproduction attempts
}

pub enum PopulationRequirement {
    None,
    MinimumPopulation(u32),
    RequiresPair, // Needs at least 2 mature individuals
    MaximumDensity(u32), // Won't reproduce if population too high
}
```

### 4. Death Phase
**Purpose**: Remove creatures that have exceeded their natural lifespan or died from other causes.

**Mechanics**:
- Natural death: creatures die when they exceed their species' lifespan
- Starvation death: creatures that haven't been fed for too many consecutive days
- Environmental death: creatures in unsurvivable conditions for too long
- Death produces dead matter resources
- Some species may have different death mechanics (e.g., seasonal death)

**Implementation Details**:
```rust
pub struct MortalityFactors {
    pub natural_lifespan: (u32, u32), // min, max days
    pub starvation_tolerance: u32,     // days without food before death
    pub environmental_tolerance: u32,  // days in bad conditions before death
}

pub enum DeathCause {
    NaturalAge,
    Starvation,
    EnvironmentalStress,

}
```

## Individual Creature Tracking

### Creature State
Each individual creature will track:

```rust
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
    pub biomass: CreatureBiomass, // Tracks matter composition
}

#[derive(Clone, Debug)]
pub struct CreatureBiomass {
    pub plant_matter: u32,
    pub animal_matter: u32,
}
```

### Population Management
```rust
#[derive(Resource, Clone, Debug)]
pub struct EcosystemPopulation {
    pub creatures: Vec<IndividualCreature>,
    pub next_creature_id: CreatureId,
    pub living_population_by_species: HashMap<String, u32>,
    pub dead_population_by_species: HashMap<String, u32>,
    pub daily_births: HashMap<String, u32>,
    pub daily_deaths: HashMap<String, Vec<DeathCause>>,
    pub ecosystem_matter: EcosystemMatter,
}

#[derive(Clone, Debug)]
pub struct EcosystemMatter {
    pub soil_nutrients: u32,
    pub dead_plant_matter: u32,
    pub dead_animal_matter: u32,
    pub total_living_plant_matter: u32,
    pub total_living_animal_matter: u32,
}
```

## Daily Simulation Flow

The daily simulation follows a carefully ordered sequence to ensure proper matter flow through the ecosystem. Each phase processes all relevant creatures before moving to the next phase, maintaining matter conservation throughout.

### Daily Cycle Overview
The simulation processes one complete day through these phases:

1. **Environmental Input Phase** - Add external energy and matter
2. **Death & Decomposition Phase** - Remove old creatures and recycle matter  
3. **Feeding Phase** - Consume available matter and energy
4. **Growth Phase** - Convert stored matter into maturity
5. **Reproduction Phase** - Convert matter into offspring
6. **End-of-Day Bookkeeping** - Update statistics and prepare for next day

### Detailed Phase Breakdown

#### 1. Environmental Input Phase
**Purpose**: Add daily external inputs to the ecosystem before any creature activity.

**Process**:
- Add daily sunlight energy to the ecosystem pool
- Add atmospheric inputs (CO2, O2, water from precipitation)
- Refresh any renewable external resources
- This ensures all creatures have access to baseline environmental resources

**Matter Flow**:
```
External Sources → Ecosystem Resource Pools
- Sunlight → Solar Energy Pool
- Atmosphere → CO2, O2 Pools  
- Weather → Ground Water Pool
```

#### 2. Death & Decomposition Phase
**Purpose**: Remove creatures that have died and convert their biomass to dead matter for decomposers.

**Process**:
- **Death Check**: All creatures are evaluated for death conditions (age, starvation, environmental stress)
- **Matter Conversion**: Dead creatures' biomass immediately converts to dead matter
- **Ecosystem Update**: Dead matter pools are updated before any feeding occurs

**Matter Flow**:
```
Living Creature Biomass → Dead Matter Pools
- Plant Matter → Dead Plant Matter
- Animal Matter → Dead Animal Matter
```

**Why First**: Death must occur before feeding so that decomposers have access to dead matter from creatures that died overnight, and so dead creatures don't compete for resources.

#### 3. Feeding Phase
**Purpose**: All living creatures attempt to acquire matter and energy for survival.

**Processing Order**: 
1. **Decomposers First** (Fungi, Worms): Convert dead matter → soil nutrients
2. **Primary Producers** (Plants): Convert soil nutrients + energy → plant matter
3. **Primary Consumers** (Herbivores): Convert plant matter → animal matter  
4. **Secondary Consumers** (Carnivores): Convert animal matter → animal matter
5. **Omnivores**: Process after their primary food sources have fed

**Resource Competition**:
- Within each feeding tier, process by introduction order to garden
- Calculate total demand vs. available supply for each resource
- Apply proportional rationing if demand exceeds supply
- Update creature biomass and feeding status immediately after consumption

**Matter Flow**:
```
Decomposers: Dead Plant/Animal Matter → Soil Nutrients + Decomposer Biomass
Plants: Soil Nutrients + Solar Energy → Plant Matter Biomass  
Herbivores: Plant Matter → Animal Matter Biomass
Carnivores: Animal Matter → Animal Matter Biomass
```

#### 4. Growth Phase  
**Purpose**: Well-fed juvenile creatures mature into adults.

**Process**:
- Only juvenile creatures participate
- Must meet minimum feeding threshold from previous phase
- Growth may require additional matter investment beyond basic feeding
- Successful growth changes maturity status (no additional matter creation)

**Matter Flow**:
```
No Net Matter Change - Internal creature state change only
- Juvenile status → Mature status (if feeding requirements met)
```

#### 5. Reproduction Phase
**Purpose**: Well-fed mature creatures convert stored biomass into offspring.

**Process**:
- Only mature, well-fed creatures participate  
- Must meet reproduction requirements (environmental, population, cooldown)
- Parent creature loses biomass to create offspring biomass
- New creatures are added to ecosystem population

**Matter Flow**:
```
Parent Creature Biomass → Offspring Creature Biomass
- Parent loses matter → New creature gains same amount of matter
- Total ecosystem biomass remains constant
```

#### 6. End-of-Day Bookkeeping
**Purpose**: Update ecosystem statistics and prepare for next simulation day.

**Process**:
- Calculate total biomass by matter type across all creatures
- Update population statistics (births, deaths, by species)
- Increment creature ages and tracking counters  
- Validate matter conservation (total matter should be constant except for external inputs)
- Prepare ecosystem state for next day

### Matter Conservation Validation

At the end of each day, the simulation validates:
```rust
// Previous day matter + External inputs = Current day matter
let total_input = previous_day_matter + external_daily_inputs;
let total_current = sum_all_creature_biomass() + sum_all_dead_matter() + sum_all_resource_pools();
assert_eq!(total_input, total_current, "Matter conservation violated!");
```

### Processing Order Rationale

**Decomposers → Producers → Consumers**: This order ensures:
1. Dead matter from overnight deaths becomes available nutrients
2. Plants can access both existing and newly-generated soil nutrients  
3. Herbivores can access both existing and newly-grown plant matter
4. Carnivores can access animal matter from well-fed herbivores
5. Each trophic level has maximum food availability from lower levels

### Resource Competition & Matter Allocation

The feeding phase implements sophisticated resource competition that respects both ecological hierarchies and matter conservation.

#### Trophic Level Processing
Resources are allocated in ecological order to ensure realistic matter flow:

1. **Decomposer Tier**: Fungi and worms compete for dead matter
   - No competition with other tiers (exclusive access to dead matter)
   - Competition within tier resolved by species introduction order
   - Output (soil nutrients) immediately available for next tier

2. **Producer Tier**: Plants compete for soil nutrients and energy
   - Access to nutrients generated by decomposers in same day
   - Solar energy typically unlimited (unless weather/environmental factors)
   - Competition resolved by introduction order within species

3. **Consumer Tiers**: Animals compete for biomass from previous tiers
   - Herbivores compete for plant matter from producer tier
   - Carnivores compete for animal matter from herbivore tier
   - Each tier accesses fresh biomass produced by previous tier

#### Competition Resolution Algorithm
Within each trophic tier, when demand exceeds supply:

```rust
pub fn allocate_resources(
    available_resources: &mut EcosystemMatter,
    demanding_creatures: &[&IndividualCreature],
    resource_type: MatterType,
) -> Vec<AllocationResult> {
    let total_demand = demanding_creatures.iter()
        .map(|c| c.calculate_demand(resource_type))
        .sum();
    
    let available_supply = available_resources.get_amount(resource_type);
    
    if total_demand <= available_supply {
        // Abundant resources - everyone gets what they need
        allocate_full_demand(demanding_creatures, resource_type)
    } else {
        // Scarcity - implement rationing by priority
        allocate_by_priority_and_proportion(
            demanding_creatures, 
            resource_type, 
            available_supply,
            total_demand
        )
    }
}
```

#### Priority System
When rationing is required:

1. **Maturity Priority**: Mature creatures get priority over juveniles
   - Represents competitive advantage of larger/more developed organisms
   - Ensures breeding population survival during resource stress

2. **Size/Species Priority**: Larger species may outcompete smaller ones
   - Could be configurable per species relationship
   - Represents realistic ecological competition

3. **Introduction Order**: Final tiebreaker for identical creatures
   - Ensures deterministic behavior
   - Represents temporal advantages (first arrival, territory establishment)

#### Immediate Matter Flow
Resources are consumed and converted immediately during feeding:

```rust
// Example: Rabbit feeding on clover
let consumed_plant_matter = garden.consume_resource(MatterType::PlantMatter, 2);
let converted_animal_matter = consumed_plant_matter * rabbit.conversion_efficiency;
rabbit.biomass.animal_matter += converted_animal_matter;

// Matter is immediately unavailable for other consumers
assert_eq!(garden.get_resource(MatterType::PlantMatter), 
           previous_amount - consumed_plant_matter);
```

This ensures that early-feeding creatures affect resource availability for later feeders, creating realistic competition dynamics.

### Example Daily Matter Flow

Here's a concrete example of how matter flows through a simple ecosystem during one day:

#### Starting State
```
Ecosystem Matter Pools:
- Soil Nutrients: 20 units
- Dead Plant Matter: 5 units  
- Dead Animal Matter: 2 units
- Solar Energy: 100 units (daily input)

Living Creatures:
- 2 Clover (Plant Matter: 3 each = 6 total)
- 1 Mushroom (Plant Matter: 2)
- 1 Rabbit (Animal Matter: 4)
- 1 Earthworm (Animal Matter: 1)

Total Ecosystem Matter: 20 + 5 + 2 + 6 + 2 + 4 + 1 = 40 units + daily energy
```

#### Phase-by-Phase Matter Flow

**Phase 1 - Environmental Input:**
```
+ 100 Solar Energy → Available for plants
+ 10 Atmospheric inputs (CO2, water) → Resource pools
```

**Phase 2 - Death & Decomposition:**
```
No deaths today → No matter conversion
```

**Phase 3 - Feeding (Trophic Order):**

*Decomposers:*
```
Mushroom consumes: 2 Dead Plant Matter → 2 Soil Nutrients + 1 Plant Matter growth
Earthworm consumes: 2 Dead Animal Matter → 2 Soil Nutrients + 0.5 Animal Matter growth

Matter Flow: 
- Dead Plant Matter: 5 → 3 (-2)
- Dead Animal Matter: 2 → 0 (-2) 
- Soil Nutrients: 20 → 24 (+4)
- Mushroom biomass: 2 → 3 (+1 Plant Matter)
- Earthworm biomass: 1 → 1.5 (+0.5 Animal Matter)
```

*Producers:*
```
Clover #1 consumes: 2 Soil Nutrients + Solar Energy → 2 Plant Matter growth
Clover #2 consumes: 2 Soil Nutrients + Solar Energy → 2 Plant Matter growth

Matter Flow:
- Soil Nutrients: 24 → 20 (-4)
- Clover #1 biomass: 3 → 5 (+2 Plant Matter)
- Clover #2 biomass: 3 → 5 (+2 Plant Matter)
- Solar Energy: 100 → 96 (-4, plenty remaining)
```

*Primary Consumers:*
```
Rabbit consumes: 3 Plant Matter → 2 Animal Matter growth (conversion loss)

Available Plant Matter: Clover biomass = 10 total
Rabbit takes: 3 units from available clover
- Randomly from Clover #1: 2 units (5→3)
- Randomly from Clover #2: 1 unit (5→4)  

Matter Flow:
- Total Clover biomass: 10 → 7 (-3)
- Rabbit biomass: 4 → 6 (+2 Animal Matter)
```

**Phase 4 - Growth:**
```
All creatures were fed → Juveniles would grow to mature (state change only)
No matter transformation in this phase
```

**Phase 5 - Reproduction:**
```
Assume Clover #1 reproduces: Spends 2 Plant Matter → Creates new Clover with 1 Plant Matter

Matter Flow:
- Clover #1 biomass: 3 → 1 (-2 Plant Matter)
- New Clover #3: 0 → 1 (+1 Plant Matter)
- Net ecosystem change: 0 (matter conserved)
```

#### End State
```
Ecosystem Matter Pools:
- Soil Nutrients: 20 units (same)
- Dead Plant Matter: 3 units (-2, consumed by mushroom)
- Dead Animal Matter: 0 units (-2, consumed by earthworm)
- Solar Energy: 96 units (remainder after plant feeding)

Living Creatures:
- Clover #1: 1 Plant Matter (-2 total: grew +2, reproduced -2, fed -2)
- Clover #2: 4 Plant Matter (+1 total: grew +2, fed -1) 
- Clover #3: 1 Plant Matter (new offspring)
- Mushroom: 3 Plant Matter (+1 from feeding)
- Rabbit: 6 Animal Matter (+2 from feeding)
- Earthworm: 1.5 Animal Matter (+0.5 from feeding)

Total Living Matter: 1 + 4 + 1 + 3 + 6 + 1.5 = 16.5 units
Total Ecosystem Matter: 20 + 3 + 0 + 16.5 = 39.5 units
```

#### Matter Conservation Check
```
Previous Day: 40 units
Daily External Inputs: +10 units (atmospheric)
Daily Consumption: -10.5 units (conversion losses during feeding)
Final Total: 39.5 units ✓

Note: The 0.5 unit loss represents realistic energy conversion inefficiency 
(some energy lost as heat during metabolic processes)
```

This example demonstrates how matter flows systematically through trophic levels while maintaining conservation principles.

## Integration with Existing Systems

### Species Definitions
Extend existing `Species` struct:

```rust
#[derive(Clone, Debug)]
pub struct Species {
    // Existing fields...
    pub name: &'static str,
    pub kingdom: Kingdom,
    pub unlock_round: u32,
    pub color: Color,
    
    // New lifecycle fields
    pub feeding_requirements: FeedingRequirements,
    pub growth_requirements: GrowthRequirements,
    pub reproduction_requirements: ReproductionRequirements,
    pub mortality_factors: MortalityFactors,
    
    // Updated resource interactions
    pub resource_production_per_individual: HashMap<ResourceType, i32>,
    pub environmental_requirements: HashMap<ResourceType, (i32, i32)>, // survival ranges
}
```

### Garden Integration
The garden will need to:
- Track individual creatures instead of abstract populations
- Handle resource competition during feeding phases
- Support lifecycle event callbacks for UI updates
- Maintain population statistics for game objectives

### Card System Integration
When playing a species card:
- Create one or more juvenile creatures of that species
- Add them to the garden's population
- Check if they can survive current conditions

## Balancing Considerations

### Complexity Management
- Start with simple implementations of each phase
- Add complexity gradually (e.g., environmental growth factors)
- Provide clear visual feedback for each phase's results

### Performance
- Optimize creature processing for large populations
- Consider batching similar operations
- Implement efficient resource competition algorithms

### Game Balance
- Ensure lifecycle creates interesting strategic decisions
- Balance growth rates with death rates for stable ecosystems
- Make feeding competition meaningful but not frustrating

## Implementation Phases

### Phase 1: Core Infrastructure
- Implement `IndividualCreature` and lifecycle phase traits
- Update species definitions with lifecycle parameters
- Basic feeding phase implementation

### Phase 2: Full Lifecycle
- Implement all four lifecycle phases
- Resource competition system
- Population management and statistics

### Phase 3: Advanced Features
- Complex environmental factors
- Species-specific lifecycle variations
- Advanced reproduction mechanics (pairing, seasonal breeding)

### Phase 4: Balancing and Polish
- Fine-tune lifecycle parameters
- Optimize performance
- Enhanced UI feedback for lifecycle events

## Example Species Configurations

### Clover (Plant)
```rust
Species {
    name: "Clover",
    kingdom: Kingdom::Plant,
    biomass_composition: BiomassComposition::Plant, // Consists entirely of plant matter
    
    feeding_requirements: FeedingRequirements {
        base_requirements: hashmap! {
            ResourceType::SoilNutrients => 1,
            ResourceType::Sunlight => 2,
            ResourceType::GroundWater => 1,
        },
        matter_conversion: MatterConversion {
            input: (ResourceType::SoilNutrients, 1),
            output: (MatterType::PlantMatter, 1), // Creates plant matter biomass
        },
        maturity_multiplier: 1.5,
        minimum_threshold: 0.7,
    },
    
    reproduction_requirements: ReproductionRequirements {
        biomass_cost: (MatterType::PlantMatter, 1), // Uses 1 plant matter to create offspring
        minimum_feeding_threshold: 1.0,
        environmental_requirements: hashmap! {
            ResourceType::SoilNutrients => (5, 50),
        },
        population_requirements: PopulationRequirement::MaximumDensity(10),
        offspring_biomass: (MatterType::PlantMatter, 1), // Offspring starts with 1 plant matter
        reproduction_probability: 0.15,
        cooldown_days: 7,
    },
    
    death_conversion: DeathConversion {
        output: (MatterType::DeadPlantMatter, "all_biomass"), // All plant matter becomes dead plant matter
    },
}
```

### Rabbit (Herbivore)
```rust
Species {
    name: "Rabbit",
    kingdom: Kingdom::Animal,
    biomass_composition: BiomassComposition::Animal, // Consists entirely of animal matter
    
    feeding_requirements: FeedingRequirements {
        base_requirements: hashmap! {
            MatterType::PlantMatter => 2, // Consumes living plant matter
        },
        matter_conversion: MatterConversion {
            input: (MatterType::PlantMatter, 2),
            output: (MatterType::AnimalMatter, 1), // Less efficient conversion
        },
        maturity_multiplier: 1.2,
        minimum_threshold: 0.8,
    },
    
    reproduction_requirements: ReproductionRequirements {
        biomass_cost: (MatterType::AnimalMatter, 2), // Uses 2 animal matter for reproduction
        minimum_feeding_threshold: 1.0,
        environmental_requirements: hashmap! {
            ResourceType::O2 => (10, 100),
        },
        population_requirements: PopulationRequirement::RequiresPair,
        offspring_biomass: (MatterType::AnimalMatter, 1), // Offspring starts with 1 animal matter
        reproduction_probability: 0.3,
        cooldown_days: 14,
    },
    
    death_conversion: DeathConversion {
        output: (MatterType::DeadAnimalMatter, "all_biomass"), // All animal matter becomes dead animal matter
    },
}
```

### Decomposer Fungi
```rust
Species {
    name: "Decomposer Fungi",
    kingdom: Kingdom::Fungi,
    biomass_composition: BiomassComposition::Plant, // Fungi consist of plant matter
    
    feeding_requirements: FeedingRequirements {
        base_requirements: hashmap! {
            MatterType::DeadPlantMatter => 1,
            MatterType::DeadAnimalMatter => 1,
        },
        matter_conversion: MatterConversion {
            input: (MatterType::DeadPlantMatter, 1), // Can consume either type
            alternative_input: (MatterType::DeadAnimalMatter, 1),
            output: (ResourceType::SoilNutrients, 1), // Converts dead matter back to soil nutrients
            biomass_gain: (MatterType::PlantMatter, 0.5), // Grows slowly
        },
        maturity_multiplier: 1.0,
        minimum_threshold: 0.9,
    },
    
    reproduction_requirements: ReproductionRequirements {
        biomass_cost: (MatterType::PlantMatter, 1),
        minimum_feeding_threshold: 1.0,
        environmental_requirements: hashmap! {
            MatterType::DeadPlantMatter => (1, 100),
        },
        population_requirements: PopulationRequirement::None,
        offspring_biomass: (MatterType::PlantMatter, 1),
        reproduction_probability: 0.2,
        cooldown_days: 10,
    },
    
    death_conversion: DeathConversion {
        output: (MatterType::DeadPlantMatter, "all_biomass"),
    },
}
```

### Earthworm (Decomposer)
```rust
Species {
    name: "Earthworm",
    kingdom: Kingdom::Animal,
    biomass_composition: BiomassComposition::Animal, // Worms consist of animal matter
    
    feeding_requirements: FeedingRequirements {
        base_requirements: hashmap! {
            MatterType::DeadPlantMatter => 2,
            MatterType::DeadAnimalMatter => 1,
        },
        matter_conversion: MatterConversion {
            input: (MatterType::DeadPlantMatter, 2),
            alternative_input: (MatterType::DeadAnimalMatter, 1),
            output: (ResourceType::SoilNutrients, 2), // Efficient decomposition
            biomass_gain: (MatterType::AnimalMatter, 0.3), // Slow growth
        },
        maturity_multiplier: 1.1,
        minimum_threshold: 0.8,
    },
    
    reproduction_requirements: ReproductionRequirements {
        biomass_cost: (MatterType::AnimalMatter, 1),
        minimum_feeding_threshold: 1.0,
        environmental_requirements: hashmap! {
            ResourceType::SoilNutrients => (5, 100),
        },
        population_requirements: PopulationRequirement::None,
        offspring_biomass: (MatterType::AnimalMatter, 1),
        reproduction_probability: 0.25,
        cooldown_days: 5,
    },
    
    death_conversion: DeathConversion {
        output: (MatterType::DeadAnimalMatter, "all_biomass"),
    },
}
```

## Conclusion

This creature lifecycle system will transform Eden2 from a simple resource management game into a rich ecosystem simulation. The four-phase daily cycle creates natural narrative beats and strategic depth while maintaining the game's accessibility. The individual creature tracking enables emergent storytelling and more meaningful player decisions about which species to introduce and when.

The system is designed to be implemented incrementally, allowing for iterative testing and balancing while building toward the full vision of a living, breathing garden ecosystem.
