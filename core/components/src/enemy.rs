//! Enemy components.

use {
    crate::all::*,
    mythmallow_core_dependencies::*,
};


/// Component for the enemies.
#[derive(Component, Debug, Default, Reflect)]
pub struct Enemy;


/// Component for the hit boxes of the enemies.
#[derive(Component, Debug, Default, Reflect)]
pub struct EnemyHitBox;

impl EnemyHitBox {
    /// Constructs an enemy hit box bundle.
    pub fn bundle(collider: Collider) -> impl Bundle {
        (
            // Tags
            Name::new("Hit Box"),
            EnemyHitBox,
            // Physics
            collider,
            CollisionLayers::new([Layer::EnemyHitBox], [Layer::DamageEnemies]),
            Sensor,
        )
    }
}
