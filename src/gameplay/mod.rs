pub mod cards;
pub mod game_state;
pub mod garden;
pub mod species;

// Re-export specific items to avoid conflicts
pub use cards::{Card, Deck, Hand};
pub use game_state::GameState;
pub use garden::{GardenState, ResourceType, GardenResources};
pub use species::{SpeciesInstance, SpeciesCollection, AddSpeciesEvent};
pub use garden::{PlayCardEvent, SimulateDayEvent};
pub use garden::{handle_species_play, handle_add_species, trigger_simulation_on_species_play, run_daily_simulation};
