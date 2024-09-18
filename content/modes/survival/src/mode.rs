//! Game mode of the `survival` mode.

use crate::prelude::*;

/// Game mode for the `survival` mode.
#[derive(Debug, Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct Survival;

impl IGameMode for Survival {
    fn id(&self) -> SmolStr {
        "survival".into()
    }

    fn name(&self) -> LocalizedText {
        LocalizedText::Localized {
            key: "survival-mode-name",
            args: smallvec![],
            fallback: "Survival Mode".into(),
        }
    }

    fn player_level_structure(&self) -> PlayerLevelStructure {
        PlayerLevelStructure {
            max_level: None,
            required_experience_calculator: |_world, level| {
                Experience((level.get() + 3).pow(2) as f64)
            },
        }
    }

    fn default_enemy_spawn_pattern(&self, world: &World) -> EnemySpawnPattern {
        let enemy_registry = world.resource::<EnemyRegistry>();
        let enemy_pack_index = world.resource::<EnemyPackIndex>();
        let enemies = &enemy_registry[enemy_pack_index.0].enemies;

        let first_melee_enemy =
            enemies.iter().find(|enemy| enemy.has_tag(MELEE_ENEMY_TAG)).map(|enemy| enemy.deref());
        let first_ranged_enemy =
            enemies.iter().find(|enemy| enemy.has_tag(RANGED_ENEMY_TAG)).map(|enemy| enemy.deref());

        let current_wave = world.resource::<CurrentWave>();

        let mut spawns = Vec::new();
        match current_wave.get() {
            1 => {
                if let Some(enemy) = first_melee_enemy {
                    spawns.push(
                        EnemySpawn::new_dyn(Duration::from_millis(500), enemy)
                            .count(3)
                            .interval(Duration::from_millis(150))
                            .spread(EnemySpawnSpread::square(100.00))
                            .repeat(Duration::from_millis(1500)),
                    );
                }
            },
            2 => {
                if let Some(enemy) = first_ranged_enemy {
                    spawns.push(
                        EnemySpawn::new_dyn(Duration::from_millis(500), enemy)
                            .count(3)
                            .interval(Duration::from_millis(150))
                            .spread(EnemySpawnSpread::square(100.00))
                            .repeat(Duration::from_millis(1500)),
                    );
                }
            },
            _ => {
                if let Some(enemy) = first_melee_enemy {
                    spawns.push(
                        EnemySpawn::new_dyn(Duration::from_millis(500), enemy)
                            .count(3)
                            .interval(Duration::from_millis(150))
                            .spread(EnemySpawnSpread::square(100.00))
                            .repeat(Duration::from_millis(3000)),
                    );
                }
                if let Some(enemy) = first_ranged_enemy {
                    spawns.push(
                        EnemySpawn::new_dyn(Duration::from_millis(1000), enemy)
                            .count(3)
                            .interval(Duration::from_millis(150))
                            .spread(EnemySpawnSpread::square(100.00))
                            .repeat(Duration::from_millis(3000)),
                    );
                }
            },
        }
        EnemySpawnPattern::new(spawns)
    }

    fn market_can_be_opened_by_player(&self) -> bool {
        true
    }

    fn initialize(&self, world: &mut World) {
        world.init_resource::<GameMode<Survival>>();
    }

    fn deinitialize(&self, world: &mut World) {
        world.remove_resource::<GameMode<Survival>>();
    }
}
