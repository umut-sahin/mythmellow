//! Mythology of the `greek` players.

use crate::prelude::*;

/// Mythology for the `greek` players.
#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct GreekMythology;

impl IMythology for GreekMythology {
    fn id(&self) -> SmolStr {
        "greek".into()
    }

    fn name(&self) -> LocalizedText {
        LocalizedText::Localized {
            key: "greek-mythology-name",
            args: smallvec![],
            fallback: "Greek Mythology".into(),
        }
    }
}
