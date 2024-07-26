//! Combat systems.

use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
};


/// Starts the attack animations.
pub fn start_attack_animations(
    mut commands: Commands,
    mut attack_query: Query<
        (Entity, &Transform, &mut Attack),
        Without<EasingChainComponent<Transform>>,
    >,
) {
    for (entity, transform, mut attack) in attack_query.iter_mut() {
        match attack.deref_mut() {
            Attack::Contact => {},
            Attack::Thrust { direction, range, duration, started } => {
                if !*started {
                    let mut target_transform = *transform;
                    target_transform.translation += (direction.xy() * range.0).extend(0.00);

                    let pattern = transform
                        .ease_to(
                            target_transform,
                            EaseFunction::QuadraticInOut,
                            EasingType::Once { duration: *duration / 2 },
                        )
                        .ease_to(
                            *transform,
                            EaseFunction::QuadraticInOut,
                            EasingType::Once { duration: *duration / 2 },
                        );
                    commands.entity(entity).insert(pattern);

                    *started = true;
                } else {
                    commands.entity(entity).remove::<Attack>();
                }
            },
        }
    }
}

/// Pauses the attack animations.
pub fn pause_attack_animations(
    mut attack_query: Query<&mut EasingComponent<Transform>, With<Attack>>,
) {
    for mut easing in attack_query.iter_mut() {
        easing.state = EasingState::Paused;
    }
}

/// Resumes the attack animations.
pub fn resume_attack_animations(
    mut attack_query: Query<&mut EasingComponent<Transform>, With<Attack>>,
) {
    for mut easing in attack_query.iter_mut() {
        easing.state = EasingState::Play;
    }
}


/// Applies damage.
pub fn apply_damage(
    commands: &mut Commands,
    name_query: &Query<&Name>,
    rng: &mut ResMut<GlobalEntropy<WyRand>>,

    damaged_entity_name: &Name,
    damaged_entity_dodge_chance: Option<&DodgeChance>,
    damaged_entity_remaining_health: &mut RemainingHealth,

    damaging_entity: Entity,
    damaging_entity_name: &Name,
    damaging_entity_originator: Option<&Originator>,

    damage: &Damage,
    damage_cooldown: Option<&DamageCooldown>,

    reason: &'static str,
) {
    if let Some(damage_cooldown) = damage_cooldown {
        commands.entity(damaging_entity).insert(Cooldown::<Damage>::new(damage_cooldown.duration));
    }

    let originator = damaging_entity_originator
        .and_then(|originator| {
            name_query
                .get(originator.0)
                .map(|originator_name| format!(" of {:?}", originator_name))
                .ok()
        })
        .unwrap_or_default();

    if let Some(damaged_entity_dodge_chance) = damaged_entity_dodge_chance {
        if rng.gen_range(0.00..100.00) < damaged_entity_dodge_chance.0 {
            log::info!(
                "{:?} dodged {:.2} damage from {:?}{}",
                damaged_entity_name,
                damage.0,
                damaging_entity_name,
                originator,
            );
            return;
        }
    }

    log::info!(
        "{:?} received {:.2} damage from {} {:?}{}",
        damaged_entity_name,
        damage.0,
        reason,
        damaging_entity_name,
        originator,
    );
    damaged_entity_remaining_health.0 -= damage.0;

    if damaged_entity_remaining_health.0 > 0.00 {
        log::info!(
            "{:?} has {:.2} health left",
            damaged_entity_name,
            damaged_entity_remaining_health.0
        );
    }
}

/// Damages the player on contact.
pub fn damage_player_on_contact(
    mut commands: Commands,
    name_query: Query<&Name>,
    mut player_query: Query<(&Name, &DodgeChance, &mut RemainingHealth), With<Player>>,
    player_hit_box_query: Query<&Parent, With<PlayerHitBox>>,
    player_damage_query: Query<
        (Entity, &Name, Option<&Originator>, &Damage, Option<&DamageCooldown>),
        (With<Attack>, With<DamagePlayerOnContact>, Without<Cooldown<Damage>>),
    >,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut collision_event_reader: EventReader<Collision>,
) {
    for Collision(contacts) in collision_event_reader.read().cloned() {
        let (player_name, player_dodge_chance, mut player_remaining_health) =
            match player_hit_box_query
                .get(contacts.entity1)
                .or_else(|_| player_hit_box_query.get(contacts.entity2))
                .and_then(|parent| player_query.get_mut(parent.get()))
            {
                Ok(query_result) => query_result,
                Err(_) => continue,
            };

        let (
            damaging_entity,
            damaging_entity_name,
            damaging_entity_originator_name,
            damage,
            damage_cooldown,
        ) = match player_damage_query
            .get(contacts.entity2)
            .or_else(|_| player_damage_query.get(contacts.entity1))
        {
            Ok(query_result) => query_result,
            Err(_) => continue,
        };

        apply_damage(
            &mut commands,
            &name_query,
            &mut rng,
            player_name,
            Some(player_dodge_chance),
            &mut player_remaining_health,
            damaging_entity,
            damaging_entity_name,
            damaging_entity_originator_name,
            damage,
            damage_cooldown,
            "being in contact with",
        );
    }
}

