use bevy::prelude::*;
use crate::types::{CardType, SpeciesType};
use rand::seq::SliceRandom;
use rand::thread_rng;

const INITIAL_HAND_SIZE: usize = 5;

/// Core game state for managing cards, deck, and player hand
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
    
    /// Draws the initial hand of cards for the player
    pub fn draw_initial_hand(&mut self) {
        for _ in 0..INITIAL_HAND_SIZE {
            if let Some(card) = self.draw_card() {
                self.hand.push(card);
            }
        }
    }
    
    /// Plays a card from the hand at the given index, replacing it with a new card from deck
    pub fn play_card(&mut self, hand_index: usize) -> Option<CardType> {
        if hand_index >= self.hand.len() {
            return None;
        }

        let played_card = self.hand.remove(hand_index);
        self.selected_card_index = None;
        
        // Replace the played card with a new one from the deck
        if let Some(new_card) = self.draw_card() {
            self.hand.push(new_card);
        }
        
        Some(played_card)
    }
    
    pub fn can_play_cards(&self) -> bool {
        !self.hand.is_empty()
    }

    pub fn shuffle_deck(&mut self) {
        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
    }
}
