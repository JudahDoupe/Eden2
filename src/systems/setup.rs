use bevy::prelude::*;
use crate::{
    components::*,
    constants::*,
    types::TileType,
    systems::cards::spawn_hand_ui,
};

pub fn setup(mut commands: Commands) {
    // Initialize game state
    let mut game_state = GameState::default();
    game_state.draw_initial_hand();
    
    // Spawn camera
    commands.spawn(Camera2d);

    // Calculate grid positioning
    let grid_width = GRID_SIZE as f32 * TILE_SIZE + (GRID_SIZE - 1) as f32 * TILE_SPACING;
    let start_x = -grid_width / 2.0 + TILE_SIZE / 2.0;
    let start_y = grid_width / 2.0 - TILE_SIZE / 2.0;

    // Spawn tiles
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            spawn_tile(&mut commands, x, y, start_x, start_y);
        }
    }

    // Spawn hand UI
    spawn_hand_ui(&mut commands, &game_state);

    // Spawn UI text for ecosystem stats
    spawn_stats_ui(&mut commands);
    
    // Insert game state as resource
    commands.insert_resource(game_state);
}

fn spawn_tile(commands: &mut Commands, x: usize, y: usize, start_x: f32, start_y: f32) {
    let pos_x = start_x + x as f32 * (TILE_SIZE + TILE_SPACING);
    let pos_y = start_y - y as f32 * (TILE_SIZE + TILE_SPACING);

    // Spawn tile background (border)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(TILE_SIZE + TILE_BORDER_WIDTH * 2.0, TILE_SIZE + TILE_BORDER_WIDTH * 2.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(pos_x, pos_y, 0.0)),
        TileBorder,
    ));

    // Spawn main tile
    commands.spawn((
        Sprite {
            color: TileType::Empty.color(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
        Transform::from_translation(Vec3::new(pos_x, pos_y, 1.0)),
        Tile {
            x,
            y,
            tile_type: TileType::Empty,
            is_hovered: false,
        },
        TileSprite,
    ));
}

fn spawn_stats_ui(commands: &mut Commands) {
    // Simplified text spawning for now - will fix once basic compilation works
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        StatsText,
    ));
}