/// Damages the player on contact.
pub fn damage_player_on_contact_started(
    mut commands: Commands,
    name_query: Query<&Name>,
    mut player_query: Query<(&Name, &DodgeChance, &mut RemainingHealth), With<Player>>,
    player_hit_box_query: Query<&Parent, With<PlayerHitBox>>,
    player_damage_query: Query<
        (Entity, &Name, Option<&Originator>, &Damage, Option<&DamageCooldown>),
        (With<Attack>, With<DamagePlayerOnContactStarted>, Without<Cooldown<Damage>>),
    >,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut collision_started_event_reader: EventReader<CollisionStarted>,
) {
    for CollisionStarted(entity1, entity2) in collision_started_event_reader.read().cloned() {
        let (player_name, player_dodge_chance, mut player_remaining_health) =
            match player_hit_box_query
                .get(entity1)
                .or_else(|_| player_hit_box_query.get(entity2))
                .and_then(|parent| player_query.get_mut(parent.get()))
            {
                Ok(query_result) => query_result,
                Err(_) => continue,
            };

        let (
            damaging_entity,
            damaging_entity_name,
            damaging_entity_originator_name,
            damage,
            damage_cooldown,
        ) = match player_damage_query.get(entity2).or_else(|_| player_damage_query.get(entity1)) {
            Ok(query_result) => query_result,
            Err(_) => continue,
        };

        apply_damage(
            &mut commands,
            &name_query,
            &mut rng,
            player_name,
            Some(player_dodge_chance),
            &mut player_remaining_health,
            damaging_entity,
            damaging_entity_name,
            damaging_entity_originator_name,
            damage,
            damage_cooldown,
            "starting to contact",
        );
    }
}

/// Damages the enemies on contact.
pub fn damage_enemies_on_contact(
    mut commands: Commands,
    name_query: Query<&Name>,
    mut enemy_query: Query<(&Name, Option<&DodgeChance>, &mut RemainingHealth), With<Enemy>>,
    enemy_hit_box_query: Query<&Parent, With<EnemyHitBox>>,
    enemy_damage_query: Query<
        (Entity, &Name, Option<&Originator>, &Damage, Option<&DamageCooldown>),
        (With<Attack>, With<DamageEnemiesOnContact>, Without<Cooldown<Damage>>),
    >,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut collision_event_reader: EventReader<Collision>,
) {
    for Collision(contacts) in collision_event_reader.read().cloned() {
        let (enemy_name, enemy_dodge_chance, mut enemy_remaining_health) = match enemy_hit_box_query
            .get(contacts.entity1)
            .or_else(|_| enemy_hit_box_query.get(contacts.entity2))
            .and_then(|parent| enemy_query.get_mut(parent.get()))
        {
            Ok(query_result) => query_result,
            Err(_) => continue,
        };

        let (
            damaging_entity,
            damaging_entity_name,
            damaging_entity_originator_name,
            damage,
            damage_cooldown,
        ) = match enemy_damage_query
            .get(contacts.entity2)
            .or_else(|_| enemy_damage_query.get(contacts.entity1))
        {
            Ok(query_result) => query_result,
            Err(_) => continue,
        };

        apply_damage(
            &mut commands,
            &name_query,
            &mut rng,
            enemy_name,
            enemy_dodge_chance,
            &mut enemy_remaining_health,
            damaging_entity,
            damaging_entity_name,
            damaging_entity_originator_name,
            damage,
            damage_cooldown,
            "being in contact with",
        );
    }
}

/// Damages the enemies on contact started.
pub fn damage_enemies_on_contact_started(
    mut commands: Commands,
    name_query: Query<&Name>,
    mut enemy_query: Query<(&Name, Option<&DodgeChance>, &mut RemainingHealth), With<Enemy>>,
    enemy_hit_box_query: Query<&Parent, With<EnemyHitBox>>,
    enemy_damage_query: Query<
        (Entity, &Name, Option<&Originator>, &Damage, Option<&DamageCooldown>),
        (With<Attack>, With<DamageEnemiesOnContactStarted>, Without<Cooldown<Damage>>),
    >,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut collision_started_event_reader: EventReader<CollisionStarted>,
) {
    for CollisionStarted(entity1, entity2) in collision_started_event_reader.read().cloned() {
        let (enemy_name, enemy_dodge_chance, mut enemy_remaining_health) = match enemy_hit_box_query
            .get(entity1)
            .or_else(|_| enemy_hit_box_query.get(entity2))
            .and_then(|parent| enemy_query.get_mut(parent.get()))
        {
            Ok(query_result) => query_result,
            Err(_) => continue,
        };

        let (
            damaging_entity,
            damaging_entity_name,
            damaging_entity_originator_name,
            damage,
            damage_cooldown,
        ) = match enemy_damage_query.get(entity2).or_else(|_| enemy_damage_query.get(entity1)) {
            Ok(query_result) => query_result,
            Err(_) => continue,
        };

        apply_damage(
            &mut commands,
            &name_query,
            &mut rng,
            enemy_name,
            enemy_dodge_chance,
            &mut enemy_remaining_health,
            damaging_entity,
            damaging_entity_name,
            damaging_entity_originator_name,
            damage,
            damage_cooldown,
            "starting to contact",
        );
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


/// Handles the death of the player.
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

/// Handles the death of the enemies.
pub fn enemy_death(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Name, &RemainingHealth), With<Enemy>>,
) {
    for (enemy_entity, enemy_name, enemy_remaining_health) in enemy_query.iter() {
        if enemy_remaining_health.0 <= 0.00 {
            log::info!("{:?} has died", enemy_name);
            commands.entity(enemy_entity).despawn_recursive();
        }
    }
}
