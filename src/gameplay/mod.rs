pub mod cards;
pub mod game_state;
pub mod species;
pub mod lifecycle;

// Re-export specific items to avoid conflicts
pub use cards::{Card, Deck, Hand, PlayCardEvent, DiscardCardEvent, handle_play_card_event, handle_discard_card_event};
pub use game_state::GameState;
pub use species::{Species, Kingdom, BiomassConversion};
pub use lifecycle::{EcosystemPopulation, IndividualCreature, DailySimulation};
