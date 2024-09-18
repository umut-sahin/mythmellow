use crate::prelude::*;

/// State of the localization.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum LocalizationState {
    /// Localization is being loaded.
    #[default]
    Loading,

    /// Localization is loaded.
    Ready,
}
