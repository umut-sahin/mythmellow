//! Utility components.

use crate::prelude::*;


/// Component for the 2d depth.
#[derive(Clone, Component, Copy, Debug)]
#[repr(u8)]
pub enum Depth {
    /// Bottom of the z axis.
    Bottom = 0,

    /// Depth of the map.
    Map,
    /// Depth of the experience points.
    ExperiencePoint,
    /// Depth of the enemies.
    Enemy,
    /// Depth of the player.
    Player,
    /// Depth of the items.
    Item,
    /// Depth of the projectiles.
    Projectile,

    /// Top of the z axis.
    Top,
}

impl Depth {
    /// Gets z value of the depth.
    pub fn z(self) -> f32 {
        (self as u8) as f32
    }
}


/// Component for the layers of collisions.
#[derive(Clone, Component, Copy, Debug, PhysicsLayer, Reflect)]
pub enum Layer {
    /// Layer for the invisible bounds of the map.
    MapBound,
    /// Layer for the obstacles in the map.
    MapObstacle,

    /// Layer for the player.
    Player,
    /// Layer for the enemies.
    Enemy,

    /// Layer for the hit box of the player.
    PlayerHitBox,
    /// Layer for damaging the player.
    DamagePlayer,

    /// Layer for the hit box of the enemies.
    EnemyHitBox,
    /// Layer for damaging the enemies.
    DamageEnemies,

    /// Layer for projectiles.
    Projectile,

    /// Layer for experience points.
    ExperiencePoint,
    /// Layer for the pickup area of the player.
    PlayerPickupArea,
}
