use bevy::prelude::*;
use crate::core::{GardenState, GameState};
use crate::types::CardType;

/// Core game simulation logic - handles playing cards and game rules
pub fn handle_card_play(
    mut game_state: ResMut<GameState>,
    mut garden_state: ResMut<GardenState>,
    mut card_play_events: EventReader<CardPlayEvent>,
) {
    for event in card_play_events.read() {
        if event.hand_index < game_state.hand.len() {
            let played_card = game_state.hand[event.hand_index];
            
            // Try to add the species to the garden
            match played_card {
                CardType::Species(species_type) => {
                    if garden_state.add_species(species_type) {
                        // Successfully played - now remove from hand and draw new
                        game_state.play_card(event.hand_index);
                        println!("Played {} successfully!", played_card.name());
                    } else {
                        println!("Not enough resources to play {}!", played_card.name());
                    }
                }
            }
        }
    }
}

/// Event for when a card is played
#[derive(Event)]
pub struct CardPlayEvent {
    pub hand_index: usize,
}

/// Ecosystem simulation systems that could run automatically
pub fn simulate_ecosystem_step(
    _garden_state: ResMut<GardenState>,
    _time: Res<Time>,
) {
    // Future: Add daily/weekly simulation cycles
    // For now this is a placeholder for potential automatic ecosystem updates
}

/// Resource decay or growth over time
pub fn simulate_resource_changes(
    _garden_state: ResMut<GardenState>,
    _time: Res<Time>,
) {
    // Future: Add natural resource regeneration/decay
    // For now this is a placeholder
}
