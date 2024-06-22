use crate::prelude::*;


/// Damages the player.
pub fn damage_player(
    mut commands: Commands,
    mut player_query: Query<(&Name, &DodgeChance, &mut RemainingHealth), With<Player>>,
    player_hit_box_query: Query<&Parent, With<PlayerHitBox>>,
    player_damage_query: Query<
        (Entity, &Name, Option<&OriginatorName>, &Damage, Option<&DamageCooldown>),
        (With<DamagePlayerOnContact>, Without<Cooldown<Attack>>),
    >,
    mut collision_event_reader: EventReader<Collision>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    let mut apply_damage_if_applicable = |player_hit_box_entity, player_damage_entity| {
        let (
            damaging_entity,
            damaging_entity_name,
            damaging_entity_originator_name,
            damage,
            damage_cooldown,
        ) = match player_damage_query.get(player_damage_entity) {
            Ok(query_result) => query_result,
            Err(_) => return,
        };
        let player = match player_hit_box_query.get(player_hit_box_entity) {
            Ok(parent) => player_query.get_mut(parent.get()),
            Err(_) => return,
        };
        if let Ok((player_name, player_dodge_chance, mut player_remaining_health)) = player {
            if let Some(damage_cooldown) = damage_cooldown {
                commands
                    .entity(damaging_entity)
                    .insert(Cooldown::<Attack>::new(damage_cooldown.duration));
            }

            let originator = damaging_entity_originator_name
                .map(|name| format!(" of {:?}", name.0))
                .unwrap_or_default();

            if rng.gen_range(0.00..100.00) < player_dodge_chance.0 {
                log::info!(
                    "{:?} dodged {:.2} damage from {:?}{}",
                    player_name,
                    damage.0,
                    damaging_entity_name,
                    originator,
                );
                return;
            }

            log::info!(
                "{:?} received {:.2} damage from {:?}{}",
                player_name,
                damage.0,
                damaging_entity_name,
                originator,
            );
            player_remaining_health.0 -= damage.0;

            if player_remaining_health.0 > 0.00 {
                log::info!("{:?} has {:.2} health left", player_name, player_remaining_health.0);
            }
        }
    };

    for Collision(contacts) in collision_event_reader.read().cloned() {
        apply_damage_if_applicable(contacts.entity1, contacts.entity2);
        apply_damage_if_applicable(contacts.entity2, contacts.entity1);
    }
}

/// Damages the enemies.
pub fn damage_enemies(
    mut commands: Commands,
    mut enemy_query: Query<(&Name, &mut RemainingHealth), With<Enemy>>,
    enemy_hit_box_query: Query<&Parent, With<EnemyHitBox>>,
    enemy_damage_query: Query<
        (Entity, &Name, Option<&OriginatorName>, &Damage, Option<&DamageCooldown>),
        (With<DamageEnemiesOnContact>, Without<Cooldown<Attack>>),
    >,
    mut collision_event_reader: EventReader<Collision>,
) {
    let mut apply_damage_if_applicable = |enemy_hit_box_entity, enemy_damage_entity| {
        let (
            damaging_entity,
            damaging_entity_name,
            damaging_entity_originator_name,
            damage,
            damage_cooldown,
        ) = match enemy_damage_query.get(enemy_damage_entity) {
            Ok(query_result) => query_result,
            Err(_) => return,
        };
        let enemy = match enemy_hit_box_query.get(enemy_hit_box_entity) {
            Ok(parent) => enemy_query.get_mut(parent.get()),
            Err(_) => return,
        };
        if let Ok((enemy_name, mut enemy_remaining_health)) = enemy {
            if let Some(damage_cooldown) = damage_cooldown {
                commands
                    .entity(damaging_entity)
                    .insert(Cooldown::<Attack>::new(damage_cooldown.duration));
            }

            let originator = damaging_entity_originator_name
                .map(|name| format!(" of {:?}", name.0))
                .unwrap_or_default();

            log::info!(
                "{:?} received {:.2} damage from {:?}{}",
                enemy_name,
                damage.0,
                damaging_entity_name,
                originator,
            );
            enemy_remaining_health.0 -= damage.0;

            if enemy_remaining_health.0 > 0.00 {
                log::info!("{:?} has {:.2} health left", enemy_name, enemy_remaining_health.0);
            }
        }
    };

    for Collision(contacts) in collision_event_reader.read().cloned() {
        apply_damage_if_applicable(contacts.entity1, contacts.entity2);
        apply_damage_if_applicable(contacts.entity2, contacts.entity1);
    }
}


