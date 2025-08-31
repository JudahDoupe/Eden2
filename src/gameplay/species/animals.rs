use bevy::prelude::*;
use super::{Kingdom, Species};
use std::collections::HashMap;

pub fn get_animal_tier_1() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Rabbits", 
        Species::new("Rabbits", Kingdom::Animal, 1, 4, Color::srgb(0.6, 0.5, 0.4))
    );

    species.insert("Earthworms", 
        Species::new("Earthworms", Kingdom::Animal, 1, 8, Color::srgb(0.5, 0.3, 0.2))
    );

    species
}

pub fn get_animal_tier_2() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Honeybees", 
        Species::new("Honeybees", Kingdom::Animal, 2, 6, Color::srgb(1.0, 0.8, 0.2))
    );

    species.insert("Field Mice", 
        Species::new("Field Mice", Kingdom::Animal, 2, 5, Color::srgb(0.4, 0.3, 0.2))
    );

    species.insert("Butterflies", 
        Species::new("Butterflies", Kingdom::Animal, 2, 6, Color::srgb(0.8, 0.6, 0.9))
    );

    species.insert("Snails", 
        Species::new("Snails", Kingdom::Animal, 2, 6, Color::srgb(0.7, 0.6, 0.5))
    );

    species
}

pub fn get_animal_tier_3() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Ladybugs", 
        Species::new("Ladybugs", Kingdom::Animal, 3, 4, Color::srgb(0.8, 0.2, 0.2))
    );

    species.insert("Frogs", 
        Species::new("Frogs", Kingdom::Animal, 3, 3, Color::srgb(0.2, 0.6, 0.3))
    );

    species
}

pub fn get_animal_tier_4() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Squirrels", 
        Species::new("Squirrels", Kingdom::Animal, 4, 3, Color::srgb(0.6, 0.4, 0.2))
    );

    species.insert("Birds", 
        Species::new("Birds", Kingdom::Animal, 4, 4, Color::srgb(0.4, 0.6, 0.8))
    );

    species
}

pub fn get_all_animal_species() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();
    
    species.extend(get_animal_tier_1());
    species.extend(get_animal_tier_2());
    species.extend(get_animal_tier_3());
    species.extend(get_animal_tier_4());
    
    species
}
