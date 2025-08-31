pub mod cards;
pub mod game_state;
pub mod garden;
pub mod species;

// Re-export specific items to avoid conflicts
pub use cards::{Card, Deck, Hand, PlayCardEvent, handle_play_card_event};
pub use game_state::GameState;
pub use garden::{Garden, ResourceType, GardenResources, AddSpeciesToGardenEvent, SpeciesDeathEvent, SimulateDayEvent, handle_add_species_to_garden_event, trigger_simulation_on_species_play, run_daily_simulation};
pub use species::{SpeciesInstance};
