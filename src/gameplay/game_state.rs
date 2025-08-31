use bevy::prelude::*;
use crate::gameplay::cards::{Deck, Hand};

const INITIAL_HAND_SIZE: usize = 3;

/// Core game state for managing available species, deck, and player hand
/// This represents what species the player can potentially add to their garden
#[derive(Resource, Clone)]
pub struct GameState {
    pub deck: Deck,
    pub hand: Hand,
}

impl Default for GameState {
    fn default() -> Self {
        let mut deck = Deck::new();
        let mut hand = Hand::new();
        
        for _ in 0..INITIAL_HAND_SIZE {
            if let Some(card) = deck.draw() {
                hand.add_card(card);
            }
        }
        
        Self {
            deck,
            hand,
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }
}
