use bevy::prelude::*;
use super::{card::Card};
use crate::gameplay::lifecycle::{AddSpeciesToEcosystemEvent, SimulateDayEvent};
use crate::gameplay::species::get_species;

/// Hand resource for managing the player's current cards
#[derive(Resource, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            cards: Vec::new(),
        }
    }
}

impl Hand {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn remove_card(&mut self, index: usize) -> Option<Card> {
        if index < self.cards.len() {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    pub fn get_card(&self, index: usize) -> Option<&Card> {
        self.cards.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Card> {
        self.cards.iter()
    }
}

// ===== EVENTS =====

#[derive(Event)]
pub struct PlayCardEvent {
    pub hand_index: usize,
}

#[derive(Event)]
pub struct DiscardCardEvent {
    pub hand_index: usize,
}

// ===== SYSTEMS =====

pub fn handle_play_card_event(
    mut game_state: ResMut<crate::gameplay::GameState>,
    mut play_card_events: EventReader<PlayCardEvent>,
    mut add_species_events: EventWriter<AddSpeciesToEcosystemEvent>,
) {
    for event in play_card_events.read() {
        let Some(card) = game_state.hand.get_card(event.hand_index) else {
            println!("Card not found at index: {}", event.hand_index);
            continue;
        };
        let card_clone = card.clone();

        let Some(species_def) = get_species(card_clone.name()) else {
            println!("Species definition not found for card: {}", card_clone.name());
            continue;
        };

        // Remove card from hand
        game_state.hand.remove_card(event.hand_index);

        // Draw replacement card
        if let Some(new_card) = game_state.deck.draw() {
            game_state.hand.add_card(new_card);
        }

        // Add species to ecosystem with appropriate starting biomass
        let starting_biomass = match species_def.biomass_composition {
            crate::gameplay::species::BiomassComposition::Plant => (3, 0), // Start with 3 plant matter
            crate::gameplay::species::BiomassComposition::Animal => (0, 2), // Start with 2 animal matter
            crate::gameplay::species::BiomassComposition::Mixed { plant_ratio, animal_ratio } => {
                // For mixed compositions, split the total biomass proportionally
                let total_biomass = 3.0;
                let plant_amount = (total_biomass * plant_ratio) as u32;
                let animal_amount = (total_biomass * animal_ratio) as u32;
                (plant_amount, animal_amount)
            }
        };

        add_species_events.write(AddSpeciesToEcosystemEvent { 
            species: species_def.clone(),
            starting_biomass,
        });
    }
}

pub fn handle_discard_card_event(
    mut game_state: ResMut<crate::gameplay::GameState>,
    mut discard_events: EventReader<DiscardCardEvent>,
    mut simulate_day_events: EventWriter<SimulateDayEvent>,
) {
    for event in discard_events.read() {
        // Remove the card from hand
        if game_state.hand.remove_card(event.hand_index).is_some() {
            // Draw a new card to replace it
            if let Some(new_card) = game_state.deck.draw() {
                game_state.hand.add_card(new_card);
            }
            
            // Simulate a day passing
            simulate_day_events.write(SimulateDayEvent);
        }
    }
}
