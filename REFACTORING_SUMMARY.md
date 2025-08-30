# File Structure Refactoring Summary

## Overview
The Eden2 project has been refactored to separate UI/presentation logic from core game logic into a cleaner, more maintainable structure.

## New File Structure

### Core Game Logic (`src/core/`)
Contains all the business logic and game state management:

- **`src/core/mod.rs`** - Module exports
- **`src/core/game_state.rs`** - Card management, deck, hand operations
- **`src/core/garden_state.rs`** - Resource management, species management, ecosystem state
- **`src/core/simulation.rs`** - Game rules, card playing logic, ecosystem simulation
- **`src/core/systems/`** - Core game systems

### UI/Presentation Logic (`src/ui/`)
Contains all the visual elements, input handling, and layout management:

- **`src/ui/mod.rs`** - Module exports
- **`src/ui/components.rs`** - UI component markers (Card, CardSprite, ResourceDisplayText, SpeciesDisplayText, etc.)
- **`src/ui/layout.rs`** - Responsive sizing, screen layout management
- **`src/ui/systems/`** - UI-specific systems:
  - **`setup.rs`** - Initial UI setup and entity spawning
  - **`cards.rs`** - Card rendering, hand UI management
  - **`display.rs`** - Text display updates (resources, species)
  - **`input.rs`** - Mouse/touch input handling
  - **`layout.rs`** - Window resizing, responsive layout updates

### Unchanged
- **`src/types.rs`** - Core type definitions (ResourceType, SpeciesType, CardType)
- **`src/main.rs`** - Entry point for native builds
- **`src/lib.rs`** - App configuration (updated to use new structure)

## Key Improvements

### 1. **Separation of Concerns**
- **Core Logic**: Game rules, state management, ecosystem simulation
- **UI Logic**: Rendering, input handling, layout management

### 2. **Event-Driven Architecture**
- Added `CardPlayEvent` to decouple UI input from game logic
- UI systems emit events, core systems handle them

### 3. **Cleaner Dependencies**
- Core modules don't depend on UI modules
- UI modules can depend on core modules
- Clear separation between presentation and business logic

### 4. **Better Organization**
- Related functionality grouped together
- Easier to find and modify specific features
- More maintainable codebase

## Migration Summary

### Files Moved/Refactored:
- `components.rs` → Split into `core/` and `ui/` modules
- `systems/` → Split into `core/systems/` and `ui/systems/`
- Added event system for UI-to-core communication

### Files Removed:
- Old `src/components.rs`
- Old `src/systems/` directory

### Files Updated:
- `src/lib.rs` - Updated imports and system registration
- Added event handling for card playing

## Benefits

1. **Maintainability**: Easier to modify UI without affecting game logic
2. **Testability**: Core game logic can be tested independently
3. **Scalability**: New UI systems or game features can be added more easily
4. **Code Clarity**: Clear distinction between what affects game state vs. presentation
5. **Performance**: Potential for better optimization of UI vs. game logic systems

## Future Considerations

- The core systems could be further modularized (e.g., separate resource management, species management)
- UI systems could be split by feature (e.g., separate card UI, garden UI, menu UI)
- Consider adding a proper ECS event bus for more complex interactions
- Add unit tests for core game logic now that it's separated from UI
