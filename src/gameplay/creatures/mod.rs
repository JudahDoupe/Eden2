pub mod individual {
    use std::collections::HashMap;

    /// Unique ID for each creature
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CreatureId(pub u64);

    /// Represents an individual creature in the ecosystem
    #[derive(Debug, Clone)]
    pub struct IndividualCreature {
        pub id: CreatureId,
        pub species_id: String,
        pub age_days: u32,
        pub biomass: CreatureBiomass,
    }

    /// Creature biomass composition
    #[derive(Clone, Debug)]
    pub struct CreatureBiomass {
        pub plant_matter: u32,
        pub animal_matter: u32,
    }
}
