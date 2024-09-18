//! Combat bundles.

use crate::prelude::*;


/// Bundle for the projectiles.
#[derive(Bundle, TypedBuilder)]
pub struct ProjectileBundle {
    /// Originator of the projectile.
    #[builder(setter(into))]
    pub originator: Originator,

    /// Mesh of the projectile.
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,

    /// Collider of the projectile.
    pub collider: Collider,

    /// Starting position of the projectile.
    pub position: Position,

    /// Velocity of the projectile.
    pub velocity: LinearVelocity,

    /// Damage of the projectile.
    pub damage: Damage,
}

impl ProjectileBundle {
    /// Spawns the projectile.
    fn spawn<'c>(
        self,
        commands: &'c mut Commands,
        layers: CollisionLayers,
        additional_components: impl Bundle,
    ) -> EntityCommands<'c> {
        commands.spawn((
            // Tags
            Name::new("Projectile"),
            Projectile,
            // Projectile
            self,
            additional_components,
            // Physics
            RigidBody::Dynamic,
            layers,
            // Lifetime
            StateScoped(InChapter),
        ))
    }

    /// Spawns the projectile towards the player.
    pub fn spawn_toward_player<'c>(self, commands: &'c mut Commands) -> EntityCommands<'c> {
        let layers = CollisionLayers::new(
            [Layer::Projectile, Layer::DamagePlayer],
            [Layer::MapBound, Layer::MapObstacle, Layer::PlayerHitBox],
        );
        self.spawn(commands, layers, (DamagePlayerOnContactStarted, Attack::Contact))
    }

    /// Spawns the projectile towards the enemies.
    pub fn spawn_toward_enemies<'c>(self, commands: &'c mut Commands) -> EntityCommands<'c> {
        let layers = CollisionLayers::new(
            [Layer::Projectile, Layer::DamageEnemies],
            [Layer::MapBound, Layer::MapObstacle, Layer::EnemyHitBox],
        );
        self.spawn(commands, layers, (DamageEnemiesOnContactStarted, Attack::Contact))
    }
}
