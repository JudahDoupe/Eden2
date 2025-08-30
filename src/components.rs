use bevy::prelude::*;
use crate::types::{ResourceType, CardType, PlantType};
use std::collections::HashMap;

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

// Simple garden state with just resources and plants
#[derive(Resource)]
pub struct GardenState {
    pub resources: HashMap<ResourceType, i32>,
    pub plants: Vec<PlantType>,
}

impl Default for GardenState {
    fn default() -> Self {
        let mut resources = HashMap::new();
        resources.insert(ResourceType::Water, 5);
        resources.insert(ResourceType::Sunlight, 5);
        resources.insert(ResourceType::Nutrients, 5);
        
        Self {
            resources,
            plants: Vec::new(),
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
    
    pub fn can_afford(&self, requirements: &HashMap<ResourceType, i32>) -> bool {
        for (resource_type, amount) in requirements {
            if self.get_resource(*resource_type) < *amount {
                return false;
            }
        }
        true
    }
    
    pub fn add_plant(&mut self, plant_type: PlantType) -> bool {
        let requirements = plant_type.required_resources();
        
        if self.can_afford(&requirements) {
            // Pay the costs
            for (resource_type, amount) in requirements {
                self.modify_resource(resource_type, -amount);
            }
            
            // Add the plant
            self.plants.push(plant_type);
            
            // Apply the benefits
            let production = plant_type.produced_resources();
            for (resource_type, amount) in production {
                self.modify_resource(resource_type, amount);
            }
            
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
        // Create a simple deck with 5 cards (will be drawn to start with 3 in hand, 2 in deck)
        let deck = vec![
            CardType::Plant(PlantType::Grass),
            CardType::Plant(PlantType::Flower),
            CardType::Plant(PlantType::Tree),
            CardType::Plant(PlantType::Bush),
            CardType::Plant(PlantType::Moss),
        ];
        
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
        for _ in 0..3 {
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
}
