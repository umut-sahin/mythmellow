//! Prelude of the `survival` mode.

pub use crate::{
    mode::Survival,
    plugin::SurvivalModePlugin,
};

pub(crate) use {
    crate::{
        components::*,
        resources::*,
    },
    mythmallow_core_data::{
        constants::enemy::{
            MELEE_ENEMY_TAG,
            RANGED_ENEMY_TAG,
        },
        prelude::*,
    },
    mythmallow_core_dependencies::prelude::*,
};
