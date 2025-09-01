# Eden2 - Garden System Gameplay

## Overview

The Garden is the central game component that represents the living ecosystem where all species interactions take place. Eden2 features a single unified garden that maintains global resource levels and tracks populations of all living species.

### Garden Responsibilities
1. **Resource Tracking**: Maintain current levels of all 6 primary resources
2. **Population Management**: Track quantity of each species currently in the garden
3. **Daily Progression**: Apply all species effects at the end of each player turn
4. **Survival Checking**: Determine which species live, die, or reproduce based on conditions
5. **Victory Validation**: Monitor whether round objectives have been met

## Resource Management

The garden maintains 6 core resources organized around fundamental ecosystem cycles. Each resource clearly represents a phase in these cycles, making it intuitive for players to understand ecosystem flow and balance.

### Primary Resources

| Resource | Description | Natural Range | Starting Value |
|----------|-------------|---------------|----------------|
| **CO₂** | Carbon dioxide available for photosynthesis | 0-20 | 10 |
| **O₂** | Oxygen available for animal respiration | 0-20 | 10 |
| **Plant Matter** | Living plant biomass and vegetation | 0-25 | 0 |
| **Animal Matter** | Living animal biomass and products | 0-20 | 0 |
| **Dead Matter** | Decomposing organic material | 0-20 | 0 |
| **Soil Nutrients** | Available soil minerals for plant growth | 0-20 | 5 |

### Resource Flow and Cycles

#### **Atmospheric Cycle** (Carbon ↔ Oxygen)
- **CO₂**: 
  - *Consumed by*: All plants during photosynthesis
  - *Produced by*: All animals and fungi during respiration
  - *Flow*: CO₂ → Plants → O₂ → Animals → CO₂

- **O₂**:
  - *Consumed by*: All animals during respiration
  - *Produced by*: All plants during photosynthesis
  - *Flow*: O₂ → Animals → CO₂ → Plants → O₂

#### **Living Matter Cycle** (Biomass → Consumption → Death)
- **Plant Matter**:
  - *Produced by*: Plants through photosynthesis and growth
  - *Consumed by*: Animals feeding on vegetation
  - *Flow*: CO₂ + Soil Nutrients → Plant Matter → Animal Matter

- **Animal Matter**:
  - *Produced by*: Animals through growth and reproduction
  - *Consumed by*: Predators and scavengers
  - *Flow*: Plant Matter → Animal Matter → Dead Matter

#### **Nutrient Cycle** (Nutrients → Growth → Death → Decomposition)
- **Soil Nutrients**:
  - *Consumed by*: Plants for growth and reproduction
  - *Produced by*: Fungi decomposing dead matter
  - *Sources*: Decomposition, nitrogen fixation, natural weathering
  - *Flow*: Soil Nutrients → Plant/Animal Matter → Dead Matter → Decomposition → Soil Nutrients

- **Dead Matter**:
  - *Produced by*: Plant/animal death, waste products, natural decay
  - *Consumed by*: Decomposer fungi and bacteria
  - *Natural increase*: +1 daily from natural decay
  - *Flow*: Living Matter → Death/Waste → Decomposition → Soil Nutrients

### Resource Dynamics and Player Strategy

#### **Cycle Balance Indicators**
Players should understand immediate feedback on cycle health:

- **Atmospheric Balance**: CO₂/O₂ ratio indicator (ideal: 1.5-2.0)
- **Living Matter Flow**: Plant Matter → Animal Matter conversion efficiency
- **Nutrient Circulation**: Dead Matter → Soil Nutrients decomposition rate
- **Biomass Balance**: Total living matter vs. dead matter ratios

#### **Critical Thresholds and Warning Signs**
- **Cycle Stagnation**: When any cycle stops flowing (e.g., no decomposers to process dead matter)
- **Resource Depletion**: When resources approach 0
- **Overflow Issues**: Negative effects when resources exceed maximums
- **Cycle Efficiency**: How well each cycle is functioning

#### **Player Decision Framework**
```
Player sees: CO₂: 12, O₂: 3 (Atmospheric cycle imbalanced)
→ Diagnosis: Too much CO₂, not enough O₂ production
→ Solution: Add more plants (CO₂ consumers, O₂ producers)
→ Card Choice: Select high-efficiency photosynthesizers like Pine Trees

Player sees: Soil Nutrients: 1, Dead Matter: 15 (Nutrient cycle stagnated)
→ Diagnosis: Decomposition bottleneck
→ Solution: Add decomposer species
→ Card Choice: Select rapid decomposers like Earthworms or Mold Clusters

Player sees: Plant Matter: 2, Animal Matter: 8 (Living matter imbalanced)
→ Diagnosis: Too many animals, not enough plants to sustain them
→ Solution: Add more plant species or reduce animal populations
→ Card Choice: Select fast-growing plants or remove some animal cards
```

## Species Population Mechanics

