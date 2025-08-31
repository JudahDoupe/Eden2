use bevy::prelude::*;
use crate::gameplay::cards::{Card, Deck, Hand};

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
        
        // Draw initial hand
        hand.draw_initial_hand(&mut deck);
        
        Self {
            deck,
            hand,
        }
    }
}

impl GameState {
    /// Create a new game state with an initialized deck and starting hand
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn draw_species(&mut self) -> Option<Card> {
        self.deck.draw()
    }
    
    /// Draws the initial hand of species for the player
    pub fn draw_initial_hand(&mut self) {
        self.hand.draw_initial_hand(&mut self.deck);
    }
    
    /// Plays a species from the hand at the given index, replacing it with a new species from deck
    pub fn play_species(&mut self, hand_index: usize) -> Option<Card> {
        self.hand.play_card(hand_index, &mut Some(&mut self.deck))
    }
    
    pub fn can_play_species(&self) -> bool {
        self.hand.can_play_card()
    }

    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle();
    }

    // Backward compatibility accessors for selected_species_index
    pub fn selected_species_index(&self) -> Option<usize> {
        self.hand.selected_card_index
    }

    pub fn set_selected_species_index(&mut self, index: Option<usize>) {
        self.hand.selected_card_index = index;
    }
}
