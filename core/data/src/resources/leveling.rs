//! Leveling resources.

use crate::prelude::*;


/// Resource for the experience required to get to the current level.
#[derive(Clone, Copy, Debug, Deref, DerefMut, Reflect, Resource)]
pub struct ExperienceRequiredToGetToCurrentLevel(pub Experience);


/// Resource for the experience required to level up.
#[derive(Clone, Copy, Debug, Deref, DerefMut, Reflect, Resource)]
pub struct ExperienceRequiredToLevelUp(pub Experience);


/// Resource for counting the spawned experience points.
#[derive(Debug, Default, Reflect, Resource)]
pub struct ExperiencePointCounter(usize);

impl ExperiencePointCounter {
    /// Gets the spawned experience point count.
    pub fn get(&self) -> usize {
        self.0
    }

    /// Increments the spawned experience point count.
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}
