//! Movement systems.

use {
    mythmallow_core_actions::*,
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
};


/// Starts dashing the player.
pub fn player_dash(
    mut commands: Commands,
    player_query: Query<
        (Entity, &ActionState<GameAction>, &LinearVelocity, &DashDuration, &DashCooldownDuration),
        (With<Player>, Without<Cooldown<Dashing>>),
    >,
) {
    let (entity, action_state, velocity, dash_duration, cooldown_duration) =
        match player_query.get_single() {
            Ok(query_result) => query_result,
            Err(_) => return,
        };

    if action_state.just_pressed(&GameAction::Dash) {
        if velocity.0 == Vec2::ZERO {
            return;
        }
        commands.entity(entity).insert((
            Dashing { timer: Timer::new(dash_duration.0, TimerMode::Once) },
            Cooldown::<Dashing>::new(cooldown_duration.0),
        ));
    }
}

/// Moves the player.
pub fn player_movement(
    mut player_query: Query<
        (&ActionState<GameAction>, &Speed, &SpeedMultiplier, &mut LinearVelocity),
        (With<Player>, Without<Dashing>),
    >,
) {
    let (action_state, speed, speed_multiplier, mut velocity) = match player_query.get_single_mut()
    {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let mut change = Vec2::ZERO;

    if action_state.pressed(&GameAction::MoveUp) {
        change.y += 1.0;
    }
    if action_state.pressed(&GameAction::MoveLeft) {
        change.x -= 1.0;
    }
    if action_state.pressed(&GameAction::MoveDown) {
        change.y -= 1.0;
    }
    if action_state.pressed(&GameAction::MoveRight) {
        change.x += 1.0;
    }

    velocity.0 = if change == Vec2::ZERO {
        Vec2::ZERO
    } else {
        change.normalize() * (speed.0 * speed_multiplier.0)
    }
}


/// Increases the velocity of the entities that just started to dash.
pub fn start_dashing(
    mut query: Query<(&mut LinearVelocity, &DashSpeedMultiplier), Added<Dashing>>,
) {
    for (mut velocity, multiplier) in &mut query {
        velocity.0 *= multiplier.0;
    }
}

/// Keeps dashing behavior until it finishes and decreases the velocity to what it was before.
pub fn keep_dashing(
    mut commands: Commands,
    time: Res<Time>,
    mut dashing_entities_query: Query<(
        Entity,
        &mut Dashing,
        &mut LinearVelocity,
        &DashSpeedMultiplier,
    )>,
) {
    for (entity, mut dashing, mut velocity, multiplier) in &mut dashing_entities_query {
        dashing.timer.tick(time.delta());
        if dashing.timer.finished() {
            velocity.0 /= multiplier.0;
            commands.entity(entity).remove::<Dashing>();
        }
    }
}


/// Attracts objects to each other.
pub fn attraction(
    time: Res<Time>,
    mut attracted_query: Query<
        (
            &Position,
            &AttractedTo,
            &mut AttractionSpeed,
            Option<&IdealAttractionDistance>,
            Option<&SlowdownOfGoingBackwardsDuringAttraction>,
            &mut LinearVelocity,
        ),
        Without<Dashing>,
    >,
    target_query: Query<&Position>,
) {
    for (
        position,
        attracted_to,
        mut attraction_speed,
        ideal_distance,
        backwards_slowdown,
        mut velocity,
    ) in attracted_query.iter_mut()
    {
        if let Ok(target_position) = target_query.get(attracted_to.0) {
            let ideal_distance = *ideal_distance.cloned().unwrap_or_default();
            let direction = target_position.0 - position.0;

            let speed = match attraction_speed.deref_mut() {
                AttractionSpeed::Constant(speed) => *speed,
                AttractionSpeed::Accelerating {
                    min_speed,
                    acceleration_per_second,
                    current_speed,
                    max_speed,
                } => {
                    let previous_speed = *current_speed;

                    if current_speed.0 != max_speed.0 {
                        let new_speed =
                            current_speed.0 + (acceleration_per_second.0 * time.delta_seconds());
                        *current_speed = Speed(new_speed.clamp(min_speed.0, max_speed.0));
                    }

                    previous_speed
                },
            };

            velocity.0 = direction.normalize() * speed.0;
            if direction.length() < ideal_distance {
                velocity.0 *= -backwards_slowdown.cloned().unwrap_or_default().0;
            }
        }
    }
}
