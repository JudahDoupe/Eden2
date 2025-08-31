# Eden2 Project Structure

## Overview

Eden2 is an ecosystem card game built with Rust and Bevy. The project follows a strict organizational philosophy designed to create clean, maintainable code with clear separation of concerns.

## Core Organizational Principles

### 1. Two-Tier Architecture

The codebase is divided into two main tiers with a unidirectional dependency flow:

```
gameplay/ (core logic) ← visualization/ (UI/rendering)
```

- **`gameplay/`** - Contains all core game mechanics, rules, and simulation logic
  - Can run independently without any visual components
  - Pure game logic and state management
  - No dependencies on visualization code

- **`visualization/`** - Contains all rendering, UI, and visual presentation code
  - Depends on gameplay but gameplay doesn't depend on visualization
  - Handles responsive sizing, user interaction, and visual updates
  - All UI components and display systems

### 2. Domain-Driven File Organization

Within each tier, code is organized by **domain concepts** where all related functionality is grouped together. Each domain represents a cohesive area of functionality.

### 3. Single-File Domain Principle

Each domain concept follows the principle of keeping all related code together in a single file:

- **Entity creation** (spawning/initialization)
- **Components** (data structures)
- **Systems** (logic and interactions)
- **Events** (communication)
- **Related utilities and helpers**

This ensures that when working on a specific concept, all relevant code is in one place.

## Directory Structure

```
src/
├── lib.rs                          # App configuration and system coordination
├── main.rs                         # Native entry point
│
├── gameplay/                       # Core game logic (Tier 1)
│   ├── mod.rs                      # Module exports
│   ├── [top level files]            
│   ├── [domain concept]/                   
│       └── [domain-related files]   
│
└── visualization/                  # UI and rendering (Tier 2)
    ├── mod.rs                      # Module exports
    ├── [top level files]            
    ├── [domain concept]/                   
        └── [domain-related files]   
```


## Key Design Benefits

### 1. Maintainability
- All related code is co-located by domain
- Easy to find and modify specific functionality
- Clear boundaries between different areas of the system

### 2. Testability
- Gameplay logic is completely independent of visualization
- Can test game mechanics without UI dependencies
- Clear interfaces between domains

### 3. Modularity
- Each domain can be understood in isolation
- New features fit naturally into existing domain boundaries
- Easy to extend or modify specific areas

### 4. Scalability
- New domains can be added following the same patterns
- Clear places to add new functionality
- Consistent organization as the project grows

## File Organization Rules

### DO:
✅ Keep all related functionality in the same file
✅ Organize by domain concept, not by technical type
✅ Maintain unidirectional dependencies (visualization → gameplay)
✅ Place entity creation, components, systems, and events together
✅ Use clear, descriptive names that reflect the domain

### DON'T:
❌ Split a single domain concept across multiple files
❌ Mix visualization logic in gameplay files
❌ Create circular dependencies between domains
❌ Organize files by technical categories (all components together, etc.)
❌ Put unrelated functionality in the same file just because it's similar code

## Working with the Structure

### Adding New Features
1. **Identify the domain** - Determine which conceptual area the feature belongs to
2. **Follow single-file principle** - Add all related code (components, systems, events) to the appropriate domain file(s)
3. **Respect tier boundaries** - Ensure gameplay features don't depend on visualization
4. **Update module exports** - Add new functionality to the appropriate mod.rs files

### Modifying Existing Features
1. **Locate the domain** - Find the domain that contains the feature
2. **Keep changes co-located** - All related changes should be in the same domain
3. **Maintain separation** - Ensure gameplay/visualization boundaries remain clear
4. **Consider domain expansion** - If a domain grows too large, consider splitting into focused sub-domains

### Understanding the Codebase
1. **Start with domains** - Begin by understanding the high-level domain responsibilities
2. **Follow dependency flow** - Understand that visualization depends on gameplay, not vice versa
3. **Look for co-location** - Related functionality is grouped together by concept
4. **Respect boundaries** - Each domain has clear responsibilities and interfaces

This organization ensures that Eden2 remains maintainable, testable, and scalable as it grows in complexity.