/// Handles player death.
pub fn player_death(
    mut commands: Commands,
    player_query: Query<(&Name, &RemainingHealth), With<Player>>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let (player_name, player_remaining_health) = match player_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };
    if player_remaining_health.0 <= 0.00 {
        log::info!("{:?} has died", player_name);
        commands.insert_resource(GameResult::Lost);
        game_state_stack.transition(GameState::Over);
        next_game_state.set(GameState::Transition);
    }
}

/// Handles enemy death.
pub fn enemy_death(
    mut commands: Commands,
    enemy_query: Query<
        (
            Entity,
            &Name,
            &RemainingHealth,
            &Experience,
            &Position,
            &ExperiencePointVisuals,
            &ExperiencePointAttractionSpeed,
        ),
        With<Enemy>,
    >,
    map_query: Query<Entity, With<Map>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut experience_point_counter: ResMut<ExperiencePointCounter>,
) {
    for (
        enemy_entity,
        enemy_name,
        enemy_remaining_health,
        enemy_experience_reward,
        enemy_position,
        experience_point_visuals,
        experience_point_attraction_speed,
    ) in enemy_query.iter()
    {
        if enemy_remaining_health.0 <= 0.00 {
            if enemy_experience_reward.0 >= 0.00 {
                let mesh = MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(experience_point_visuals.size)).into(),
                    material: materials.add(ColorMaterial::from(experience_point_visuals.color)),
                    transform: Transform::from_translation(
                        enemy_position.extend(Depth::ExperiencePoint.z()),
                    ),
                    ..default()
                };
                let experience_point_bundle = ExperiencePointBundle {
                    position: *enemy_position,
                    attraction_speed: experience_point_attraction_speed.0.clone(),
                    mesh,
                    collider: Collider::circle(experience_point_visuals.size),
                    experience: *enemy_experience_reward,
                };
                let mut experience_point_entity =
                    experience_point_bundle.spawn(&mut commands, &mut experience_point_counter);
                experience_point_entity.set_parent(map_query.get_single().unwrap());

                log::info!(
                    "{:?} has died and dropped {:?} with {:.2} experience points",
                    enemy_name,
                    format!("Experience Point {}", experience_point_counter.get()),
                    enemy_experience_reward.0,
                );
            } else {
                log::info!("{:?} has died", enemy_name);
            }
            commands.entity(enemy_entity).despawn_recursive();
        }
    }
}


/// Despawns the projectiles on contact.
pub fn despawn_projectiles_on_contact(
    mut commands: Commands,
    projectile_query: Query<Entity, With<Projectile>>,
    mut collision_started_event_reader: EventReader<CollisionStarted>,
) {
    for CollisionStarted(entity1, entity2) in collision_started_event_reader.read().cloned() {
        if projectile_query.get(entity1).is_ok() {
            commands.entity(entity1).despawn_recursive();
        } else if projectile_query.get(entity2).is_ok() {
            commands.entity(entity2).despawn_recursive();
        }
    }
}

/// Despawns the projectiles.
pub fn despawn_projectiles(
    mut commands: Commands,
    projectile_query: Query<Entity, With<Projectile>>,
) {
    for projectile_entity in projectile_query.iter() {
        commands.entity(projectile_entity).despawn_recursive();
    }
}
