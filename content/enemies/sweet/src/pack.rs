//! Enemy pack of the `sweet` enemies.

use crate::prelude::*;

/// Enemy pack for the `sweet` enemies.
#[derive(Debug, Default, Reflect, Resource)]
pub struct SweetEnemyPack;

impl IEnemyPack for SweetEnemyPack {
    fn id(&self) -> SmolStr {
        "sweet".into()
    }

    fn name(&self) -> LocalizedText {
        LocalizedText::Localized {
            key: "sweet-enemies-pack-name",
            args: smallvec![],
            fallback: "Sweet Enemies".into(),
        }
    }
}
