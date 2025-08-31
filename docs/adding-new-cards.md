# Adding New Cards to Eden2

With the new card definition system, adding a new card is now incredibly simple - you only need to edit **one place**!

## How to Add a New Card

1. Open `src/core/card_definitions.rs`
2. Add your new card to the `get_all_card_definitions()` function
3. Add the card name to the `get_all_card_names()` function in the appropriate position

That's it! The old enum system has been removed and cards are now identified by their string names directly.

## Example: Adding a "Mushroom Cloud" Fungi Card

Here's how to add a new card called "Mushroom Cloud" that unlocks in round 2:

### Step 1: Add to card definitions (src/core/card_definitions.rs)

In the `get_all_card_definitions()` function, add this anywhere in the fungi section:

```rust
cards.insert("Mushroom Cloud", 
    CardDefinition::new("Mushroom Cloud", Kingdom::Fungi, 2, 5, Color::srgb(0.6, 0.6, 0.7))
        .survival_requirement(ResourceType::DeadMatter, 2, 7)
        .survival_requirement(ResourceType::GroundWater, 1, 5)
        .survival_requirement(ResourceType::O2, 0, 3)
        .consumes(ResourceType::DeadMatter, 2)
        .consumes(ResourceType::GroundWater, 1)
        .produces(ResourceType::CO2, 2)
        .produces(ResourceType::SoilNutrients, 3)
);
```

### Step 2: Add to card name list (src/core/card_definitions.rs)

In the `get_all_card_names()` function, add `"Mushroom Cloud"` to the appropriate position in the fungi section.

## Card Definition Builder Pattern

The `CardDefinition` uses a builder pattern for easy construction:

```rust
CardDefinition::new(name, kingdom, unlock_round, max_population, color)
    .survival_requirement(resource, min, max)  // Add as many as needed
    .consumes(resource, amount)                // Add as many as needed  
    .produces(resource, amount)                // Add as many as needed
```

### Available Methods:

- `survival_requirement(resource, min, max)` - Sets the min/max range for a resource the species needs to survive
- `consumes(resource, amount)` - Sets how much of a resource this species consumes daily
- `produces(resource, amount)` - Sets how much of a resource this species produces daily

You can chain as many of these as needed to define the complete behavior of your card.

## Benefits of This System

✅ **Single source of truth** - All card data in one place  
✅ **Easy to add cards** - Just one location to edit  
✅ **Type safe** - Rust compiler catches errors  
✅ **Readable** - Clear builder pattern syntax  
✅ **Maintainable** - No more hunting through huge match statements  
✅ **Consistent** - All cards follow the same pattern  

## Old vs New

### Before (required editing ~10 different match statements):
- Add enum variant
- Add to `name()` match  
- Add to `kingdom()` match
- Add to `unlock_round()` match
- Add to `max_population()` match  
- Add to `survival_requirements()` match (complex)
- Add to `daily_consumption()` match (complex)
- Add to `daily_production()` match (complex)
- Add to `color()` match
- Add to `all()` vec

### After (edit 1 location):
- Add one entry to `get_all_card_definitions()`
- Add minimal enum plumbing (3 simple additions)

The new system reduces the complexity from ~200 lines of boilerplate per card to ~10 lines of declarative data!
