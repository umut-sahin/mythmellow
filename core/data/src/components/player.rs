//! Player components.

use crate::prelude::*;


/// Component for the player.
#[derive(Component, Debug, Default, Reflect)]
pub struct Player;


/// Component for the hit box of the player.
#[derive(Component, Debug, Default, Reflect)]
pub struct PlayerHitBox;

impl PlayerHitBox {
    /// Constructs a player hit box bundle.
    pub fn bundle(collider: Collider) -> impl Bundle {
        (
            // Tags
            Name::new("Hit Box"),
            PlayerHitBox,
            // Physics
            collider,
            CollisionLayers::new([Layer::PlayerHitBox], [Layer::DamagePlayer]),
            Sensor,
        )
    }
}


/// Component for the pickup area of the player.
#[derive(Component, Debug, Default, Reflect)]
pub struct PlayerPickupArea;

impl PlayerPickupArea {
    /// Constructs a player pickup area bundle.
    pub fn bundle(collider: Collider) -> impl Bundle {
        (
            // Tags
            Name::new("Pickup Area"),
            PlayerPickupArea,
            // Physics
            collider,
            CollisionLayers::new([Layer::PlayerPickupArea], [Layer::ExperiencePoint]),
            Sensor,
        )
    }
}
