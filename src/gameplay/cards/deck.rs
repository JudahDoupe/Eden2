use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use super::card::Card;
use crate::gameplay::species::plants::get_plant_tier_1;
use crate::gameplay::species::animals::get_animal_tier_1;
use crate::gameplay::species::fungi::get_fungi_tier_1;

/// Deck resource for managing the collection of cards available to draw from
#[derive(Resource, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::new();
        
        let tier_1_plants = get_plant_tier_1();
        let tier_1_animals = get_animal_tier_1();
        let tier_1_fungi = get_fungi_tier_1();
        
        for name in tier_1_plants.keys() {
            cards.push(Card::new(name));
        }
        
        for name in tier_1_animals.keys() {
            cards.push(Card::new(name));
        }
        
        for name in tier_1_fungi.keys() {
            cards.push(Card::new(name));
        }
        
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        
        Self {
            cards,
        }
    }
}

impl Deck {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn draw(&mut self) -> Option<Card> {
        if !self.cards.is_empty() {
            Some(self.cards.remove(0))
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn add_to_bottom(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_to_top(&mut self, card: Card) {
        self.cards.insert(0, card);
    }
}
