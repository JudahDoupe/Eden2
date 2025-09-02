use bevy::prelude::*;
use crate::rendering::svg::{SvgSpriteBundle, SvgComponent, SvgAsset};
use rand::prelude::*;
use crate::gameplay::lifecycle::IndividualCreature;
use crate::gameplay::species::Species;
use crate::creatures::CreatureComponent;

/// Component to specify which SVG to use based on creature type and lifecycle stage
#[derive(Component)]
pub struct CreatureSvgRenderer {
    pub species_name: String,
    pub lifecycle_stage: String,
}

/// Component to link a visualization entity to its creature data entity
#[derive(Component)]
pub struct CreatureVisualLink {
    pub creature_entity: Entity,
}

/// Component to mark a creature as having a visual representation
#[derive(Component)]
pub struct VisualizedCreature {
    pub visual_entity: Entity,
    pub current_lifecycle_stage: String,
}

/// Resource to store garden bounds for random placement
#[derive(Resource)]
pub struct GardenBounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl Default for GardenBounds {
    fn default() -> Self {
        Self {
            min_x: -400.0,
            max_x: 400.0,
            min_y: -300.0,
            max_y: 300.0,
        }
    }
}

/// Convert maturity stage to string representation
fn get_lifecycle_stage_string(maturity_stage: &str) -> String {
    match maturity_stage {
        "Juvenile" => "juvenile",
        "Mature" => "mature",
        "Dead" => "dead",
        _ => "unknown",
    }.to_string()
}

/// System to initialize visual entities for all existing creatures
pub fn initialize_all_creature_visualizations(
    mut commands: Commands,
    creatures_query: Query<Entity, (With<IndividualCreature>, Without<VisualizedCreature>)>,
) {
    let creature_count = creatures_query.iter().count();
    if creature_count > 0 {
        println!("Initializing visualization for {} existing creatures", creature_count);
        // Let the spawn_creature_visualizations system handle the actual spawning
    }
}

/// System to spawn visual entities for newly added creatures
pub fn spawn_creature_visualizations(
    mut commands: Commands,
    creatures_query: Query<(Entity, &IndividualCreature), Without<VisualizedCreature>>,
    garden_bounds: Res<GardenBounds>,
) {
    let mut rng = rand::thread_rng();
    
    for (creature_entity, individual) in creatures_query.iter() {
        // Get lifecycle stage as string
        let lifecycle_stage = get_lifecycle_stage_string(&format!("{:?}", individual.maturity_stage));
        
        // Get species name as string
        let species_name = format!("{:?}", individual.species).to_lowercase();
        
        // Random position within garden bounds
        let random_x = rng.gen_range(garden_bounds.min_x..garden_bounds.max_x);
        let random_y = rng.gen_range(garden_bounds.min_y..garden_bounds.max_y);
        
        // Load SVG asset - using path from visualization/garden/creatures
        let svg_path = format!("visualization/garden/creatures/{}/{}.svg", species_name, lifecycle_stage);
        
        // Create visualization entity
        let visual_entity = commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                ..default()
            },
            CreatureSvgRenderer {
                species_name: species_name.clone(),
                lifecycle_stage: lifecycle_stage.clone(),
            },
            SvgComponent,
            Name::new(format!("{} {} (Visual)", species_name, lifecycle_stage)),
        )).id();
        
        // Link visualization entity back to creature entity
        commands.entity(visual_entity).insert(CreatureVisualLink {
            creature_entity,
        });
        
        // Mark the creature as having a visual representation
        commands.entity(creature_entity).insert(VisualizedCreature {
            visual_entity,
            current_lifecycle_stage: lifecycle_stage,
        });
    }
}

/// System to update visual entities when creatures change state
pub fn update_creature_visualizations(
    mut commands: Commands,
    mut creatures_query: Query<(Entity, &IndividualCreature, &mut VisualizedCreature)>,
    mut visuals_query: Query<&mut CreatureSvgRenderer>,
    asset_server: Res<AssetServer>,
) {
    for (creature_entity, individual, mut visualized) in creatures_query.iter_mut() {
        // Get current lifecycle stage as string
        let lifecycle_stage = get_lifecycle_stage_string(&format!("{:?}", individual.maturity_stage));
        
        // Check if lifecycle stage has changed
        if lifecycle_stage != visualized.current_lifecycle_stage {
            // Update the current stage
            visualized.current_lifecycle_stage = lifecycle_stage.clone();
            
            // Get the species name
            let species_name = format!("{:?}", individual.species).to_lowercase();
            
            // Update the renderer component on the visual entity
            if let Ok(mut renderer) = visuals_query.get_mut(visualized.visual_entity) {
                renderer.lifecycle_stage = lifecycle_stage;
                renderer.species_name = species_name;
                
                // Load new SVG asset for the updated lifecycle stage
                let svg_path = format!("visualization/garden/creatures/{}/{}.svg", renderer.species_name, renderer.lifecycle_stage);
                let svg_handle: Handle<SvgAsset> = asset_server.load(&svg_path);
                
                // Replace the existing entity with a new one containing the updated asset
                commands.entity(visualized.visual_entity).insert(svg_handle);
            }
        }
    }
}

/// System to cleanup visual entities when their simulation entities are removed
pub fn cleanup_visual_entities(
    mut commands: Commands,
    removed_creatures: RemovedComponents<IndividualCreature>,
    visual_links: Query<(Entity, &CreatureVisualLink)>,
) {
    for removed_entity in removed_creatures.iter() {
        // Find and remove the corresponding visual entity
        for (visual_entity, link) in visual_links.iter() {
            if link.creature_entity == removed_entity {
                commands.entity(visual_entity).despawn_recursive();
                break;
            }
        }
    }
}

/// Simple system to add minimal movement to creatures
pub fn manage_creature_positions(
    mut creatures_query: Query<(&IndividualCreature, &VisualizedCreature)>,
    mut transforms: Query<&mut Transform>,
    garden_bounds: Res<GardenBounds>,
    time: Res<Time>,
) {
    // Add a very subtle movement to creatures to make them feel alive
    for (_, visualized) in creatures_query.iter() {
        if let Ok(mut transform) = transforms.get_mut(visualized.visual_entity) {
            // Simple gentle swaying for all creatures
            let entity_id = visualized.visual_entity.index();
            let sway = (time.elapsed_seconds() * 1.0 + entity_id as f32 * 0.1).sin() * 0.2;
            transform.translation.y += sway * time.delta_seconds();
        }
    }
}

/// Simple system for basic visual effects
pub fn apply_lifecycle_visual_effects(
    creatures_query: Query<(&IndividualCreature, &VisualizedCreature)>,
    mut sprites: Query<&mut Sprite>,
) {
    // For now, just ensure all sprites are displayed with normal coloring
    for (_, visualized) in creatures_query.iter() {
        if let Ok(mut sprite) = sprites.get_mut(visualized.visual_entity) {
            // Set normal coloration
            sprite.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
        }
    }
}

// We'll implement performance optimizations later when needed
// For now, we're focusing on getting the creatures rendered correctly
