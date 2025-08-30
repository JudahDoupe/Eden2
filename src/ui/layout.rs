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

impl ResponsiveExt for ScreenLayout {
    fn responsive(&self) -> &ResponsiveSize {
        &self.responsive
    }
}

/// Screen layout manager for responsive UI positioning
#[derive(Resource, Clone)]
pub struct ScreenLayout {
    pub window_size: Vec2,
    pub garden_area: Vec2,
    pub garden_center: Vec2,
    pub card_area_y: f32,
    pub responsive: ResponsiveSize,
}

impl Default for ScreenLayout {
    fn default() -> Self {
        let window_size = Vec2::new(800.0, 600.0);
        let responsive = ResponsiveSize::new(window_size);
        Self {
            window_size,
            garden_area: Vec2::new(window_size.x, window_size.y * 2.0 / 3.0),
            garden_center: Vec2::new(0.0, window_size.y / 6.0),
            card_area_y: -window_size.y * 0.4167, // Middle of bottom third (-41.67%)
            responsive,
        }
    }
}

impl ScreenLayout {
    pub fn update_for_window_size(&mut self, window_size: Vec2) {
        self.window_size = window_size;
        self.responsive = ResponsiveSize::new(window_size);
        self.garden_area = Vec2::new(window_size.x, self.responsive.height_pct(66.67)); // Top 2/3rds
        self.garden_center = Vec2::new(0.0, self.responsive.height_pct(16.67)); // Center of top 2/3rds
        
        // Position cards in the middle of the bottom third
        // Bottom third goes from -33.33% to -50% of screen height
        // So middle of bottom third is at -41.67% of screen height
        self.card_area_y = -self.responsive.height_pct(41.67);
    }
    
    pub fn calculate_card_size(&self, hand_size: usize) -> Vec2 {
        if hand_size == 0 {
            return Vec2::new(self.w(15.0), self.w(20.0)); // Default responsive size using shorthand
        }
        
        let card_area_width = self.w(80.0); // 80% of screen width available
        
        // Calculate card width based on available space and number of cards
        let max_card_width = card_area_width / hand_size as f32;
        let aspect_ratio = 2.0 / 3.0; // Standard card aspect ratio (width:height) - taller than wide
        
        // Try width-constrained sizing first
        let width_constrained_width = max_card_width * 0.8; // Leave some spacing between cards
        let width_constrained_height = width_constrained_width / aspect_ratio;
        
        // Try height-constrained sizing
        let height_constrained_height = self.h(20.0); // Max 20% of screen height using shorthand
        let height_constrained_width = height_constrained_height * aspect_ratio;
        
        // Use the smaller of the two to ensure cards fit
        if width_constrained_height <= height_constrained_height {
            Vec2::new(width_constrained_width, width_constrained_height)
        } else {
            Vec2::new(height_constrained_width, height_constrained_height)
        }
    }
    
    pub fn calculate_card_spacing(&self, hand_size: usize) -> f32 {
        if hand_size <= 1 {
            return 0.0;
        }
        
        let card_size = self.calculate_card_size(hand_size);
        let total_card_width = card_size.x * hand_size as f32;
        let available_width = self.w(80.0); // 80% of screen width using shorthand
        
        // Calculate spacing to center the cards
        let remaining_width = available_width - total_card_width;
        let spacing = remaining_width / (hand_size - 1) as f32;
        
        // Ensure minimum and maximum spacing using responsive sizing
        let min_spacing = self.space(SpacingClass::Tight);
        let max_spacing = self.space(SpacingClass::Relaxed);
        spacing.clamp(min_spacing, max_spacing)
    }
    
    pub fn resource_text_position(&self) -> Vec3 {
        Vec3::new(
            -self.garden_area.x / 2.0 + self.pad(PaddingClass::Large), // Left side with responsive padding using shorthand
            self.garden_center.y + self.garden_area.y / 4.0, // Upper area of garden
            10.0
        )
    }
    
    pub fn species_text_position(&self) -> Vec3 {
        Vec3::new(
            self.garden_area.x / 2.0 - self.pad(PaddingClass::Large), // Right side with responsive padding using shorthand
            self.garden_center.y + self.garden_area.y / 4.0, // Upper area of garden
            10.0
        )
    }
    
    // Helper methods for common UI measurements
    pub fn text_font_size(&self, class: FontSizeClass) -> f32 {
        self.responsive.font_size(class)
    }
    
    pub fn ui_padding(&self, class: PaddingClass) -> f32 {
        self.responsive.padding(class)
    }
    
    pub fn ui_spacing(&self, class: SpacingClass) -> f32 {
        self.responsive.spacing(class)
    }
}
