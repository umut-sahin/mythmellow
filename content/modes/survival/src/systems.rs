//! Systems of the `survival` mode.

use crate::{
    constants::*,
    localization,
    prelude::*,
};


/// Initializes the game mode.
pub fn initialize(
    mut commands: Commands,
    hud_query: Query<Entity, With<Hud>>,
    localization: Res<Localization>,
    ui_font: Res<UiFont>,
) {
    let wave_durations = WaveDurations::get(WAVES);
    let current_wave = CurrentWave::default();

    log::info!("number of waves: {}", WAVES);
    log::info!("wave durations: {:#?}", wave_durations);

    if let Ok(hud) = hud_query.get_single() {
        let wave_duration =
            wave_durations.get(current_wave.index()).copied().unwrap_or(Duration::ZERO);

        commands.entity(hud).with_children(|hud| {
            hud.spawn((
                Name::new("Current Wave"),
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        align_self: AlignSelf::End,
                        justify_self: JustifySelf::Start,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(10.00),
                        height: Val::Percent(5.00),
                        top: Val::Percent(4.00),
                        right: Val::Percent(2.00),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|parent| {
                let text = localization::current_wave(&current_wave);
                parent.spawn((
                    Name::new("Text"),
                    CurrentWaveText,
                    TextBundle {
                        text: Text::from_section(
                            text.get(&localization),
                            TextStyle {
                                color: CURRENT_WAVE_TEXT_COLOR,
                                font: ui_font.clone(),
                                ..default()
                            },
                        )
                        .with_justify(JustifyText::Center),
                        ..default()
                    },
                    text,
                    ScaledFontSize { base: CURRENT_WAVE_TEXT_FONT_SIZE },
                ));
            });

            hud.spawn((
                Name::new("Remaining Seconds"),
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        align_self: AlignSelf::End,
                        justify_self: JustifySelf::Start,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(10.00),
                        height: Val::Percent(5.00),
                        top: Val::Percent(8.00),
                        right: Val::Percent(2.00),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("Text"),
                    RemainingSecondsText,
                    TextBundle {
                        text: Text::from_section(
                            format!("{:.0}", wave_duration.as_secs_f32().ceil()),
                            TextStyle {
                                color: REMAINING_SECONDS_TEXT_COLOR,
                                font: ui_font.clone(),
                                ..default()
                            },
                        )
                        .with_justify(JustifyText::Center),
                        ..default()
                    },
                    ScaledFontSize { base: REMAINING_SECONDS_TEXT_FONT_SIZE },
                ));
            });
        });
    }

    commands.insert_resource(wave_durations);
    commands.insert_resource(current_wave);
}

/// Selects the wave from the arguments of the game mode.
pub fn select_wave_when_starting_in_game(
    mut current_wave_text_query: Query<&mut LocalizedText, With<CurrentWaveText>>,
    arguments: Res<Arguments>,
    survival_mode_arguments: Res<SurvivalModeArguments>,
    mut current_wave: ResMut<CurrentWave>,
) {
    if arguments.start_in_game {
        if let Some(wave) = &survival_mode_arguments.start_in_game_waves {
            log::info!("jumping to wave {}", wave);
            *current_wave = CurrentWave(*wave);
            if let Ok(mut current_wave_text) = current_wave_text_query.get_single_mut() {
                *current_wave_text = localization::current_wave(&current_wave);
            }
        }
    }
}


/// Loads the current wave.
pub fn load(
    mut commands: Commands,
    mut remaining_seconds_text_query: Query<&mut Text, With<RemainingSecondsText>>,
    current_wave: Res<CurrentWave>,
    wave_durations: Res<WaveDurations>,
) {
    log::info!("wave: {}", current_wave.0);

    let wave_duration = wave_durations.get(current_wave.index()).copied().unwrap_or(Duration::ZERO);
    log::info!("wave duration: {:?}", wave_duration);
    commands.insert_resource(WaveTimer::new(wave_duration));

    if let Ok(mut remaining_seconds_text) = remaining_seconds_text_query.get_single_mut() {
        remaining_seconds_text.sections[0].value =
            format!("{:.0}", wave_duration.as_secs_f32().ceil());
    }
}

