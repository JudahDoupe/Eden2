# Individual Creature Sprite Implementation Plan

## Overview

This document outlines the implementation plan for replacing the current population counter with individual sprites for each creature in the Eden2 ecosystem. This change aligns with the project's core principles of individual creature tracking and provides a more intuitive visualization of the ecosystem.

## Current Implementation Analysis

**Current System:**
- The `update_species_display` function in `garden.rs` renders text showing population counts per species
- Population counts are derived from `ecosystem_state.living_population_by_species` HashMap
- The display is updated whenever the ecosystem state changes
- Individual creatures already exist in the simulation layer but are only represented collectively in the UI

**Existing SVG Visualization Components:**
- `SvgRenderingPlugin` and `CreatureVisualizationPlugin` are implemented but not actively used
- Components for linking simulation creatures to visual entities already exist:
  - `CreatureSvgRenderer`
  - `CreatureVisualLink`
  - `VisualizedCreature`
- Systems for spawning and updating visual entities exist but may need activation

## Implementation Phases

### Phase 1: Disable Current Population Counter ⬜

- [ ] Modify `update_species_display` in `garden.rs` to stop showing species counts
- [ ] Keep basic species presence information only (optional)
- [ ] Update UI layout to accommodate visual rather than text-based display

### Phase 2: Enable Individual Creature Sprite Rendering ⬜

- [ ] Verify SVG assets are properly organized:
  ```
  assets/
  └── creatures/
      ├── clover/
      │   ├── juvenile.svg
      │   ├── mature.svg
      │   └── dead.svg
      ├── rabbit/
      │   ├── juvenile.svg
      │   ├── mature.svg
      │   └── dead.svg
      └── [species_name]/
          ├── juvenile.svg
          ├── mature.svg
          └── dead.svg
  ```
- [ ] Activate `CreatureVisualizationPlugin` in app initialization
- [ ] Verify the following systems are running:
  - [ ] `spawn_creature_visualizations`
  - [ ] `update_creature_visualizations`
  - [ ] `cleanup_visual_entities`
- [ ] Configure `GardenBounds` resource to define creature placement area

### Phase 3: Add Missing Integration Components ⬜

- [ ] Create system to synchronize existing creatures:
  - [ ] Implement `initialize_all_creature_visualizations` system
  - [ ] Add to startup systems in plugin
- [ ] Update Garden UI for sprite support:
  - [ ] Modify layout to accommodate sprites
  - [ ] Ensure proper Z-ordering
- [ ] Implement creature position management:
  - [ ] Create `manage_creature_positions` system
  - [ ] Implement species-specific positioning strategies
  - [ ] Add to update systems in plugin

### Phase 4: Lifecycle Integration ⬜

- [ ] Add visual transitions for lifecycle changes:
  - [ ] Implement `apply_lifecycle_visual_effects` system
  - [ ] Add visual indicators for feeding status
  - [ ] Add visual effects for maturation and death
- [ ] Add reproduction visualization:
  - [ ] Position offspring near parents
  - [ ] Add brief animation for reproduction events
- [ ] Add movement patterns by species:
  - [ ] Plants: gentle swaying
  - [ ] Animals: occasional movement
  - [ ] Fungi: growth/expansion effects

### Phase 5: Performance Optimization ⬜

- [ ] Implement culling for large populations:
  - [ ] Create `population_culling` system
  - [ ] Set appropriate thresholds for different device capabilities
- [ ] Add sprite batching optimization:
  - [ ] Group similar creatures for batch rendering
  - [ ] Implement instanced rendering for very large populations
- [ ] Test performance with various population sizes:
  - [ ] Small (10-20 creatures)
  - [ ] Medium (50-100 creatures)
  - [ ] Large (200+ creatures)

## Code Implementations

### Activate Creature Visualization Plugin

