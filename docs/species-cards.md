# Eden2 - Species Cards Reference

## Overview

Eden2 features 30 unique species cards divided across three kingdoms that maintain ecological balance. Each species has specific resource requirements for survival and unique effects on the garden ecosystem during daily progression phases.

### Design Philosophy
- **Simplicity to Complexity**: Early unlocked cards have basic needs, later cards require specific conditions
- **Kingdom Interdependence**: Plants, Animals, and Fungi create resource cycles that support each other
- **Specialization**: Each species fills a unique ecological niche with distinct resource interactions

## Kingdom Balance

### Primary Resource Cycles

**Plants** (Producers)
- **Consume**: CO₂, Soil Nutrients
- **Produce**: O₂, Plant Matter
- **Role**: Foundation of the ecosystem, convert CO₂ and nutrients into biomass

**Animals** (Consumers)
- **Consume**: O₂, Plant Matter, Animal Matter
- **Produce**: CO₂, Animal Matter, Dead Matter
- **Role**: Convert plant matter into animal biomass, create waste products

**Fungi** (Decomposers)
- **Consume**: Dead Matter
- **Produce**: Soil Nutrients, CO₂
- **Role**: Recycle nutrients back into the ecosystem, break down organic waste

## Card Mechanics

### Survival Requirements
Each species lists resource ranges for survival:
- **Minimum**: Species dies if resource falls below this level
- **Optimal**: Species thrives and may reproduce at these levels
- **Maximum**: Species dies if resource exceeds this level

### Daily Effects
During the garden progression phase, each living species:
1. **Consumes** specified resources (if available)
2. **Produces** specified resources
3. **Reproduces** if conditions are optimal (adds +1 population)
4. **Dies** if conditions are outside survival range (removes -1 population)

### Population Mechanics
- Species cards start with 1 population when played
- Population can grow through reproduction (max varies by species)
- Population affects resource consumption/production (linear scaling)
- Species is removed from garden when population reaches 0

---

## Plant Kingdom

### 1. Grass
**Type**: Basic Ground Cover  
**Unlock**: Round 1  
**Max Population**: 5

**Survival Requirements**:
- CO₂: 1-10
- Soil Nutrients: 1-5

**Daily Effects** (per population):
- Consumes: 1 CO₂, 1 Soil Nutrients
- Produces: 2 O₂, 1 Plant Matter

*"Hardy grass that forms the foundation of many ecosystems."*

---

### 2. Berry Bushes
**Type**: Fruit Producer  
**Unlock**: Round 2  
**Max Population**: 3

**Survival Requirements**:
- CO₂: 1-8
- Soil Nutrients: 2-6

**Daily Effects** (per population):
- Consumes: 1 CO₂, 2 Soil Nutrients
- Produces: 1 O₂, 2 Plant Matter

*"Sweet berries that provide vital nutrition and attract animals."*

---

### 3. Wildflowers
**Type**: Pollinator Attractor  
**Unlock**: Round 1  
**Max Population**: 4

**Survival Requirements**:
- CO₂: 1-8
- Soil Nutrients: 1-4

**Daily Effects** (per population):
- Consumes: 1 CO₂, 1 Soil Nutrients
- Produces: 1 O₂, 1 Plant Matter
- Special: +1 reproduction chance for all Animal species

*"Colorful flowers that support animal populations."*

---

### 4. Oak Saplings
**Type**: Large Tree  
**Unlock**: Round 3  
**Max Population**: 2

**Survival Requirements**:
- Sunlight: 4-9
- Ground Water: 3-7
- Soil Nutrients: 3-8
- CO₂: 2-10

**Daily Effects** (per population):
- Consumes: 2 CO₂, 2 Sunlight, 3 Soil Nutrients
- Produces: 4 O₂, 2 Green Vegetation
- Special: Reduces Sunlight reaching ground by 1

*"Mighty oaks that dominate the canopy but shade other plants."*

---

### 5. Clover
**Type**: Atmospheric Nitrogen Fixer  
**Unlock**: Round 2  
**Max Population**: 6

**Survival Requirements**:
- Sunlight: 2-6
- Ground Water: 2-5
- Soil Nutrients: 0-4
- CO₂: 1-8

