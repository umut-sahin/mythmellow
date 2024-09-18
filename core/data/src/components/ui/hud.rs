//! Head-up display components.

use crate::prelude::*;


/// Component for the head-up display.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct Hud;


/// Component for the health bar in the head-up display.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct HudHealthBar;


/// Component for the text of the health bar in the head-up display.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct HudHealthBarText;


/// Component for the experience bar in the head-up display.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct HudExperienceBar;


/// Component for the text of the experience bar in the head-up display.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct HudExperienceBarText;
