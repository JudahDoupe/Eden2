use bevy::prelude::*;
use super::{Kingdom, Species};
use std::collections::HashMap;

pub fn get_fungi_tier_1() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Rot Fungi", 
        Species::new("Rot Fungi", Kingdom::Fungi, 1, 6, Color::srgb(0.4, 0.3, 0.2))
    );

    species.insert("Mold Clusters", 
        Species::new("Mold Clusters", Kingdom::Fungi, 1, 8, Color::srgb(0.3, 0.5, 0.3))
    );

    species
}

pub fn get_fungi_tier_2() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Puffballs", 
        Species::new("Puffballs", Kingdom::Fungi, 2, 4, Color::srgb(0.8, 0.8, 0.7))
    );

    species.insert("Mycorrhizal Fungi", 
        Species::new("Mycorrhizal Fungi", Kingdom::Fungi, 2, 5, Color::srgb(0.5, 0.4, 0.3))
    );

    species.insert("Yeast Colonies", 
        Species::new("Yeast Colonies", Kingdom::Fungi, 2, 10, Color::srgb(0.7, 0.7, 0.6))
    );

    species
}

pub fn get_fungi_tier_3() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Giant Mushrooms", 
        Species::new("Giant Mushrooms", Kingdom::Fungi, 3, 2, Color::srgb(0.6, 0.4, 0.3))
    );

    species.insert("Shelf Fungi", 
        Species::new("Shelf Fungi", Kingdom::Fungi, 3, 4, Color::srgb(0.7, 0.5, 0.3))
    );

    species.insert("Coral Fungi", 
        Species::new("Coral Fungi", Kingdom::Fungi, 3, 4, Color::srgb(0.9, 0.7, 0.5))
    );

    species
}

pub fn get_fungi_tier_4() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();

    species.insert("Truffle Fungi", 
        Species::new("Truffle Fungi", Kingdom::Fungi, 4, 3, Color::srgb(0.3, 0.2, 0.1))
    );

    species.insert("Slime Molds", 
        Species::new("Slime Molds", Kingdom::Fungi, 4, 3, Color::srgb(0.6, 0.8, 0.4))
    );

    species
}

pub fn get_all_fungi_species() -> HashMap<&'static str, Species> {
    let mut species = HashMap::new();
    
    species.extend(get_fungi_tier_1());
    species.extend(get_fungi_tier_2());
    species.extend(get_fungi_tier_3());
    species.extend(get_fungi_tier_4());
    
    species
}
