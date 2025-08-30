use bevy::prelude::*;
use crate::types::{ResourceType, CardType, SpeciesType, Kingdom};
use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Component)]
pub struct GardenText;

#[derive(Component)]
pub struct ResourceDisplayText;

#[derive(Component)]
pub struct SpeciesDisplayText;

// Card components
#[derive(Component)]
pub struct Card {
    pub card_type: CardType,
    pub hand_index: usize,
    pub is_selected: bool,
}

#[derive(Component)]
pub struct CardSprite;

#[derive(Component)]
pub struct CardText;

// Flag to track if layout has been initialized to actual window size
#[derive(Resource, Default)]
pub struct LayoutInitialized(pub bool);

// Layout components for responsive design
#[derive(Component)]
pub struct GardenBackground;

// Responsive sizing utility
#[derive(Resource, Clone)]
pub struct ResponsiveSize {
    pub window_size: Vec2,
}

impl ResponsiveSize {
    pub fn new(window_size: Vec2) -> Self {
        Self { window_size }
    }
    
    // Width-based sizing (percentage of screen width)
    pub fn width_pct(&self, percentage: f32) -> f32 {
        self.window_size.x * (percentage / 100.0)
    }
    
    // Height-based sizing (percentage of screen height)
    pub fn height_pct(&self, percentage: f32) -> f32 {
        self.window_size.y * (percentage / 100.0)
    }
    
    // Minimum dimension based sizing (useful for square elements)
    pub fn min_pct(&self, percentage: f32) -> f32 {
        self.window_size.x.min(self.window_size.y) * (percentage / 100.0)
    }
    
    // Maximum dimension based sizing
    pub fn max_pct(&self, percentage: f32) -> f32 {
        self.window_size.x.max(self.window_size.y) * (percentage / 100.0)
    }
    
    // Responsive font size based on screen dimensions
    pub fn font_size(&self, size_class: FontSizeClass) -> f32 {
        let base_size = match size_class {
            FontSizeClass::Small => self.min_pct(2.0),    // 2% of smallest dimension
            FontSizeClass::Medium => self.min_pct(2.5),   // 2.5% of smallest dimension
            FontSizeClass::Large => self.min_pct(3.0),    // 3% of smallest dimension
            FontSizeClass::XLarge => self.min_pct(4.0),   // 4% of smallest dimension
        };
        base_size.clamp(10.0, 24.0) // Ensure readable range
    }
    
    // Responsive padding/margin
    pub fn padding(&self, padding_class: PaddingClass) -> f32 {
        match padding_class {
            PaddingClass::XSmall => self.min_pct(0.5),   // 0.5%
            PaddingClass::Small => self.min_pct(1.0),    // 1%
            PaddingClass::Medium => self.min_pct(2.0),   // 2%
            PaddingClass::Large => self.min_pct(3.0),    // 3%
            PaddingClass::XLarge => self.min_pct(4.0),   // 4%
        }
    }
    
    // Responsive spacing between elements
    pub fn spacing(&self, spacing_class: SpacingClass) -> f32 {
        match spacing_class {
            SpacingClass::Tight => self.width_pct(1.0),     // 1% of width
            SpacingClass::Normal => self.width_pct(2.0),    // 2% of width
            SpacingClass::Relaxed => self.width_pct(3.0),   // 3% of width
            SpacingClass::Loose => self.width_pct(5.0),     // 5% of width
        }
    }
}

#[derive(Clone, Copy)]
pub enum FontSizeClass {
    Small,
    Medium,
    Large,
    XLarge,
}

#[derive(Clone, Copy)]
pub enum PaddingClass {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
}

#[derive(Clone, Copy)]
pub enum SpacingClass {
    Tight,
    Normal,
    Relaxed,
    Loose,
}

// Convenience trait for easy responsive sizing access
pub trait ResponsiveExt {
    fn responsive(&self) -> &ResponsiveSize;
    
    // Shorthand methods for common operations
    fn w(&self, pct: f32) -> f32 { self.responsive().width_pct(pct) }
    fn h(&self, pct: f32) -> f32 { self.responsive().height_pct(pct) }
    fn font(&self, class: FontSizeClass) -> f32 { self.responsive().font_size(class) }
    fn pad(&self, class: PaddingClass) -> f32 { self.responsive().padding(class) }
    fn space(&self, class: SpacingClass) -> f32 { self.responsive().spacing(class) }
}

impl ResponsiveExt for ScreenLayout {
    fn responsive(&self) -> &ResponsiveSize {
        &self.responsive
    }
}

