use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum TileType {
    Empty,
    Land,
    Water,
    Forest,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CardType {
    PlantSeed,  // Creates Land
    Irrigate,   // Creates Water
    GrowForest, // Creates Forest
    ClearLand,  // Creates Empty
}

impl CardType {
    pub fn name(&self) -> &'static str {
        match self {
            CardType::PlantSeed => "Plant Seed",
            CardType::Irrigate => "Irrigate", 
            CardType::GrowForest => "Grow Forest",
            CardType::ClearLand => "Clear Land",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            CardType::PlantSeed => "Turn tile into Land",
            CardType::Irrigate => "Turn tile into Water",
            CardType::GrowForest => "Turn tile into Forest", 
            CardType::ClearLand => "Turn tile into Empty",
        }
    }
    
    pub fn target_tile_type(&self) -> TileType {
        match self {
            CardType::PlantSeed => TileType::Land,
            CardType::Irrigate => TileType::Water,
            CardType::GrowForest => TileType::Forest,
            CardType::ClearLand => TileType::Empty,
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            CardType::PlantSeed => Color::srgb(0.6, 0.4, 0.2),   // Brown
            CardType::Irrigate => Color::srgb(0.2, 0.4, 0.8),    // Blue
            CardType::GrowForest => Color::srgb(0.2, 0.6, 0.2),  // Green
            CardType::ClearLand => Color::srgb(0.3, 0.3, 0.3),   // Gray
        }
    }
}

impl TileType {
    pub fn color(&self) -> Color {
        match self {
            TileType::Empty => Color::srgb(0.3, 0.3, 0.3),  // Dark gray
            TileType::Land => Color::srgb(0.6, 0.4, 0.2),   // Brown
            TileType::Water => Color::srgb(0.2, 0.4, 0.8),  // Blue
            TileType::Forest => Color::srgb(0.2, 0.6, 0.2), // Green
        }
    }
    
    pub fn hover_color(&self) -> Color {
        match self {
            TileType::Empty => Color::srgb(0.5, 0.5, 0.5),  // Lighter gray
            TileType::Land => Color::srgb(0.8, 0.6, 0.4),   // Lighter brown
            TileType::Water => Color::srgb(0.4, 0.6, 1.0),  // Lighter blue
            TileType::Forest => Color::srgb(0.4, 0.8, 0.4), // Lighter green
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            TileType::Empty => "Empty",
            TileType::Land => "Land",
            TileType::Water => "Water", 
            TileType::Forest => "Forest",
        }
    }
    
    pub fn next(&self) -> TileType {
        match self {
            TileType::Empty => TileType::Land,
            TileType::Land => TileType::Water,
            TileType::Water => TileType::Forest,
            TileType::Forest => TileType::Empty,
        }
    }
}