**Daily Effects** (per population):
- Consumes: 2 CO₂, 1 Sunlight, 1 Ground Water
- Produces: 2 O₂, 2 Soil Nutrients, 1 Green Vegetation
- Special: Fixes atmospheric nitrogen - produces Soil Nutrients without consuming Dead Matter

*"Nitrogen-fixing plants that enrich soil by capturing atmospheric nitrogen."*

---

### 6. Ferns
**Type**: Water Cycle Regulator  
**Unlock**: Round 2  
**Max Population**: 4

**Survival Requirements**:
- Sunlight: 1-4
- Ground Water: 3-8
- Soil Nutrients: 2-6
- CO₂: 1-8

**Daily Effects** (per population):
- Consumes: 1 CO₂, 1 Sunlight, 3 Ground Water
- Produces: 1 O₂, 1 Green Vegetation
- Special: Releases 1 Ground Water back to atmosphere (creates humidity cycle)

*"Ancient ferns that regulate moisture and create humid microclimates."*

---

### 7. Sunflowers
**Type**: High Energy Producer  
**Unlock**: Round 3  
**Max Population**: 3

**Survival Requirements**:
- Sunlight: 6-10
- Ground Water: 2-6
- Soil Nutrients: 3-7
- CO₂: 2-10

**Daily Effects** (per population):
- Consumes: 2 CO₂, 3 Sunlight, 2 Soil Nutrients
- Produces: 3 O₂, 1 Green Vegetation, 2 Fruit

*"Tall sunflowers that track the sun and produce abundant seeds."*

---

### 8. Moss
**Type**: Moisture Retainer  
**Unlock**: Round 1  
**Max Population**: 8

**Survival Requirements**:
- Sunlight: 1-3
- Ground Water: 4-10
- Soil Nutrients: 1-3
- CO₂: 1-6

**Daily Effects** (per population):
- Consumes: 1 CO₂, 1 Sunlight
- Produces: 1 O₂
- Special: Prevents 1 Ground Water from evaporating

*"Soft moss that retains moisture in the garden."*

---

### 9. Pine Trees
**Type**: Carbon Sink  
**Unlock**: Round 4  
**Max Population**: 2

**Survival Requirements**:
- Sunlight: 3-8
- Ground Water: 2-6
- Soil Nutrients: 2-6
- CO₂: 2-10

**Daily Effects** (per population):
- Consumes: 3 CO₂, 2 Sunlight, 2 Soil Nutrients
- Produces: 4 O₂, 1 Green Vegetation
- Special: Stores 1 CO₂ permanently (carbon sequestration), reduces atmospheric CO₂

*"Evergreen carbon sinks that lock away atmospheric carbon long-term."*

---

### 10. Vegetable Plants
**Type**: Nutrient Dense Producer  
**Unlock**: Round 3  
**Max Population**: 4

**Survival Requirements**:
- Sunlight: 5-8
- Ground Water: 3-7
- Soil Nutrients: 4-8
- CO₂: 2-8

**Daily Effects** (per population):
- Consumes: 1 CO₂, 2 Sunlight, 3 Soil Nutrients
- Produces: 1 O₂, 3 Fruit

*"Cultivated vegetables that require rich soil but provide abundant food."*

---

## Animal Kingdom

### 1. Rabbits
**Type**: Primary Herbivore  
**Unlock**: Round 1  
**Max Population**: 4

**Survival Requirements**:
- O₂: 2-8
- Plant Matter: 2-8

**Daily Effects** (per population):
- Consumes: 1 O₂, 2 Plant Matter
- Produces: 1 CO₂, 1 Animal Matter, 1 Dead Matter

*"Quick-breeding herbivores that convert plant matter efficiently."*

---

### 2. Earthworms
**Type**: Nutrient Accelerator  
**Unlock**: Round 1  
**Max Population**: 8

**Survival Requirements**:
- O₂: 1-5
- Dead Matter: 1-6

**Daily Effects** (per population):
- Consumes: 1 O₂, 2 Dead Matter
- Produces: 1 CO₂, 3 Soil Nutrients
- Special: Accelerates nutrient cycle - doubles Soil Nutrient production from Dead Matter

*"Essential decomposers that rapidly convert organic matter into plant-available nutrients."*

---

### 3. Honeybees
**Type**: Pollinator  
**Unlock**: Round 2  
**Max Population**: 6

**Survival Requirements**:
- O₂: 2-8
- Plant Matter: 1-4

