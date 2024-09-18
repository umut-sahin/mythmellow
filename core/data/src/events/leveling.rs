//! Leveling events.

use crate::prelude::*;


/// Event for gaining experience.
#[derive(Debug, Event, Reflect)]
pub struct ExperienceGainedEvent {
    /// Entity that gained the experience.
    pub entity: Entity,

    /// Amount of experience gained.
    pub experience: Experience,

    /// Reason for getting the experience.
    pub by: String,
}


/// Event for levelling up.
#[derive(Debug, Event, Reflect)]
pub struct LeveledUpEvent {
    /// Entity that gained the level.
    pub entity: Entity,
    /// New level of the entity.
    pub new_level: Level,
}