#[derive(Resource, Clone)]
pub struct ScreenLayout {
    pub window_size: Vec2,
    pub garden_area: Vec2,
    pub garden_center: Vec2,
    pub card_area_y: f32,
    pub responsive: ResponsiveSize,
}

impl Default for ScreenLayout {
    fn default() -> Self {
        let window_size = Vec2::new(800.0, 600.0);
        let responsive = ResponsiveSize::new(window_size);
        Self {
            window_size,
            garden_area: Vec2::new(window_size.x, window_size.y * 2.0 / 3.0),
            garden_center: Vec2::new(0.0, window_size.y / 6.0),
            card_area_y: -window_size.y * 0.4167, // Middle of bottom third (-41.67%)
            responsive,
        }
    }
}

impl ScreenLayout {
    pub fn update_for_window_size(&mut self, window_size: Vec2) {
        self.window_size = window_size;
        self.responsive = ResponsiveSize::new(window_size);
        self.garden_area = Vec2::new(window_size.x, self.responsive.height_pct(66.67)); // Top 2/3rds
        self.garden_center = Vec2::new(0.0, self.responsive.height_pct(16.67)); // Center of top 2/3rds
        
        // Position cards in the middle of the bottom third
        // Bottom third goes from -33.33% to -50% of screen height
        // So middle of bottom third is at -41.67% of screen height
        self.card_area_y = -self.responsive.height_pct(41.67);
    }
    
    pub fn calculate_card_size(&self, hand_size: usize) -> Vec2 {
        if hand_size == 0 {
            return Vec2::new(self.w(15.0), self.w(20.0)); // Default responsive size using shorthand
        }
        
        let card_area_width = self.w(80.0); // 80% of screen width available
        
        // Calculate card width based on available space and number of cards
        let max_card_width = card_area_width / hand_size as f32;
        let aspect_ratio = 2.0 / 3.0; // Standard card aspect ratio (width:height) - taller than wide
        
        // Try width-constrained sizing first
        let width_constrained_width = max_card_width * 0.8; // Leave some spacing between cards
        let width_constrained_height = width_constrained_width / aspect_ratio;
        
        // Try height-constrained sizing
        let height_constrained_height = self.h(20.0); // Max 20% of screen height using shorthand
        let height_constrained_width = height_constrained_height * aspect_ratio;
        
        // Use the smaller of the two to ensure cards fit
        if width_constrained_height <= height_constrained_height {
            Vec2::new(width_constrained_width, width_constrained_height)
        } else {
            Vec2::new(height_constrained_width, height_constrained_height)
        }
    }
    
    pub fn calculate_card_spacing(&self, hand_size: usize) -> f32 {
        if hand_size <= 1 {
            return 0.0;
        }
        
        let card_size = self.calculate_card_size(hand_size);
        let total_card_width = card_size.x * hand_size as f32;
        let available_width = self.w(80.0); // 80% of screen width using shorthand
        
        // Calculate spacing to center the cards
        let remaining_width = available_width - total_card_width;
        let spacing = remaining_width / (hand_size - 1) as f32;
        
        // Ensure minimum and maximum spacing using responsive sizing
        let min_spacing = self.space(SpacingClass::Tight);
        let max_spacing = self.space(SpacingClass::Relaxed);
        spacing.clamp(min_spacing, max_spacing)
    }
    
    pub fn resource_text_position(&self) -> Vec3 {
        Vec3::new(
            -self.garden_area.x / 2.0 + self.pad(PaddingClass::Large), // Left side with responsive padding using shorthand
            self.garden_center.y + self.garden_area.y / 4.0, // Upper area of garden
            10.0
        )
    }
    
    pub fn species_text_position(&self) -> Vec3 {
        Vec3::new(
            self.garden_area.x / 2.0 - self.pad(PaddingClass::Large), // Right side with responsive padding using shorthand
            self.garden_center.y + self.garden_area.y / 4.0, // Upper area of garden
            10.0
        )
    }
    
    // Helper methods for common UI measurements
    pub fn text_font_size(&self, class: FontSizeClass) -> f32 {
        self.responsive.font_size(class)
    }
    
    pub fn ui_padding(&self, class: PaddingClass) -> f32 {
        self.responsive.padding(class)
    }
    
    pub fn ui_spacing(&self, class: SpacingClass) -> f32 {
        self.responsive.spacing(class)
    }
}

// Simple garden state with resources and species
#[derive(Resource)]
pub struct GardenState {
    pub resources: HashMap<ResourceType, i32>,
    pub species: Vec<SpeciesInstance>,
}

