//! Leveling resources.

use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
};


/// Resource for the experience required to get to the current level.
#[derive(Clone, Copy, Debug, Deref, DerefMut, Reflect, Resource)]
pub struct ExperienceRequiredToGetToCurrentLevel(pub Experience);


/// Resource for the experience required to level up.
#[derive(Clone, Copy, Debug, Deref, DerefMut, Reflect, Resource)]
pub struct ExperienceRequiredToLevelUp(pub Experience);