The garden maintains a dynamic registry of all living species and their current populations. Species can be added through card play and removed through death or other effects.

### Population Changes
- **Addition**: Playing a species card adds 1 to that species' population
- **Reproduction**: Species may gain +1 population during daily progression
- **Death**: Species lose -1 population when conditions are unfavorable
- **Removal**: Species with 0 population are removed from the garden

### Population Effects
- All resource consumption/production scales linearly with population
- Each species has a maximum population (defined in species cards)
- Reproduction fails if maximum population is reached
- Some species require minimum populations of other species to survive
- Higher populations increase both benefits and resource demands

## Daily Progression System

At the end of each player turn, the garden advances one day through a structured progression system that applies all environmental and species effects.

### Daily Progression Phases

#### Phase 1: Environmental Effects
1. **Natural Resource Changes**:
   - Dead Matter: +1 (natural decay and accumulation)

2. **Random Events** (future feature):
   - Weather variations affecting atmospheric gases
   - Seasonal changes affecting plant/animal cycles
   - Natural disasters

#### Phase 2: Species Survival Check
For each species in the garden:
1. **Check Survival Requirements**: Compare current resource levels to species' survival ranges
2. **Apply Death**: Remove 1 population if any resource is outside survival range
3. **Mark for Removal**: Remove species entirely if population reaches 0

#### Phase 3: Species Effects Application
For each surviving species, apply daily effects based on current population:

1. **Resource Consumption**:
   - Verify sufficient resources are available
   - Apply consumption (resources cannot go below 0)
   - If insufficient resources, species may die

2. **Resource Production**:
   - Add produced resources to garden totals
   - Apply resource maximums (excess is lost)

3. **Special Abilities**:
   - Apply unique species effects
   - Modify other species' reproduction chances
   - Environmental modifications (shade, moisture retention, etc.)

#### Phase 4: Reproduction Check
For each species in optimal conditions:
1. **Check Optimal Ranges**: All survival requirements in optimal zones
2. **Apply Reproduction Bonuses**: From other species' special abilities
3. **Attempt Reproduction**: Random chance with modifiers
4. **Population Growth**: Add successful reproductions (up to maximum)

#### Phase 5: Garden State Update
1. **Update Resource Display**: Show new resource levels to player
2. **Update Population Display**: Show current species counts
3. **Check Victory Conditions**: Evaluate round objectives
4. **Prepare Next Turn**: Ready garden for next player action

### Example Daily Progression

**Starting State**:
```
Resources: CO₂:8, O₂:5, Plant Matter:3, Animal Matter:2, Dead Matter:3, Soil Nutrients:4
Species: Grass:2, Rabbits:1, Rot Fungi:1
```

**Phase 1 - Environmental**:
```
Dead Matter: 3 → 4 (natural accumulation +1)
```

**Phase 2 - Survival Check**:
```
Grass: CO₂:8, Soil Nutrients:4 - All within range ✓
Rabbits: O₂:5, Plant Matter:3 - All within range ✓
Rot Fungi: Dead Matter:4 - All within range ✓
```

**Phase 3 - Species Effects**:
```
Grass (2 pop): Consumes 2 CO₂, 2 Soil Nutrients → Produces 4 O₂, 2 Plant Matter
Rabbits (1 pop): Consumes 1 O₂, 2 Plant Matter → Produces 1 CO₂, 1 Animal Matter, 1 Dead Matter
Rot Fungi (1 pop): Consumes 2 Dead Matter → Produces 1 CO₂, 2 Soil Nutrients
```

**Final State**:
```
Resources: CO₂:7, O₂:8, Plant Matter:3, Animal Matter:3, Dead Matter:3, Soil Nutrients:4
Species: Grass:2, Rabbits:1, Rot Fungi:1
```

## Game State and Victory

### Current Round State
- Round number (determines difficulty and species availability)
- Target species count for victory
- Current species count in garden
- Player's hand of cards
- Player's deck of remaining cards
- Current resource levels
- Species populations
- Day counter within current round

### Victory Conditions
Each round has specific objectives that must be met for the player to advance to the next round.

#### Primary Victory Condition: Species Target
- **Round 1**: Establish 3 different species
- **Round 2**: Establish 4 different species
- **Round 3**: Establish 5 different species
- **Round 4+**: Establish (2 + Round Number) species

#### Victory Requirements
1. **Species Count**: Must have target number of different species simultaneously
2. **Survival Duration**: All target species must survive for at least 3 consecutive days
3. **Ecosystem Stability**: No resources at critical levels (0 or maximum) when victory is achieved

#### Failure Conditions
- **Deck Exhaustion**: Player runs out of cards before achieving victory
- **Ecosystem Collapse**: All species die due to resource depletion
- **Voluntary Restart**: Player chooses to restart round (with penalty)

#### Victory Rewards
- **Card Selection**: Choose new cards to add to deck for next round
- **Species Unlock**: Gain access to more advanced species
- **Achievement Progress**: Track accomplishments and milestones