**Daily Effects** (per population):
- Consumes: 1 O₂, 1 Plant Matter
- Produces: 1 CO₂
- Special: +2 reproduction chance for all Plant species

*"Vital pollinators that boost plant reproduction dramatically."*

---

### 4. Field Mice
**Type**: Seed Disperser  
**Unlock**: Round 2  
**Max Population**: 5

**Survival Requirements**:
- O₂: 2-7
- Fruit: 2-6
- Green Vegetation: 1-4

**Daily Effects** (per population):
- Consumes: 1 O₂, 1 Fruit, 1 Green Vegetation
- Produces: 1 CO₂, 1 Dead Matter
- Special: +1 reproduction chance for Plant species

*"Small rodents that help spread seeds throughout the garden."*

---

### 5. Butterflies
**Type**: Mobile Pollinator  
**Unlock**: Round 2  
**Max Population**: 6

**Survival Requirements**:
- O₂: 2-8
- Fruit: 1-3
- Sunlight: 4-9

**Daily Effects** (per population):
- Consumes: 1 O₂, 1 Fruit
- Produces: 1 CO₂
- Special: +1 reproduction chance for flowering Plant species

*"Beautiful pollinators that prefer sunny gardens."*

---

### 6. Ladybugs
**Type**: Pest Controller  
**Unlock**: Round 3  
**Max Population**: 4

**Survival Requirements**:
- O₂: 2-8
- Green Vegetation: 1-5 (indirect requirement)

**Daily Effects** (per population):
- Consumes: 1 O₂
- Produces: 1 CO₂
- Special: Prevents 1 Green Vegetation loss from all Plant species

*"Beneficial insects that protect plants from harmful pests."*

---

### 7. Frogs
**Type**: Nutrient Accelerator  
**Unlock**: Round 3  
**Max Population**: 3

**Survival Requirements**:
- O₂: 2-7
- Ground Water: 4-9
- Animal Population: 2-10 (other animals present)

**Daily Effects** (per population):
- Consumes: 1 O₂, 1 Ground Water
- Produces: 2 CO₂, 2 Dead Matter
- Special: Accelerates decomposition - converts 1 Green Vegetation to Dead Matter

*"Amphibians that speed up the nutrient cycle by processing living matter."*

---

### 8. Squirrels
**Type**: Tree Dweller  
**Unlock**: Round 4  
**Max Population**: 3

**Survival Requirements**:
- O₂: 2-8
- Fruit: 2-6
- Green Vegetation: 3-8 (needs trees)

**Daily Effects** (per population):
- Consumes: 1 O₂, 2 Fruit
- Produces: 1 CO₂, 1 Dead Matter
- Special: Requires tree-type plants to survive

*"Arboreal mammals that depend on large plants for shelter."*

---

### 9. Snails
**Type**: Moisture Recycler  
**Unlock**: Round 2  
**Max Population**: 6

**Survival Requirements**:
- O₂: 1-6
- Dead Matter: 1-4
- Ground Water: 3-8

**Daily Effects** (per population):
- Consumes: 1 O₂, 1 Dead Matter, 2 Ground Water
- Produces: 1 CO₂, 1 Soil Nutrients
- Special: Releases 1 Ground Water back to soil (moisture recycling)

*"Mollusks that help maintain the water cycle while decomposing matter."*

---

### 10. Birds
**Type**: Aerial Seed Disperser  
**Unlock**: Round 4  
**Max Population**: 4

**Survival Requirements**:
- O₂: 3-9
- Fruit: 2-6
- Animal Population: 1-8 (for insects)

**Daily Effects** (per population):
- Consumes: 1 O₂, 1 Fruit
- Produces: 1 CO₂, 1 Dead Matter
- Special: +2 reproduction chance for fruit-producing plants

*"Flying seed dispersers that spread plants across wide areas."*

---

## Fungi Kingdom

### 1. Rot Fungi
**Type**: Basic Decomposer  
**Unlock**: Round 1  
**Max Population**: 6

**Survival Requirements**:
- Dead Matter: 2-8
- O₂: 0-4 (prefers low oxygen)

**Daily Effects** (per population):
- Consumes: 2 Dead Matter
- Produces: 1 CO₂, 2 Soil Nutrients

*"Essential decomposers that break down dead organic matter."*

---

