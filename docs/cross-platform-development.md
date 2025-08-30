# Cross-Platform Development Guide

## Overview
Eden2 targets both native desktop (Windows/Linux/Mac) and WebAssembly (browser) platforms using Bevy 0.16. This document outlines critical considerations and common pitfalls when developing for both platforms simultaneously.

## Architecture Pattern
- **Native Entry Point**: `src/main.rs` - Desktop executable
- **Web Entry Point**: `src/lib.rs` - WASM module with `#[wasm_bindgen(start)]`
- **Shared Core**: Both entry points use the shared `create_app()` function to ensure identical configuration

## ✅ SOLUTION: Shared App Configuration

### Centralized System Registration
We've eliminated the duplication problem by creating a shared `create_app()` function in `lib.rs`:

```rust
// Shared app configuration - ONE SOURCE OF TRUTH
pub fn create_app(window_config: Window) -> App {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window_config),
        ..default()
    }))
    .add_systems(Startup, setup)
    .add_systems(Update, (
        handle_card_clicks,
        update_resource_display,
        update_species_display,
        update_hand_ui,
        update_card_visuals,
    ));
    
    app
}
```

### Platform-Specific Entry Points
Both entry points now use the shared configuration:

**Native (`main.rs`):**
```rust
use eden2::{create_app, native_window_config};

fn main() {
    create_app(native_window_config()).run();
}
```

**Web (`lib.rs`):**
```rust
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Eden2 starting...".into());
    create_app(web_window_config()).run();
}
```

### Benefits
- ✅ **Impossible to have mismatched system registration** - single source of truth
- ✅ **Reduced code duplication** - system lists only exist in one place
- ✅ **Platform-specific window configs** - while sharing core game logic
- ✅ **Future-proof** - adding new systems only requires one change

## Critical Rule: System Registration Consistency

### ⚠️ ~~ALWAYS keep system registration identical between both entry points~~ 
### ✅ NOW ENFORCED: Use the shared `create_app()` function for all builds

**Old Problem**: Different systems registered in `main.rs` vs `lib.rs` caused platform-specific bugs:
- Missing text rendering
- Duplicate UI elements
- Inconsistent game behavior

**New Solution**: The shared `create_app()` function makes it impossible to have mismatched systems.

## Text Rendering Requirements

### Use Text2d Consistently
**Problem**: UI nodes don't render properly in WASM builds
**Solution**: Always use `Text2d` for game text, never mix with UI node text

```rust
// ✅ CORRECT - Works on both platforms
commands.spawn((
    Text2d::new("Card Name"),
    TextFont { font_size: 14.0, ..default() },
    TextColor(Color::WHITE),
    Transform::from_translation(Vec3::new(x, y, z)),
    ComponentMarker,
));

// ❌ WRONG - Breaks in WASM
commands.spawn((
    Node { /* UI node properties */ },
    Text::new("Card Name"),
    // ... UI positioning
));
```

### Entity Lifecycle Management
**Problem**: Spawning new text entities without removing old ones creates duplicates
**Solution**: Always despawn old entities before spawning new ones

```rust
// ✅ CORRECT Pattern
pub fn update_display(
    mut commands: Commands,
    state: Res<GameState>,
    text_query: Query<Entity, With<DisplayText>>,
) {
    if state.is_changed() {
        // Remove old entities first
        for entity in text_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Then spawn new ones
        commands.spawn((/* new text */));
    }
}
```

## WASM-Specific Considerations

### Window/Canvas Configuration
```rust
#[cfg(target_arch = "wasm32")]
WindowPlugin {
    primary_window: Some(Window {
        canvas: Some("#bevy".to_string()),
        prevent_default_event_handling: false,
        ..default()
    }),
    ..default()
}
```

### Debugging
- Use `web_sys::console::log_1()` for WASM console output
- Add `console_error_panic_hook::set_once()` for better error messages
- Test both platforms with every text/UI change

## Development Workflow

### Testing Both Platforms
1. **Native Build**: `cargo run` - Fast iteration for gameplay logic
2. **Web Build**: `.\scripts\deploy\deploy.bat` - Full WASM deployment test
3. **Always test both** when making UI/text changes

### Common Debugging Steps
1. Check system registration in both `main.rs` and `lib.rs`
2. Verify Text2d usage (not UI nodes) for game text
3. Confirm entity despawn/spawn patterns
4. Test resource display updates by playing cards

## Historical Issues Fixed

### Issue: Card Text Missing in Web Build
**Cause**: `spawn_hand_ui` used UI nodes instead of Text2d
**Fix**: Changed to Text2d with world-space positioning

### Issue: Duplicate Garden Text
**Cause**: Multiple conflicting text update systems
**Fix**: Removed `update_garden_display`, kept `update_resource_display` and `update_species_display`

### Issue: System Registration Mismatch
**Cause**: Different systems in `main.rs` vs `lib.rs`
**Fix**: Standardized both to essential core systems only

## Checklist for Cross-Platform Changes

Before committing UI/text changes:
- [ ] All new systems added to `create_app()` function in `lib.rs` (automatically used by both platforms)
- [ ] All game text uses Text2d (not UI nodes)
- [ ] Entity despawn/spawn cycle implemented correctly
- [ ] Tested native build: `cargo run`
- [ ] Tested web build: `.\scripts\deploy\deploy.bat` + browser test
- [ ] Text appears correctly on cards in both platforms
- [ ] Resource/plant displays update without duplicates

## Emergency Debugging

If text is missing or duplicated:
1. ~~Check git diff between `main.rs` and `lib.rs` system registration~~ (No longer needed - using shared function)
2. Verify all systems are properly registered in `create_app()` function
3. Search for UI node usage: `grep -r "Node {" src/`
4. Verify Text2d usage: `grep -r "Text2d::new" src/`
5. Check for missing entity despawn in update systems

## Future Considerations

- ✅ **DONE**: Created shared system registration function to eliminate duplication
- Document any new WASM-specific workarounds discovered
- Add automated tests for text rendering consistency
- Monitor Bevy updates for cross-platform changes
- Consider extending shared config pattern to other game settings
