//! Item components.

use mythmallow_core_dependencies::*;


/// Component for the items.
#[derive(Clone, Component, Copy, Debug, Default, Reflect)]
pub struct Item;


/// Component for the weapons.
#[derive(Clone, Component, Copy, Debug, Default, Reflect)]
pub struct Weapon;
