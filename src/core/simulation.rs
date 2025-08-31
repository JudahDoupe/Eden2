use bevy::prelude::*;
use crate::core::{GameState, AddSpeciesEvent};
use crate::types::Card;

/// Event for when a card is played
#[derive(Event)]
pub struct CardPlayEvent {
    pub hand_index: usize,
}

/// Handles card play events by validating resources and applying effects
pub fn handle_card_play(
    mut game_state: ResMut<GameState>,
    mut card_play_events: EventReader<CardPlayEvent>,
    mut add_species_events: EventWriter<AddSpeciesEvent>,
) {
    for event in card_play_events.read() {
        if let Some(played_card) = try_play_card(&mut game_state, &mut add_species_events, event.hand_index) {
            println!("Successfully played: {}", played_card.name());
        }
    }
}

/// Attempts to play a card from the hand, returns the played card if successful
fn try_play_card(
    game_state: &mut GameState,
    add_species_events: &mut EventWriter<AddSpeciesEvent>,
    hand_index: usize,
) -> Option<Card> {
    // Validate hand index
    if hand_index >= game_state.hand.len() {
        return None;
    }

    let card = game_state.hand[hand_index].clone();
    
    // Send event to add species - the simulation system will handle resource validation
    add_species_events.write(AddSpeciesEvent { card: card.clone() });
    
    // Remove card from hand regardless - the add species system will handle validation
    game_state.play_card(hand_index)
}
