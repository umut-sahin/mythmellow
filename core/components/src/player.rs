//! Player components.

use {
    crate::all::*,
    mythmallow_core_dependencies::*,
};


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
