//! Constants of the `survival` mode.

use crate::prelude::*;


/// Size of the grid. Value of 10 means the map will be a 10x10 grid of squares.
pub const GRID_SIZE: i32 = 10;

/// Amount of space between the grid lines.
pub const GRID_SPACING: f32 = 50.00;

/// Thickness of the grid lines.
pub const GRID_THICKNESS: f32 = 2.00;

/// Color of the grid lines.
pub const GRID_COLOR: Color = Color::srgb(0.27, 0.27, 0.27);


/// Size of the map.
pub const MAP_SIZE: f32 = (GRID_SIZE as f32) * GRID_SPACING;

/// Bounds of the map.
pub const MAP_BOUNDS: MapBounds = MapBounds {
    x_min: -(MAP_SIZE / 2.00),
    x_max: (MAP_SIZE / 2.00),
    y_min: -(MAP_SIZE / 2.00),
    y_max: (MAP_SIZE / 2.00),
};


/// Number of waves.
pub const WAVES: u8 = 3;


/// Color of the current wave text in the head-up display.
pub const CURRENT_WAVE_TEXT_COLOR: Color = Color::srgb(1.00, 1.00, 1.00);

/// Font size of the current wave text in the head-up display.
pub const CURRENT_WAVE_TEXT_FONT_SIZE: f32 = 150.0;


/// Color of the remaining seconds text in the head-up display.
pub const REMAINING_SECONDS_TEXT_COLOR: Color = Color::srgb(0.90, 0.90, 0.90);

/// Font size of the remaining seconds text in the head-up display.
pub const REMAINING_SECONDS_TEXT_FONT_SIZE: f32 = 130.0;
