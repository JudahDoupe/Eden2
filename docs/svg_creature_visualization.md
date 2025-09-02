# SVG Creature Visualization System

## Overview

The SVG Creature Visualization system automatically creates visual representations for creatures in the Eden2 ecosystem. The system renders SVG files as sprites in the Bevy game engine, associating each creature with the appropriate visual representation based on its species and lifecycle stage.

## Architecture: Simulation-Visualization Separation

**Important**: The system maintains a strict separation between simulation entities and visualization entities:

- **Simulation Entities**: Contain all gameplay logic, lifecycle states, biomass data, and other ecological properties
- **Visualization Entities**: Purely visual representations with no gameplay logic
- **Entity Linking**: Each visualization entity is linked to exactly one simulation entity through the `CreatureVisualLink` component

This separation allows the simulation to run independently of the visualization layer, ensuring:
1. Simulation logic remains untainted by rendering concerns
2. Visualization can be updated, replaced, or disabled without affecting the simulation
3. Multiple visualization strategies can be implemented for the same simulation data
4. Performance optimization by updating visualizations at different rates than simulation

## Key Features

- **SVG to Sprite Conversion**: Converts SVG files to texture assets that can be rendered as sprites
- **Lifecycle-Aware Visualization**: Automatically shows the correct visual representation based on creature maturity stage (juvenile, mature, dead)
- **Dynamic Creature Tracking**: Creates and updates visual entities when creatures are added to the ecosystem or change state
- **Random Placement**: Places creatures at random positions within defined garden bounds

## Directory Structure

SVG assets should be organized in the following structure:

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

## Components and Resources

### Components

- **`SvgComponent`**: Marks an entity as having an SVG-based sprite
- **`CreatureSvgRenderer`**: Specifies which SVG to use based on creature type and lifecycle stage
- **`CreatureVisualLink`**: Links a visualization entity to its simulation entity

### Resources

- **`GardenBounds`**: Defines the area where creatures can be placed

## Entity Relationship

```
Simulation Layer:
┌─────────────────────────┐
│ IndividualCreature      │
│ - species               │
│ - maturity_stage        │◄───┐
│ - biomass               │    │
│ - other simulation data │    │ One-to-one
└─────────────────────────┘    │ relationship
                               │
Visualization Layer:           │
┌─────────────────────────┐    │
│ SpriteBundle            │    │
│ SvgComponent            │    │
│ CreatureSvgRenderer     │    │
│ CreatureVisualLink ─────┼────┘
└─────────────────────────┘
```

## Systems

- **`load_creature_svgs`**: Loads SVG assets for new creatures
- **`spawn_creature_visualizations`**: Creates visual entities for newly added simulation entities
- **`update_creature_visualizations`**: Updates visual entities when simulation entities change state
- **`update_svg_sprites`**: Updates sprite textures when SVG assets are loaded
- **`cleanup_visual_entities`**: Removes visualization entities when their simulation entities are removed

## Usage Example

```rust
// Initialize the system
app
    .add_plugins(SvgRenderingPlugin)
    .insert_resource(GardenBounds {
        min_x: -400.0,
        max_x: 400.0,
        min_y: -300.0,
        max_y: 300.0,
    });

// When adding a new creature (simulation entity only)
commands
    .spawn((
        IndividualCreature {
            id: next_id(),
            species: Species::Clover,
            maturity_stage: MaturityStage::Juvenile,
            // other simulation data...
        },
        // The visualization system will automatically create a separate
        // visual entity linked to this simulation entity
    ));
```

## Implementation Details

### SVG Asset Loading

The system uses the resvg, tiny-skia, and usvg libraries to convert SVG files to pixel data that can be used as textures in Bevy. SVG files are loaded asynchronously and converted to Image assets.

### Visual Entity Creation

When a new simulation entity is detected (by having creature components but no associated visual entity), the system:
1. Determines the appropriate SVG asset based on species and lifecycle stage
2. Creates a new, separate entity with sprite rendering components
3. Sets the sprite's position to a random location within the garden bounds
4. Links the visual entity to the simulation entity via the `CreatureVisualLink` component

### Visual Entity Updates

When a simulation entity changes state (e.g., matures from juvenile to adult), the system:
1. Detects the change by comparing the current stage with the rendered stage
2. Updates the SVG asset on the corresponding visualization entity to reflect the new stage
3. Maintains the same position and other visual properties

## Technical Considerations

- SVG assets are processed at runtime, which may cause a slight performance impact when first loading a species
- The system is designed to handle hundreds of individual creatures efficiently
- For very large ecosystems, consider implementing a spatial partitioning system to optimize rendering
- The separation of simulation and visualization entities allows different update frequencies if needed
