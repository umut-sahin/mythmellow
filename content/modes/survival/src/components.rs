//! Components of the `survival` mode.

use crate::prelude::*;


/// Component for the current wave text in the head-up display.
#[derive(Clone, Component, Copy, Debug, Default, Reflect)]
pub struct CurrentWaveText;


/// Component for the remaining seconds text in the head-up display.
#[derive(Clone, Component, Copy, Debug, Default, Reflect)]
pub struct RemainingSecondsText;
