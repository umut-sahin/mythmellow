//! Map resources.

use crate::prelude::*;


/// Resource for the bounds of the map.
#[derive(Clone, Copy, Debug, Reflect, Resource)]
pub struct MapBounds {
    /// Minimum x-coordinate of the map.
    pub x_min: f32,
    /// Maximum x-coordinate of the map.
    pub x_max: f32,
    /// Minimum y-coordinate of the map.
    pub y_min: f32,
    /// Maximum y-coordinate of the map.
    pub y_max: f32,
}