```rust
// main.rs
fn main() {
    App::new()
        .add_plugins(SvgRenderingPlugin)
        .add_plugins(CreatureVisualizationPlugin)
        // ... other plugins
        .run();
}
```

### Update Species Display Function

```rust
// garden.rs
pub fn update_species_display(
    ecosystem_state: Res<EcosystemPopulation>,
    mut text_query: Query<&mut Text2d, With<SpeciesDisplayText>>,
) {
    if ecosystem_state.is_changed() {
        let mut species_text = String::from("Species Present:");
        
        if ecosystem_state.creatures.is_empty() {
            species_text.push_str("\nNo species yet");
        } else {
            // Just show which species are present, not counts
            let unique_species: Vec<_> = ecosystem_state.living_population_by_species
                .iter()
                .filter(|(_, &count)| count > 0)
                .map(|(name, _)| name)
                .collect();
                
            for species_name in unique_species {
                species_text.push_str(&format!("\n- {}", species_name));
            }
            
            // Optional: Show total without details
            let total_creatures = ecosystem_state.living_creatures().count();
            species_text.push_str(&format!("\n({} total creatures)", total_creatures));
        }
        
        if let Ok(mut text) = text_query.single_mut() {
            **text = species_text;
        }
    }
}
```

### Initialize All Creature Visualizations

```rust
// svg_renderer.rs
pub fn initialize_all_creature_visualizations(
    mut commands: Commands,
    creatures_query: Query<Entity, (With<IndividualCreature>, Without<VisualizedCreature>)>,
    garden_bounds: Res<GardenBounds>,
) {
    let creature_count = creatures_query.iter().count();
    if creature_count > 0 {
        println!("Initializing visualization for {} existing creatures", creature_count);
        // Let the spawn_creature_visualizations system handle the actual spawning
    }
}

// Update CreatureVisualizationPlugin
impl Plugin for CreatureVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add the SVG plugin for asset loading
            .add_plugins(SvgPlugin)
            
            // Initialize garden bounds resource
            .init_resource::<GardenBounds>()
            
            // Register visualization systems
            .add_systems(Startup, initialize_all_creature_visualizations)
            .add_systems(Update, (
                spawn_creature_visualizations,
                update_creature_visualizations,
                cleanup_visual_entities,
            ));
    }
}
```

### Creature Position Management

```rust
// svg_renderer.rs
pub fn manage_creature_positions(
    creatures_query: Query<(&IndividualCreature, &VisualizedCreature)>,
    mut transforms: Query<&mut Transform>,
    garden_bounds: Res<GardenBounds>,
    time: Res<Time>,
) {
    // Group creatures by species for organization
    let mut species_groups: HashMap<String, Vec<(Entity, Transform)>> = HashMap::new();
    
    // Collect all visualized creatures
    for (creature, visualized) in creatures_query.iter() {
        let species_name = format!("{:?}", creature.species).to_lowercase();
        
        if let Ok(transform) = transforms.get(visualized.visual_entity) {
            species_groups
                .entry(species_name)
                .or_insert_with(Vec::new)
                .push((visualized.visual_entity, *transform));
        }
    }
    
    // Process each species group to optimize positioning
    for (species_name, entities) in species_groups {
        // Different positioning strategies by species type
        match species_name.as_str() {
            "clover" | "grass" => position_plants(&mut transforms, entities, &garden_bounds, time.delta_seconds()),
            "rabbit" | "fox" => position_animals(&mut transforms, entities, &garden_bounds, time.delta_seconds()),
            "mushroom" | "fungi" => position_fungi(&mut transforms, entities, &garden_bounds, time.delta_seconds()),
            _ => position_generic(&mut transforms, entities, &garden_bounds, time.delta_seconds()),
        }
    }
}
```

### Lifecycle Visual Effects

