use bevy::prelude::*;
use crate::types::{TileType, CardType};

#[derive(Component)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub tile_type: TileType,
    pub is_hovered: bool,
}

#[derive(Component)]
pub struct TileSprite;

#[derive(Component)]
pub struct TileBorder;

#[derive(Component)]
pub struct StatsText;

#[derive(Component)]
pub struct Card {
    pub card_type: CardType,
    pub hand_index: usize,
    pub is_selected: bool,
    pub is_hovered: bool,
}

#[derive(Component)]
pub struct CardSprite;

#[derive(Component)]
pub struct CardText;

#[derive(Component)]
pub struct HandUI;

#[derive(Resource)]
pub struct GameState {
    pub deck: Vec<CardType>,
    pub hand: Vec<CardType>,
    pub selected_card_index: Option<usize>,
}

impl Default for GameState {
    fn default() -> Self {
        // Create a deck with multiple copies of each card type
        let mut deck = Vec::new();
        for _ in 0..5 {
            deck.push(CardType::PlantSeed);
            deck.push(CardType::Irrigate);
            deck.push(CardType::GrowForest);
            deck.push(CardType::ClearLand);
        }
        
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
            let index = fastrand::usize(..self.deck.len());
            Some(self.deck.remove(index))
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
            } else {
                // If deck is empty, game ends gracefully
                println!("Deck is empty! No more cards to draw.");
            }
            
            Some(played_card)
        } else {
            None
        }
    }
    
    pub fn has_cards(&self) -> bool {
        !self.hand.is_empty() || !self.deck.is_empty()
    }
    
    pub fn can_play_cards(&self) -> bool {
        !self.hand.is_empty()
    }
}
