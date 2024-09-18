//! Property components.

use crate::prelude::*;


/// Component for the damage of the entities of the game.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct Damage(pub f32);


/// Component for the cooldown duration of dashing of the entities of the game.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct DashCooldownDuration(pub Duration);


/// Component for the duration of dashing of the entities of the game.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct DashDuration(pub Duration);


/// Component for the speed multiplier of dashing of the entities of the game.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct DashSpeedMultiplier(pub f32);

impl Default for DashSpeedMultiplier {
    fn default() -> DashSpeedMultiplier {
        DashSpeedMultiplier(3.00)
    }
}


/// Component for the dodge chance of the entities of the game.
#[derive(Clone, Copy, Component, Debug, Default, Deref, DerefMut, Reflect)]
pub struct DodgeChance(pub f32);


/// Component for the health of the entities of the game.
#[derive(Clone, Component, Copy, Debug, Deref, DerefMut, Reflect)]
pub struct Health(pub f32);


/// Component for the pickup range of the entities of the game.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct PickupRange(pub f32);


/// Component for the range of the entities of the game.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct Range(pub f32);


/// Component for the remaining health of the entities of the game.
#[derive(Clone, Component, Copy, Debug, Deref, DerefMut, Reflect)]
pub struct RemainingHealth(pub f32);


/// Component for the speed of the entities of the game.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct Speed(pub f32);


/// Component for the speed multiplier of the entities of the game.
#[derive(Clone, Copy, Component, Debug, Deref, DerefMut, Reflect)]
pub struct SpeedMultiplier(pub f32);

impl Default for SpeedMultiplier {
    fn default() -> SpeedMultiplier {
        SpeedMultiplier(1.00)
    }
}