#[derive(Clone, Debug)]
pub struct SpeciesInstance {
    pub species_type: SpeciesType,
    pub population: u32,
}

impl Default for GardenState {
    fn default() -> Self {
        let mut resources = HashMap::new();
        resources.insert(ResourceType::GroundWater, 5);
        resources.insert(ResourceType::Sunlight, 5);
        resources.insert(ResourceType::SoilNutrients, 5);
        resources.insert(ResourceType::CO2, 10);
        resources.insert(ResourceType::O2, 10);
        resources.insert(ResourceType::GreenVegetation, 0);
        resources.insert(ResourceType::Fruit, 0);
        resources.insert(ResourceType::DeadMatter, 0);
        resources.insert(ResourceType::PlantPopulation, 0);
        resources.insert(ResourceType::AnimalPopulation, 0);
        resources.insert(ResourceType::FungiPopulation, 0);
        
        Self {
            resources,
            species: Vec::new(),
        }
    }
}

impl GardenState {
    pub fn get_resource(&self, resource_type: ResourceType) -> i32 {
        self.resources.get(&resource_type).copied().unwrap_or(0)
    }
    
    pub fn modify_resource(&mut self, resource_type: ResourceType, change: i32) {
        let current = self.get_resource(resource_type);
        let new_value = (current + change).max(0); // Don't go below 0
        self.resources.insert(resource_type, new_value);
    }

    pub fn update_population_counters(&mut self) {
        let mut plant_pop = 0;
        let mut animal_pop = 0;
        let mut fungi_pop = 0;

        for instance in &self.species {
            match instance.species_type.kingdom() {
                Kingdom::Plant => plant_pop += instance.population,
                Kingdom::Animal => animal_pop += instance.population,
                Kingdom::Fungi => fungi_pop += instance.population,
            }
        }

        self.resources.insert(ResourceType::PlantPopulation, plant_pop as i32);
        self.resources.insert(ResourceType::AnimalPopulation, animal_pop as i32);
        self.resources.insert(ResourceType::FungiPopulation, fungi_pop as i32);
    }
    
    pub fn can_afford(&self, requirements: &HashMap<ResourceType, i32>) -> bool {
        for (resource_type, amount) in requirements {
            if self.get_resource(*resource_type) < *amount {
                return false;
            }
        }
        true
    }
    
    pub fn add_species(&mut self, species_type: SpeciesType) -> bool {
        let requirements = species_type.daily_consumption();
        
        if self.can_afford(&requirements) {
            // Pay the costs
            for (resource_type, amount) in requirements {
                self.modify_resource(resource_type, -amount);
            }
            
            // Add the species
            self.species.push(SpeciesInstance {
                species_type,
                population: 1,
            });
            
            // Apply the benefits
            let production = species_type.daily_production();
            for (resource_type, amount) in production {
                self.modify_resource(resource_type, amount);
            }

            // Update population counters
            self.update_population_counters();
            
            true
        } else {
            false
        }
    }
}

// Game state for cards
#[derive(Resource, Clone)]
pub struct GameState {
    pub deck: Vec<CardType>,
    pub hand: Vec<CardType>,
    pub selected_card_index: Option<usize>,
}

impl Default for GameState {
    fn default() -> Self {
        // Create a deck with all species cards
        let mut deck: Vec<CardType> = SpeciesType::all()
            .iter()
            .map(|&species| CardType::Species(species))
            .collect();
        
        // Randomize the deck
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
        
        Self {
            deck,
            hand: Vec::new(),
            selected_card_index: None,
        }
    }
}

impl GameState {
    pub fn draw_card(&mut self) -> Option<CardType> {
        if !self.deck.is_empty() {
            Some(self.deck.remove(0))
        } else {
            None
        }
    }
    
    pub fn draw_initial_hand(&mut self) {
        for _ in 0..5 {  // Draw 5 cards for initial hand
            if let Some(card) = self.draw_card() {
                self.hand.push(card);
            }
        }
    }
    
    pub fn play_card(&mut self, hand_index: usize) -> Option<CardType> {
        if hand_index < self.hand.len() {
            let played_card = self.hand.remove(hand_index);
            self.selected_card_index = None;
            
            // Draw a new card to replace the played one, if deck isn't empty
            if let Some(new_card) = self.draw_card() {
                self.hand.push(new_card);
            }
            
            Some(played_card)
        } else {
            None
        }
    }
    
    pub fn can_play_cards(&self) -> bool {
        !self.hand.is_empty()
    }

    pub fn shuffle_deck(&mut self) {
        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
    }
}
