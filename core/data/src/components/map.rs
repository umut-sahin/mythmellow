//! Map components.

use crate::prelude::*;


/// Component for the map.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct Map;


/// Component for the map bounds.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct MapBound;