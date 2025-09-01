use bevy::prelude::*;
use bevy::window::{WindowResized, PrimaryWindow};
use crate::gameplay::GameState;
use crate::visualization::garden::{GardenBackground, LayoutInitialized, init_garden_ui, ResourceDisplayText, SpeciesDisplayText};
use crate::visualization::cards::init_hand_cards;
use crate::visualization::ui::init_action_buttons;
use super::responsive_size_utils::{ResponsiveSize, FontSizeClass, PaddingClass, SpacingClass, ResponsiveExt};

/// Screen layout manager for responsive UI positioning
#[derive(Resource, Clone)]
pub struct ScreenLayout {
    pub window_size: Vec2,
    pub garden_area: Vec2,
    pub garden_center: Vec2,
    pub card_area_y: f32,
    pub button_area_y: f32,
    pub responsive: ResponsiveSize,
}

impl Default for ScreenLayout {
    fn default() -> Self {
        let window_size = Vec2::new(800.0, 600.0);
        let responsive = ResponsiveSize::new(window_size);
        
        // Allocate screen space: 10% buttons, 20% cards, 70% garden
        let button_height_pct = 10.0;
        let card_height_pct = 20.0;
        let garden_height_pct = 70.0;
        
        let button_area_y = -window_size.y * 0.5 + (button_height_pct / 100.0) * window_size.y * 0.5;
        let card_area_y = button_area_y + (button_height_pct / 100.0) * window_size.y + (card_height_pct / 100.0) * window_size.y * 0.5;
        let garden_height = (garden_height_pct / 100.0) * window_size.y;
        let garden_center_y = card_area_y + (card_height_pct / 100.0) * window_size.y * 0.5 + garden_height * 0.5;
        
        Self {
            window_size,
            garden_area: Vec2::new(window_size.x, garden_height),
            garden_center: Vec2::new(0.0, garden_center_y),
            card_area_y,
            button_area_y,
            responsive,
        }
    }
}

impl ResponsiveExt for ScreenLayout {
    fn responsive(&self) -> &ResponsiveSize {
        &self.responsive
    }
}

impl ScreenLayout {
    pub fn update_for_window_size(&mut self, window_size: Vec2) {
        self.window_size = window_size;
        self.responsive = ResponsiveSize::new(window_size);
        
        // Allocate screen space: 10% buttons, 20% cards, 70% garden
        let button_height_pct = 10.0;
        let card_height_pct = 20.0;
        let garden_height_pct = 70.0;
        
        // Calculate positions from bottom to top
        // Buttons are at the bottom (just above the very bottom edge)
        self.button_area_y = -window_size.y * 0.5 + self.responsive.height_pct(button_height_pct * 0.5);
        
        // Cards are above buttons
        self.card_area_y = self.button_area_y + self.responsive.height_pct(button_height_pct * 0.5) + self.responsive.height_pct(card_height_pct * 0.5);
        
        // Garden takes up the remaining space at the top
        let garden_height = self.responsive.height_pct(garden_height_pct);
        self.garden_area = Vec2::new(window_size.x, garden_height);
        
        // Garden center is in the middle of the garden area
        let garden_bottom = self.card_area_y + self.responsive.height_pct(card_height_pct * 0.5);
        self.garden_center = Vec2::new(0.0, garden_bottom + garden_height * 0.5);
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

// ==============================================
// LAYOUT SYSTEMS (from layout.rs)
// ==============================================

/// System to initialize screen layout on startup
pub fn init_screen_layout(
    mut screen_layout: ResMut<ScreenLayout>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(window) = window_query.iter().next() {
        let window_size = Vec2::new(window.width(), window.height());
        screen_layout.update_for_window_size(window_size);
    }
}

/// Window resize handling system
pub fn handle_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut screen_layout: ResMut<ScreenLayout>,
    mut layout_initialized: ResMut<LayoutInitialized>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut garden_query: Query<&mut Transform, With<GardenBackground>>,
    mut garden_sprite_query: Query<&mut Sprite, With<GardenBackground>>,
    mut resource_query: Query<&mut Transform, (With<ResourceDisplayText>, Without<GardenBackground>, Without<SpeciesDisplayText>)>,
    mut species_query: Query<&mut Transform, (With<SpeciesDisplayText>, Without<GardenBackground>, Without<ResourceDisplayText>)>,
) {
    let mut layout_updated = false;
    
    // Handle window resize events
    for event in resize_events.read() {
        let new_size = Vec2::new(event.width, event.height);
        screen_layout.update_for_window_size(new_size);
        layout_updated = true;
    }
    
    // If not initialized yet, try to initialize from current window
    if !layout_initialized.0 {
        if let Some(window) = window_query.iter().next() {
            let window_size = Vec2::new(window.width(), window.height());
            if window_size.x > 0.0 && window_size.y > 0.0 && window_size != Vec2::new(800.0, 600.0) {
                screen_layout.update_for_window_size(window_size);
                layout_initialized.0 = true;
                layout_updated = true;
            }
        }
    }
    
    // Update UI elements if layout was updated
    if layout_updated {
        // Update garden background size and position
        for mut transform in garden_query.iter_mut() {
            transform.translation = Vec3::new(screen_layout.garden_center.x, screen_layout.garden_center.y, 0.0);
        }
        
        for mut sprite in garden_sprite_query.iter_mut() {
            sprite.custom_size = Some(screen_layout.garden_area);
        }
        
        // Update relative positions of child text elements when garden size changes
        for mut transform in resource_query.iter_mut() {
            transform.translation = Vec3::new(
                -screen_layout.garden_area.x / 4.0,
                0.0,
                1.0
            );
        }
        
        for mut transform in species_query.iter_mut() {
            transform.translation = Vec3::new(
                screen_layout.garden_area.x / 4.0,
                0.0,
                1.0
            );
        }
    }
}

/// Initial setup system for UI elements
pub fn init_ui_elements(
    mut commands: Commands,
    game_state: Res<GameState>,
    screen_layout: Res<ScreenLayout>,
) {
    commands.spawn(Camera2d);
    // Initialize layout tracking resource
    commands.insert_resource(LayoutInitialized::default());
    
    init_garden_ui(&mut commands, &screen_layout);
    init_hand_cards(&mut commands, &game_state, &screen_layout);
    init_action_buttons(&mut commands, &screen_layout);
}


