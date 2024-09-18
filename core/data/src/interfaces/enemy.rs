//! Enemy interfaces.

use crate::prelude::*;

/// Interface for the enemy packs.
pub trait IEnemyPack: Any + Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the enemy pack.
    fn id(&self) -> SmolStr;

    /// Gets the localized name of the enemy pack.
    fn name(&self) -> LocalizedText;

    /// Gets the spawn pattern of the enemy pack,
    #[allow(unused_variables)]
    fn spawn_pattern(&self, world: &World) -> Option<EnemySpawnPattern> {
        None
    }
}

/// Interface for the enemies.
pub trait IEnemy: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the enemy.
    fn id(&self) -> SmolStr;

    /// Gets the localized name of the enemy.
    fn name(&self) -> LocalizedText;


    /// Gets the contact damage of the enemy.
    fn contact_damage(&self) -> Option<(Damage, DamageCooldown)> {
        None
    }

    /// Gets the health of the enemy.
    fn health(&self) -> Health;

    /// Gets the speed of the enemy.
    fn speed(&self) -> Speed;


    /// Gets the experience reward for defeating the enemy.
    fn experience_reward(&self) -> Experience;

    /// Gets the visuals of experience point dropped from the enemy.
    fn experience_point_visuals(&self) -> ExperiencePointVisuals {
        ExperiencePointVisuals::default()
    }

    /// Gets the speed of the experience point dropped from the enemy when they are attracted.
    fn experience_point_attraction_speed(&self) -> ExperiencePointAttractionSpeed {
        ExperiencePointAttractionSpeed::default()
    }


    /// Gets the collider of the enemy.
    fn collider(&self) -> Collider;

    /// Spawns the enemy.
    fn spawn(&self, world: &mut World, position: Position);
}
