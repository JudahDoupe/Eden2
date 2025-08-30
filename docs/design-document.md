# Eden2 - Game Design Document

## Game Overview

### Vision Statement
Eden2 is a strategic ecosystem simulation card game where players build and manage a living garden by introducing species that interact with environmental resources. The goal is to successfully establish a target number of species in each round while maintaining ecological balance through careful resource management.

### Genre
- **Primary**: Strategy Card Game / Ecosystem Simulation
- **Secondary**: Resource Management, Deck Building

### Platform
- **Web**: Browser-based WebAssembly deployment
- **Mobile**: Native Android app

### Target Audience
- Strategy game enthusiasts
- Players interested in ecological simulation and science
- Deck-building game fans

### Session Length
- **Single Round**: 3-7 minutes
- **Campaign**: Multiple progressive rounds with increasing difficulty

## Core Mechanics

### Primary Loop
1. **Species Selection**: Choose a species card from your hand to introduce to the garden
2. **Card Play**: Play the card to add the species to the garden ecosystem
3. **Alternative Action**: Discard a card if no viable species can survive in current conditions
4. **Garden Progression**: The garden advances one day, with all species affecting resources
5. **Species Lifecycle**: Each species consumes resources, grows, reproduces, or dies based on conditions
6. **Round Continuation**: Repeat until target species count is achieved or round fails

### Resource Management
The garden maintains eight core resources that fluctuate throughout each round:

- **O₂ (Oxygen)**: Produced by photosynthesis, consumed by respiration
- **CO₂ (Carbon Dioxide)**: Consumed by photosynthesis, produced by respiration and decay
- **Ground Water**: Available moisture for plant uptake
- **Sunlight**: Energy source for photosynthetic organisms
- **Green Vegetation**: Living plant biomass
- **Fruit**: Reproductive plant matter and food source
- **Soil Nutrients**: Minerals and organic compounds for plant growth
- **Dead Matter**: Decomposing organic material

### Species Survival
Each species card defines:
- **Survival Ranges**: Min/max tolerance for each resource
- **Resource Effects**: How the species modifies resources each day
- **Population Dynamics**: Growth, reproduction, and death rates

### Round Structure
- **Objective**: Successfully establish a target number of different species
- **Target Scaling**: Species requirement increases with each successive round
- **Success**: Achieve target species count with stable populations
- **Failure**: Unable to maintain minimum viable populations

### Deck Building Between Rounds
- **Card Selection**: Choose which species cards to add to deck for next round
- **Strategic Planning**: Build deck composition based on anticipated challenges
- **Progressive Difficulty**: Higher rounds require more sophisticated ecosystem strategies

## Game Elements

### Garden Resources
The garden ecosystem is defined by eight interconnected resources:

| Resource | Description | Affected By |
|----------|-------------|-------------|
| **O₂** | Oxygen levels in the environment | +Photosynthesis, -Respiration |
| **CO₂** | Carbon dioxide concentration | +Respiration/Decay, -Photosynthesis |
| **Ground Water** | Available soil moisture | +Rain events, -Plant uptake |
| **Sunlight** | Available solar energy | Fixed daily cycles, -Plant absorption |
| **Green Vegetation** | Living plant biomass | +Plant growth, -Herbivore consumption |
| **Fruit** | Seeds and reproductive plant matter | +Plant reproduction, -Frugivore consumption |
| **Soil Nutrients** | Minerals and organic compounds | +Decomposition, -Plant uptake |
| **Dead Matter** | Decomposing organic material | +Species death, -Decomposer activity |

### Species Cards
Each species card represents a unique organism with specific requirements and effects:

#### Card Properties
- **Species Name**: Unique identifier (e.g., "Oak Tree", "Rabbit", "Decomposer Fungi")
- **Survival Ranges**: Min/max tolerance for each of the 8 resources
- **Resource Modifications**: Daily changes the species causes to garden resources
- **Population Dynamics**: Growth, reproduction, and mortality rates
- **Dependencies**: Requirements for other species (food chains, symbiosis)

#### Example Species Cards
```
Oak Tree
- O₂: Produces +2 daily, requires 10-30 range
- CO₂: Consumes -3 daily, tolerates 5-25 range  
- Sunlight: Consumes -4 daily, requires 15+ range
- Soil Nutrients: Consumes -2 daily, requires 8+ range
- Green Vegetation: Produces +3 daily
```

```
Rabbit
- O₂: Consumes -1 daily, requires 12+ range
- CO₂: Produces +1 daily, tolerates 0-20 range
- Green Vegetation: Consumes -2 daily, requires 5+ range
- Dead Matter: Produces +1 daily (waste)
```
Rabbit
- O2: Consumes -1 daily, requires 12+ range
- CO2: Produces +1 daily, tolerates 0-20 range
- Green Vegetation: Consumes -2 daily, requires 5+ range
- Dead Matter: Produces +1 daily (waste)
```

### Garden State
The garden maintains:
- **Current Resource Levels**: Real-time values for all 8 resources
- **Active Species**: List of successfully established species and their populations
- **Day Counter**: Tracks ecosystem progression
- **Stability Metrics**: Measures ecosystem health and balance

---

## Gameplay Flow

### Round Start
1. **Round Objective**: Display target number of species to establish
2. **Garden Initialization**: Set starting resource levels for the round
3. **Deck Preparation**: Use player's constructed deck from previous rounds
4. **Hand Draw**: Draw initial hand of species cards

### Player Turn Sequence
1. **Card Analysis Phase**
   - Examine current garden resource levels
   - Review species cards in hand
   - Assess survival viability for each potential species

2. **Action Selection Phase**
   - **Option A**: Play a species card to introduce it to the garden
   - **Option B**: Discard a card if no species can survive current conditions
   - Consider long-term ecosystem balance and resource trends

3. **Species Introduction** (if card played)
   - Species enters garden with initial population
   - Immediate resource check for survival
   - Species added to active ecosystem participants

### Garden Progression Phase (Daily Cycle)
After each player action, the garden advances one day:

1. **Species Resource Modification**
   - Each active species modifies resources according to its effects
   - Resource changes applied simultaneously across all species

2. **Survival Check**
   - Each species population checked against resource tolerance ranges
   - Species outside tolerance ranges begin declining
   - Species within optimal ranges may grow or reproduce

3. **Population Dynamics**
   - Growth: Species in favorable conditions increase population
   - Reproduction: Mature populations may spawn new individuals
   - Death: Species in poor conditions or overcrowded environments decline
   - Extinction: Species with zero population are removed from garden

4. **Resource Stabilization**
   - Apply any environmental effects (weather, seasonal changes)
   - Update resource level displays
   - Calculate ecosystem stability metrics

### Round End Conditions
- **Success**: Target species count achieved with stable populations
- **Failure**: Unable to maintain minimum viable ecosystem
- **Timeout**: Maximum day limit reached without achieving objective

### Inter-Round Deck Building
1. **Performance Review**: Analyze which species thrived or failed
2. **Card Selection**: Choose new species cards to add to deck
3. **Deck Composition**: Balance specialists vs. generalists for next round
4. **Difficulty Scaling**: Next round increases target species requirement