### 2. Puffballs
**Type**: Spore Spreader  
**Unlock**: Round 2  
**Max Population**: 4

**Survival Requirements**:
- Dead Matter: 1-5
- Soil Nutrients: 2-6

**Daily Effects** (per population):
- Consumes: 1 Dead Matter, 1 Soil Nutrients
- Produces: 1 CO₂, 1 Soil Nutrients
- Special: +1 reproduction chance for all Fungi species

*"Reproductive fungi that help spread fungal spores throughout the garden."*

---

### 3. Giant Mushrooms
**Type**: Ecosystem Engineer  
**Unlock**: Round 3  
**Max Population**: 2

**Survival Requirements**:
- Dead Matter: 3-8
- Soil Nutrients: 3-7
- O₂: 0-3 (prefers low oxygen)

**Daily Effects** (per population):
- Consumes: 3 Dead Matter, 2 Soil Nutrients
- Produces: 2 CO₂, 4 Soil Nutrients
- Special: Creates microhabitat - increases survival range for all other Fungi by +1

*"Massive ecosystem engineers that create favorable conditions for other fungi."*

---

### 4. Mycorrhizal Fungi
**Type**: Plant Symbiont  
**Unlock**: Round 2  
**Max Population**: 5

**Survival Requirements**:
- Plant Population: 2-10 (needs living plants)
- Soil Nutrients: 1-6
- Ground Water: 2-6

**Daily Effects** (per population):
- Consumes: 1 Soil Nutrients, 1 Ground Water
- Produces: 1 CO₂
- Special: +1 Soil Nutrients production for all Plant species

*"Beneficial fungi that form partnerships with plant roots."*

---

### 5. Shelf Fungi
**Type**: Wood Decomposer  
**Unlock**: Round 3  
**Max Population**: 4

**Survival Requirements**:
- Dead Matter: 2-6
- Green Vegetation: 3-8 (needs dead trees)
- Ground Water: 1-5

**Daily Effects** (per population):
- Consumes: 1 Dead Matter, 1 Green Vegetation
- Produces: 1 CO₂, 2 Soil Nutrients
- Special: Can consume living plant matter

*"Tree-dwelling fungi that slowly decompose woody material."*

---

### 6. Mold Clusters
**Type**: Rapid Nutrient Cycler  
**Unlock**: Round 1  
**Max Population**: 8

**Survival Requirements**:
- Dead Matter: 1-6
- Ground Water: 3-8
- O₂: 0-5

**Daily Effects** (per population):
- Consumes: 1 Dead Matter, 1 Ground Water
- Produces: 1 CO₂, 2 Soil Nutrients
- Special: Rapid cycling - converts Dead Matter to Nutrients 2x faster than other decomposers

*"Fast-growing mold that rapidly cycles nutrients back into the ecosystem."*

---

### 7. Truffle Fungi
**Type**: Underground Delicacy  
**Unlock**: Round 4  
**Max Population**: 3

**Survival Requirements**:
- Soil Nutrients: 4-8
- Ground Water: 3-7
- Plant Population: 3-8 (needs tree roots)

**Daily Effects** (per population):
- Consumes: 2 Soil Nutrients, 1 Ground Water
- Produces: 1 CO₂, 1 Soil Nutrients, 2 Fruit
- Special: Requires tree-type plants nearby

*"Prized underground fungi that form complex root relationships."*

---

### 8. Coral Fungi
**Type**: Branching Network  
**Unlock**: Round 3  
**Max Population**: 4

**Survival Requirements**:
- Dead Matter: 2-6
- Soil Nutrients: 2-5
- Fungi Population: 2-8 (needs other fungi)

**Daily Effects** (per population):
- Consumes: 1 Dead Matter, 1 Soil Nutrients
- Produces: 1 CO₂, 2 Soil Nutrients
- Special: +1 efficiency to all other Fungi species

*"Interconnected fungi that boost the entire fungal network."*

---

### 9. Yeast Colonies
**Type**: Microscopic Decomposer  
**Unlock**: Round 2  
**Max Population**: 10

**Survival Requirements**:
- Fruit: 1-6 (ferments sugars)
- Ground Water: 2-6
- O₂: 0-3

**Daily Effects** (per population):
- Consumes: 1 Fruit, 1 Ground Water
- Produces: 2 CO₂, 1 Soil Nutrients
- Special: Converts fruit into CO₂ efficiently

