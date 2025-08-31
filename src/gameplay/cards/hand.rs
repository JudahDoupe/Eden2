use bevy::prelude::*;
use super::{card::Card, deck::Deck};

const INITIAL_HAND_SIZE: usize = 5;

/// Hand resource for managing the player's current cards
#[derive(Resource, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub selected_card_index: Option<usize>,
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            cards: Vec::new(),
            selected_card_index: None,
        }
    }
}

impl Hand {
    /// Create a new empty hand
    pub fn new() -> Self {
        Self::default()
    }

    /// Draw the initial hand from the deck
    pub fn draw_initial_hand(&mut self, deck: &mut Deck) {
        for _ in 0..INITIAL_HAND_SIZE {
            if let Some(card) = deck.draw() {
                self.cards.push(card);
            }
        }
    }

    /// Add a card to the hand
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Play a card from the hand at the given index, optionally replacing it with a new card from deck
    pub fn play_card(&mut self, hand_index: usize, deck: &mut Option<&mut Deck>) -> Option<Card> {
        if hand_index >= self.cards.len() {
            return None;
        }

        let played_card = self.cards.remove(hand_index);
        self.selected_card_index = None;
        
        // Replace the played card with a new one from the deck if deck is provided
        if let Some(ref mut deck) = deck {
            if let Some(new_card) = deck.draw() {
                self.cards.push(new_card);
            }
        }
        
        Some(played_card)
    }

    /// Check if the hand has any cards
    pub fn can_play_card(&self) -> bool {
        !self.cards.is_empty()
    }

    /// Get the number of cards in hand
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Check if the hand is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Get a reference to the card at the given index
    pub fn get_card(&self, index: usize) -> Option<&Card> {
        self.cards.get(index)
    }

    /// Remove a card at the given index without replacement
    pub fn remove_card(&mut self, index: usize) -> Option<Card> {
        if index < self.cards.len() {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    /// Get an iterator over the cards in the hand
    pub fn iter(&self) -> std::slice::Iter<'_, Card> {
        self.cards.iter()
    }
}
