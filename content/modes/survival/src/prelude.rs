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
    mythmallow_core_components::all::*,
    mythmallow_core_constants::enemy::{
        MELEE_ENEMY_TAG,
        RANGED_ENEMY_TAG,
    },
    mythmallow_core_dependencies::*,
    mythmallow_core_interfaces::*,
    mythmallow_core_registries::all::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_sets::*,
    mythmallow_core_states::*,
};