```rust
// svg_renderer.rs
pub fn apply_lifecycle_visual_effects(
    mut creatures_query: Query<(&IndividualCreature, &VisualizedCreature)>,
    mut sprites: Query<&mut Sprite>,
    time: Res<Time>,
) {
    for (creature, visualized) in creatures_query.iter() {
        if let Ok(mut sprite) = sprites.get_mut(visualized.visual_entity) {
            // Apply visual effects based on creature status
            match creature.fed_status {
                FeedingResult::FullyFed => {
                    // Well-fed creatures look vibrant
                    sprite.color = sprite.color.with_saturation(1.0).with_lightness(0.5);
                },
                FeedingResult::PartiallyFed(level) => {
                    // Partially fed creatures have reduced saturation
                    sprite.color = sprite.color.with_saturation(0.7).with_lightness(0.5);
                },
                FeedingResult::Starving => {
                    // Starving creatures look pale
                    sprite.color = sprite.color.with_saturation(0.4).with_lightness(0.6);
                }
            }
            
            // Dead creatures appear faded
            if creature.maturity_stage == MaturityStage::Dead {
                sprite.color = sprite.color.with_alpha(0.5);
            }
        }
    }
}
```

### Population Culling

```rust
// svg_renderer.rs
const MAX_VISIBLE_CREATURES_PER_SPECIES: usize = 100;

pub fn population_culling(
    mut commands: Commands,
    creatures_query: Query<(&IndividualCreature, &VisualizedCreature)>,
    mut sprites: Query<&mut Visibility>,
) {
    // Count creatures by species
    let mut species_counts: HashMap<String, Vec<(Entity, CreatureId)>> = HashMap::new();
    
    for (creature, visualized) in creatures_query.iter() {
        let species_name = format!("{:?}", creature.species);
        
        species_counts
            .entry(species_name)
            .or_insert_with(Vec::new)
            .push((visualized.visual_entity, creature.id));
    }
    
    // Apply culling for species with too many individuals
    for (_, entities) in species_counts {
        if entities.len() > MAX_VISIBLE_CREATURES_PER_SPECIES {
            // Sort by ID for deterministic culling
            let mut sorted_entities = entities.clone();
            sorted_entities.sort_by_key(|(_, id)| *id);
            
            // Show only a subset
            for (i, (entity, _)) in sorted_entities.iter().enumerate() {
                if let Ok(mut visibility) = sprites.get_mut(*entity) {
                    if i < MAX_VISIBLE_CREATURES_PER_SPECIES {
                        *visibility = Visibility::Visible;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}
```

## Testing Checklist

### Functional Testing ⬜
- [ ] Creature sprites appear correctly for each species
- [ ] Lifecycle changes update visuals appropriately
- [ ] New creatures spawn visuals when added
- [ ] Dead creatures are properly visualized
- [ ] Removed creatures have visuals removed

### Visual Quality Testing ⬜
- [ ] Creatures are positioned reasonably within garden bounds
- [ ] Visual style is consistent across all species
- [ ] Lifecycle states are visually distinct
- [ ] Garden appearance is aesthetically pleasing with many creatures
- [ ] Species grouping is visually identifiable

### Performance Testing ⬜
- [ ] Small population test (10-20 creatures)
- [ ] Medium population test (50-100 creatures) 
- [ ] Large population test (200+ creatures)
- [ ] Culling works properly for very large populations
- [ ] FPS remains acceptable across all population sizes

### Edge Case Testing ⬜
- [ ] Visualization during rapid population changes
- [ ] Garden dimension changes (responsive layout)
- [ ] All creature species/lifecycle combinations
- [ ] Window resize events
- [ ] Different screen resolutions

## Implementation Progress Tracking

- [ ] Phase 1: Disable Current Population Counter
- [ ] Phase 2: Enable Individual Creature Sprite Rendering
- [ ] Phase 3: Add Missing Integration Components
- [ ] Phase 4: Lifecycle Integration
- [ ] Phase 5: Performance Optimization
- [ ] Testing and Refinement

## Notes and Observations

*(Add notes here as you implement the changes)*