*"Tiny organisms that ferment plant sugars into useful compounds."*

---

### 10. Slime Molds
**Type**: Mobile Decomposer  
**Unlock**: Round 4  
**Max Population**: 3

**Survival Requirements**:
- Dead Matter: 3-8
- Ground Water: 4-9
- O₂: 1-6

**Daily Effects** (per population):
- Consumes: 2 Dead Matter, 2 Ground Water
- Produces: 1 CO₂, 3 Soil Nutrients
- Special: Can move to areas with most Dead Matter

*"Unique organisms that can move to find the best decomposition sites."*

---

## Resource Interactions

### Ecosystem Cycles

**Carbon Cycle** (CO₂ ↔ O₂):
- **Heavy CO₂ Consumers**: Pine Trees (3 CO₂), Clover (2 CO₂), Oak Saplings (2 CO₂), Sunflowers (2 CO₂)
- **Standard CO₂ Consumers**: Most plants (1 CO₂ per population)
- **Heavy O₂ Producers**: Pine Trees (4 O₂), Oak Saplings (4 O₂), Sunflowers (3 O₂)
- **O₂ to CO₂ Converters**: All animals consume O₂, produce CO₂
- **CO₂ Amplifiers**: Giant Mushrooms (2 CO₂), Yeast Colonies (2 CO₂)

**Living Matter Cycle** (Plant Matter ↔ Animal Matter):
- **Plant Matter Sources**: All plants produce Plant Matter through photosynthesis
- **Plant Matter Consumers**: Herbivorous animals convert Plant Matter to Animal Matter
- **Animal Matter Sources**: Animals produce Animal Matter through growth
- **Animal Matter Consumers**: Carnivorous animals and decomposers

**Nutrient Cycle** (Soil Nutrients → Growth → Dead Matter → Soil Nutrients):
- **Soil Nutrient Sources**: 
  - *From Dead Matter*: Earthworms (3x), Mold Clusters (2x), Giant Mushrooms (4x), Rot Fungi
  - *From Atmosphere*: Clover (atmospheric nitrogen fixing)
- **Dead Matter Creators**: All animals produce Dead Matter through waste and death
- **Cycle Accelerators**: 
  - Earthworms (2x Dead Matter → 3x Soil Nutrients)
  - Frogs (Living Matter → Dead Matter)
  - Mold Clusters (rapid nutrient cycling)

### Strategic Considerations

1. **Early Game - Establish Base Cycles**: 
   - Start with basic producers (Grass) and rapid nutrient cyclers (Mold Clusters, Earthworms)
   - Add water conservers (Moss) to prevent resource loss
   - Focus on building sustainable CO₂ ↔ O₂ exchange

2. **Mid Game - Accelerate Cycles**: 
   - Introduce cycle accelerators (Clover for nitrogen, Frogs for decomposition)
   - Add pollinators (Bees, Butterflies) to boost plant reproduction
   - Balance animal populations to create steady Dead Matter flow

3. **Late Game - Optimize and Specialize**: 
   - Add ecosystem engineers (Giant Mushrooms, Pine Trees) for environmental control
   - Introduce specialists that require complex conditions (Truffle Fungi, Squirrels)
   - Fine-tune ratios between kingdoms based on resource bottlenecks

4. **Cycle Management**:
   - **Carbon Imbalance**: Too much CO₂? Add more plants. Too little? Add more animals/fungi
   - **Soil Nutrient Shortage**: Boost decomposers (Earthworms, Mold) or nitrogen fixers (Clover)
   - **Living Matter Imbalance**: Too much Plant Matter? Add herbivores. Too much Animal Matter? Add carnivores/decomposers
   - **Dead Matter Accumulation**: Add more decomposer fungi to process organic waste

5. **Kingdom Balance Ratios**:
   - **Plants**: 40-50% of species (CO₂ processing, Plant Matter foundation)
   - **Animals**: 25-35% of species (O₂ processing, matter conversion)
   - **Fungi**: 20-30% of species (nutrient recycling and ecosystem engineering)

This simplified ecosystem creates emergent gameplay where players must understand and manage fundamental interdependencies between CO₂/O₂ cycles, living matter flows, and nutrient recycling. Each cycle requires careful balance to prevent collapse while maximizing efficiency through the three core kingdoms.
