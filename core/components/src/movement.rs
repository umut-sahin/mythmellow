//! Movement components.

use {
    crate::all::*,
    mythmallow_core_dependencies::*,
};


/// Component for the dashing entities.
#[derive(Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Dashing {
    /// Timer to track how much time is left until dashing is over.
    pub timer: Timer,
}

impl Debug for Dashing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}s", self.timer.remaining_secs())
    }
}


/// Component for being attracted to an entity.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
#[component(storage = "SparseSet")]
pub struct AttractedTo(pub Entity);


/// Component for the speed of attraction to an entity.
#[derive(Clone, Component, Debug, Reflect)]
pub enum AttractionSpeed {
    /// Attracted with a constant speed.
    Constant(Speed),
    /// Attracted while accelerating.
    Accelerating {
        /// Minimum speed of attraction.
        min_speed: Speed,
        /// Amount of acceleration per second.
        acceleration_per_second: Speed,
        /// Current speed of attraction.
        current_speed: Speed,
        /// Maximum speed of attraction.
        max_speed: Speed,
    },
}


/// Component for the ideal distance to the attracted object.
#[derive(Clone, Copy, Component, Debug, Default, Deref, DerefMut, Reflect)]
pub struct IdealAttractionDistance(pub f32);


/// Component for the slowdown when going backwards towards the ideal distance.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct SlowdownOfGoingBackwardsDuringAttraction(pub f32);

impl Default for SlowdownOfGoingBackwardsDuringAttraction {
    fn default() -> Self {
        SlowdownOfGoingBackwardsDuringAttraction(1.00)
    }
}