/// Spawns the map.
pub fn spawn_map(mut commands: Commands) {
    log::info!("spawning the map");
    commands.insert_resource(MAP_BOUNDS);
    commands
        .spawn((Name::new("Map"), Map, StateScoped(InChapter), SpatialBundle::default()))
        .with_children(|parent| {
            // Spawn horizontal lines.
            for i in 0..=GRID_SIZE {
                parent.spawn((
                    Name::new(format!("Horizontal Line {}", i + 1)),
                    SpriteBundle {
                        transform: Transform::from_translation(Vec3::new(
                            0.00,
                            (((GRID_SIZE as f32) / 2.00) - (i as f32)) * GRID_SPACING,
                            Depth::Map.z(),
                        )),
                        sprite: Sprite {
                            color: GRID_COLOR,
                            custom_size: Some(Vec2::new(
                                GRID_SIZE as f32 * GRID_SPACING,
                                GRID_THICKNESS,
                            )),
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
            // Spawn vertical lines.
            for i in 0..=GRID_SIZE {
                parent.spawn((
                    Name::new(format!("Vertical Line {}", i + 1)),
                    SpriteBundle {
                        transform: Transform::from_translation(Vec3::new(
                            ((i as f32) - ((GRID_SIZE as f32) / 2.00)) * GRID_SPACING,
                            0.00,
                            Depth::Map.z(),
                        )),
                        sprite: Sprite {
                            color: GRID_COLOR,
                            custom_size: Some(Vec2::new(
                                GRID_THICKNESS,
                                GRID_SIZE as f32 * GRID_SPACING,
                            )),
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
        });
}


/// Ticks the wave timer and wins the current wave when wave timer is finished.
pub fn tick(
    mut remaining_seconds_text_query: Query<&mut Text, With<RemainingSecondsText>>,
    time: Res<Time>,
    mut wave_timer: ResMut<WaveTimer>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    wave_timer.tick(time.delta());
    if let Ok(mut remaining_seconds_text) = remaining_seconds_text_query.get_single_mut() {
        remaining_seconds_text.sections[0].value =
            format!("{:.0}", wave_timer.remaining_secs().ceil());
    }

    if wave_timer.just_finished() {
        game_state_stack.transition(GameState::Won);
        next_game_state.set(GameState::Transition);
    }
}


/// Wins the current wave.
pub fn win(
    mut commands: Commands,
    mut current_wave_text_query: Query<&mut LocalizedText, With<CurrentWaveText>>,
    mut player_query: Query<(&mut Position, &mut RemainingHealth, &Health), With<Player>>,
    mut current_wave: ResMut<CurrentWave>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if current_wave.is_last() {
        log::info!("game is won");
        commands.insert_resource(GameResult::Won);

        game_state_stack.transition(GameState::Over);
        next_game_state.set(GameState::Transition);
    } else {
        let remaining_waves = WAVES - current_wave.get();
        log::info!(
            "{} more wave{} to go",
            remaining_waves,
            if remaining_waves == 1 { "" } else { "s" },
        );

        game_state_stack.pop();
        game_state_stack.push(GameState::Loading);

        if let Ok((mut player_position, mut player_remaining_health, player_health)) =
            player_query.get_single_mut()
        {
            log::info!("resetting the position of the player to [0.00, 0.00]");
            player_position.0 = Vector::ZERO;

            log::info!("resetting the health of the player to {:.2}", player_health.0);
            player_remaining_health.0 = player_health.0;
        }

        next_game_state.set(GameState::Transition);

        current_wave.increment();
        if let Ok(mut current_wave_text) = current_wave_text_query.get_single_mut() {
            *current_wave_text = localization::current_wave(&current_wave);
        }
    }
}


/// Unloads the current wave.
pub fn unload(mut commands: Commands) {
    commands.remove_resource::<WaveTimer>();
}


/// Deinitializes the game mode.
pub fn deinitialize(mut commands: Commands) {
    commands.remove_resource::<CurrentWave>();
    commands.remove_resource::<WaveDurations>();
}
