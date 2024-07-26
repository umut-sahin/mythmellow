//! Combat components.

use {
    crate::all::*,
    mythmallow_core_dependencies::*,
};


/// Component for attacks.
#[derive(Component, Debug, Reflect)]
#[component(storage = "SparseSet")]
pub enum Attack {
    /// Contact attack.
    Contact,
    /// Thrust attack.
    Thrust {
        /// Direction of the thrust.
        direction: Vec2,
        /// Range of the thrust.
        range: Range,
        /// Duration of the thrust.
        duration: Duration,
        /// Whether the attack has started.
        started: bool,
    },
}


/// Component for the entities that apply damage to the player on contact.
#[derive(Component, Debug, Default, Reflect)]
pub struct DamagePlayerOnContact;


/// Component for the entities that apply damage to the player on contact started.
#[derive(Component, Debug, Default, Reflect)]
pub struct DamagePlayerOnContactStarted;


/// Component for the entities that apply damage to the enemies on contact.
#[derive(Component, Debug, Default, Reflect)]
pub struct DamageEnemiesOnContact;


/// Component for the entities that apply damage to the enemies on contact started.
#[derive(Component, Debug, Default, Reflect)]
pub struct DamageEnemiesOnContactStarted;


/// Component for cooldown of applying damage.
#[derive(Component, Debug, Deref, DerefMut, Reflect)]
pub struct DamageCooldown {
    /// Duration of the damage cooldown.
    pub duration: Duration,
}

impl DamageCooldown {
    /// Creates a new damage cooldown.
    pub fn new(duration: Duration) -> DamageCooldown {
        DamageCooldown { duration }
    }
}


/// Component for the originator of entities.
#[derive(Component, Debug, Deref, DerefMut, Reflect)]
pub struct Originator(pub Entity);

impl From<Entity> for Originator {
    fn from(entity: Entity) -> Originator {
        Originator(entity)
    }
}


/// Component for the projectiles.
#[derive(Clone, Component, Copy, Debug, Default, Reflect)]
pub struct Projectile;
