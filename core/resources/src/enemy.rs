//! Enemy resources.

use mythmallow_core_dependencies::*;


/// Resource for the index of the selected enemy pack in the enemy registry.
#[derive(Debug, Deref, Resource, Reflect)]
pub struct EnemyPackIndex(pub usize);


/// Resource for counting the spawned enemies.
#[derive(Debug, Default, Reflect, Resource)]
pub struct EnemyCounter(usize);

impl EnemyCounter {
    /// Gets the spawned enemy count.
    pub fn get(&self) -> usize {
        self.0
    }

    /// Increments the spawned enemy count.
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}
