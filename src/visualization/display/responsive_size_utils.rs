use bevy::prelude::*;

/// Responsive sizing utility for UI elements
#[derive(Resource, Clone)]
pub struct ResponsiveSize {
    pub window_size: Vec2,
}

impl ResponsiveSize {
    pub fn new(window_size: Vec2) -> Self {
        Self { window_size }
    }
    
    // Width-based sizing (percentage of screen width)
    pub fn width_pct(&self, percentage: f32) -> f32 {
        self.window_size.x * (percentage / 100.0)
    }
    
    // Height-based sizing (percentage of screen height)
    pub fn height_pct(&self, percentage: f32) -> f32 {
        self.window_size.y * (percentage / 100.0)
    }
    
    // Minimum dimension based sizing (useful for square elements)
    pub fn min_pct(&self, percentage: f32) -> f32 {
        self.window_size.x.min(self.window_size.y) * (percentage / 100.0)
    }
    
    // Maximum dimension based sizing
    pub fn max_pct(&self, percentage: f32) -> f32 {
        self.window_size.x.max(self.window_size.y) * (percentage / 100.0)
    }
    
    // Responsive font size based on screen dimensions
    pub fn font_size(&self, size_class: FontSizeClass) -> f32 {
        let base_size = match size_class {
            FontSizeClass::Small => self.min_pct(2.0),    // 2% of smallest dimension
            FontSizeClass::Medium => self.min_pct(2.5),   // 2.5% of smallest dimension
            FontSizeClass::Large => self.min_pct(3.0),    // 3% of smallest dimension
            FontSizeClass::XLarge => self.min_pct(4.0),   // 4% of smallest dimension
        };
        base_size.clamp(10.0, 24.0) // Ensure readable range
    }
    
    // Responsive padding/margin
    pub fn padding(&self, padding_class: PaddingClass) -> f32 {
        match padding_class {
            PaddingClass::XSmall => self.min_pct(0.5),   // 0.5%
            PaddingClass::Small => self.min_pct(1.0),    // 1%
            PaddingClass::Medium => self.min_pct(2.0),   // 2%
            PaddingClass::Large => self.min_pct(3.0),    // 3%
            PaddingClass::XLarge => self.min_pct(4.0),   // 4%
        }
    }
    
    // Responsive spacing between elements
    pub fn spacing(&self, spacing_class: SpacingClass) -> f32 {
        match spacing_class {
            SpacingClass::Tight => self.width_pct(1.0),     // 1% of width
            SpacingClass::Normal => self.width_pct(2.0),    // 2% of width
            SpacingClass::Relaxed => self.width_pct(3.0),   // 3% of width
            SpacingClass::Loose => self.width_pct(5.0),     // 5% of width
        }
    }
}

#[derive(Clone, Copy)]
pub enum FontSizeClass {
    Small,
    Medium,
    Large,
    XLarge,
}

#[derive(Clone, Copy)]
pub enum PaddingClass {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
}

#[derive(Clone, Copy)]
pub enum SpacingClass {
    Tight,
    Normal,
    Relaxed,
    Loose,
}

/// Convenience trait for easy responsive sizing access
pub trait ResponsiveExt {
    fn responsive(&self) -> &ResponsiveSize;
    
    // Shorthand methods for common operations
    fn w(&self, pct: f32) -> f32 { self.responsive().width_pct(pct) }
    fn h(&self, pct: f32) -> f32 { self.responsive().height_pct(pct) }
    fn font(&self, class: FontSizeClass) -> f32 { self.responsive().font_size(class) }
    fn pad(&self, class: PaddingClass) -> f32 { self.responsive().padding(class) }
    fn space(&self, class: SpacingClass) -> f32 { self.responsive().spacing(class) }
}

// Forward declaration for ScreenLayout - implementation will be in display.rs
// This avoids circular dependencies while allowing the trait to be used
