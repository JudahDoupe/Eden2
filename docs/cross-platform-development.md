# Cross-Platform Development Guide

## Overview
Eden2 targets both native desktop (Windows/Linux/Mac) and WebAssembly (browser) platforms using Bevy 0.16. This document outlines critical considerations and common pitfalls when developing for both platforms simultaneously, with special focus on mobile-responsive design.

## Architecture Pattern
- **Native Entry Point**: `src/main.rs` - Desktop executable
- **Web Entry Point**: `src/lib.rs` - WASM module with `#[wasm_bindgen(start)]`
- **Shared Core**: Both entry points use the shared `create_app()` function to ensure identical configuration
- **Responsive Design**: Centralized sizing system for mobile compatibility

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
    .add_systems(Startup, (setup, initialize_screen_layout).chain())
    .add_systems(Update, (
        handle_window_resize,
        handle_card_clicks,
        update_resource_display,
        update_species_display,
        update_hand_ui,
        update_hand_layout,
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
- ✅ **Mobile-responsive design** - automatic sizing for all screen types
- ✅ **Future-proof** - adding new systems only requires one change

## 📱 Mobile-Responsive Design System

### Centralized Responsive Sizing
**Problem**: Hardcoded pixel values break on mobile screens
**Solution**: `ResponsiveSize` utility for screen-relative measurements

```rust
// ❌ OLD WAY - Hardcoded pixels
font_size: 16.0,
custom_size: Some(Vec2::new(120.0, 160.0)),
Transform::from_translation(Vec3::new(-250.0, 150.0, 10.0))

// ✅ NEW WAY - Responsive sizing
font_size: screen_layout.font(FontSizeClass::Medium),
custom_size: Some(Vec2::new(screen_layout.w(15.0), screen_layout.h(20.0))),
Transform::from_translation(screen_layout.resource_text_position())
```

### Semantic Sizing Classes
```rust
// Font sizes that scale with screen
FontSizeClass::Small | Medium | Large | XLarge

// Padding that scales with screen 
PaddingClass::XSmall | Small | Medium | Large | XLarge

// Spacing between elements
SpacingClass::Tight | Normal | Relaxed | Loose
```

### Responsive Layout Structure
- **Top 2/3rds**: Garden area (full width)
- **Bottom 1/3rd**: Card hand (responsive sizing and spacing)
- **Text anchoring**: Resource/species text anchored to garden sides with responsive padding

### Dynamic Card Sizing Algorithm
```rust
pub fn calculate_card_size(&self, hand_size: usize) -> Vec2 {
    // Cards scale based on:
    // - Available screen width (80% for cards)
    // - Number of cards in hand
    // - Maximum height constraint (20% of screen)
    // - Maintains 2:3 aspect ratio (width:height) - standard card proportions
}
```

## 🔧 Entity Relationship Patterns

### Parent-Child Text Anchoring
**Problem**: Card text positioning becomes misaligned during window resize
**Solution**: Make text a child entity of the card sprite

```rust
// ✅ ROBUST - Text automatically follows parent card
let card_entity = commands.spawn((
    Sprite { /* card sprite */ },
    Transform::from_translation(Vec3::new(x_position, card_y, 1.0)),
    Card { /* card data */ },
    CardSprite,
)).id();

let text_entity = commands.spawn((
    Text2d::new(card_type.name()),
    Transform::from_translation(Vec3::new(0.0, card_size.y * 0.3, 1.0)), // Relative to parent
    CardText,
)).id();

commands.entity(card_entity).add_child(text_entity);
```

### Benefits of Parent-Child Anchoring
- **Always perfectly aligned** - text physically anchored to card
- **No timing issues** - works regardless of initialization order
- **Automatic cleanup** - `despawn()` on parent removes children
- **Performance optimized** - fewer position calculations needed

## ⚡ Initialization Timing & Window Detection

### Critical Initialization Order
**Problem**: Window dimensions not available during startup, causing layout issues
**Solution**: Robust window detection with fallback handling

```rust
// ✅ SAFE - Handles missing/invalid window gracefully
pub fn handle_window_resize(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut layout_initialized: ResMut<LayoutInitialized>,
    mut screen_layout: ResMut<ScreenLayout>,
) {
    // Handle both resize events AND initial window detection
    if !layout_initialized.0 {
        if let Some(window) = window_query.iter().next() {
            let window_size = Vec2::new(window.width(), window.height());
            if window_size.x > 0.0 && window_size.y > 0.0 && window_size != Vec2::new(800.0, 600.0) {
                screen_layout.update_for_window_size(window_size);
                layout_initialized.0 = true;
            }
        }
    }
}
```

### Web-Specific Window Timing
- **`fit_canvas_to_parent: true`**: Window size not immediately available
- **Startup vs Update**: Window queries work better in Update systems
- **Fallback layout**: Always provide sensible defaults for startup

### Startup System Chain
```rust
// ✅ CORRECT - Initialize layout after setup but handle gracefully
.add_systems(Startup, (setup, initialize_screen_layout).chain())
.add_systems(Update, (handle_window_resize, /* other systems */))
```

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

### Issue: Card Text Positioning Bug on Load
**Cause**: Cards spawned with default layout before window size initialization
**Fix**: Parent-child entity relationships + robust window detection
**Lesson**: Entity hierarchies solve timing-dependent positioning issues

### Issue: Cards Not in Bottom Third on Mobile
**Cause**: Hardcoded card Y position didn't account for tall mobile screens
**Fix**: Dynamic positioning using `height_pct(41.67)` for bottom third center
**Lesson**: Test with actual mobile resolutions (450x1000), not just desktop

### Issue: Card Text Missing in Web Build
**Cause**: `spawn_hand_ui` used UI nodes instead of Text2d
**Fix**: Changed to Text2d with world-space positioning
**Lesson**: Text2d is the reliable cross-platform text solution

### Issue: Everything Broken After Parent-Child Implementation
**Cause**: Window initialization order changed, breaking startup sequence
**Fix**: Combined resize handler with initialization detection in Update systems
**Lesson**: Window queries are unreliable during Startup, use Update systems

### Issue: Duplicate Garden Text
**Cause**: Multiple conflicting text update systems
**Fix**: Removed `update_garden_display`, kept `update_resource_display` and `update_species_display`

### Issue: System Registration Mismatch
**Cause**: Different systems in `main.rs` vs `lib.rs`
**Fix**: Standardized both to shared `create_app()` function
**Lesson**: Single source of truth prevents divergence

## 📱 Mobile Development Best Practices

### Screen Layout Testing
- **Test resolutions**: 450x1000 (mobile portrait), 800x600 (desktop), 1920x1080 (large)
- **Orientation changes**: Ensure layout updates on device rotation
- **Touch targets**: Minimum 44px touch targets (use responsive sizing)

### Responsive Design Checklist
- [ ] No hardcoded pixel values (use `screen_layout.w()`, `screen_layout.h()`)
- [ ] Text uses semantic size classes (`FontSizeClass::Medium`)
- [ ] Spacing uses responsive classes (`SpacingClass::Normal`)
- [ ] Cards scale dynamically with hand size
- [ ] Touch targets are adequately sized
- [ ] Layout works in portrait and landscape

### Performance Considerations
- **Parent-child relationships**: More efficient than manual position sync
- **Responsive calculations**: Cached in ScreenLayout, not recalculated per frame
- **Entity cleanup**: Always use `despawn()` (automatically recursive in Bevy 0.16)

## Checklist for Cross-Platform Changes

Before committing UI/text changes:
- [ ] All new systems added to `create_app()` function in `lib.rs` (automatically used by both platforms)
- [ ] All game text uses Text2d (not UI nodes)
- [ ] All pixel values use responsive sizing (`screen_layout.w()`, `screen_layout.h()`, etc.)
- [ ] Entity parent-child relationships used for anchored elements
- [ ] Entity despawn/spawn cycle implemented correctly
- [ ] Window resize handling works properly
- [ ] Tested native build: `cargo run`
- [ ] Tested web build: `.\scripts\deploy\deploy.bat` + browser test
- [ ] Tested mobile resolution: Change native config to (450, 1000)
- [ ] Text appears correctly on cards in both platforms
- [ ] Resource/plant displays update without duplicates
- [ ] Cards properly positioned in bottom third
- [ ] Layout responsive to window resizing

## Emergency Debugging

If text is missing or duplicated:
1. Verify all systems are properly registered in `create_app()` function
2. Check for hardcoded pixel values: `grep -r "Vec2::new([0-9]" src/`
3. Search for UI node usage: `grep -r "Node {" src/`
4. Verify Text2d usage: `grep -r "Text2d::new" src/`
5. Check for missing entity despawn in update systems
6. Verify parent-child relationships for anchored elements
7. Test window resize events: Manually resize browser window
8. Check initialization order: Look for startup vs update system conflicts

## Future Considerations

- ✅ **DONE**: Created shared system registration function to eliminate duplication
- ✅ **DONE**: Implemented responsive sizing system for mobile compatibility
- ✅ **DONE**: Added parent-child entity anchoring for robust positioning
- ✅ **DONE**: Solved window initialization timing issues
- Document any new mobile-specific patterns discovered
- Add automated tests for responsive layout consistency
- Monitor Bevy updates for cross-platform changes
- Consider extending responsive sizing to more UI elements
- Add support for different mobile screen densities (DPI scaling)
