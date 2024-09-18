//! Enemy bundles.

use crate::prelude::*;


/// Bundle for the enemies.
#[derive(Bundle, TypedBuilder)]
pub struct EnemyBundle<E: IEnemy + Component> {
    /// Enemy.
    pub enemy: E,

    /// Mesh of the enemy.
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,

    /// Position of the enemy.
    pub position: Position,
}

impl<E: IEnemy + Component> EnemyBundle<E> {
    /// Spawns the enemy.
    pub fn spawn<'c>(
        self,
        commands: &'c mut Commands,
        counter: &mut EnemyCounter,
    ) -> EntityCommands<'c> {
        counter.increment();

        let name = format!("Enemy {} [{}]", counter.get(), self.enemy.id());
        let collider = self.enemy.collider();

        let contact_damage = self.enemy.contact_damage();
        let health = self.enemy.health();
        let speed = AttractionSpeed::Constant(self.enemy.speed());

        let experience_reward = self.enemy.experience_reward();
        let experience_point_visuals = self.enemy.experience_point_visuals();
        let experience_point_attraction_speed = self.enemy.experience_point_attraction_speed();

        let mut collision_groups = LayerMask::from([Layer::Enemy]);
        let mut collision_masks = LayerMask::from([Layer::MapBound, Layer::Enemy]);

        if contact_damage.is_some() {
            collision_groups.add([Layer::DamagePlayer]);
            collision_masks.add([Layer::PlayerHitBox]);
        }

        let collision_layers = CollisionLayers::new(collision_groups, collision_masks);

        let mut enemy = commands.spawn((
            // Tags
            Name::new(name),
            Enemy,
            // Enemy
            self,
            health,
            speed,
            RemainingHealth(health.0),
            // Leveling
            experience_reward,
            experience_point_visuals,
            experience_point_attraction_speed,
            // Physics
            (
                RigidBody::Dynamic,
                LinearVelocity::ZERO,
                Restitution::PERFECTLY_INELASTIC,
                LockedAxes::ROTATION_LOCKED,
                collider.clone(),
                collision_layers,
            ),
            // Lifetime
            StateScoped(InChapter),
        ));


        enemy.with_children(|parent| {
            parent.spawn(EnemyHitBox::bundle(collider));
        });

        if let Some((damage, cooldown)) = contact_damage {
            enemy.insert((Attack::Contact, DamagePlayerOnContact, damage, cooldown));
        }

        enemy
    }
}
