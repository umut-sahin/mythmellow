//! Head-up display systems.

use {
    mythmallow_core_assets::all::*,
    mythmallow_core_components::all::*,
    mythmallow_core_constants::ui::widget::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_localizations::ui as localization,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
};


/// Spawns the head-up display.
pub fn spawn_hud(
    mut commands: Commands,
    player_query: Query<(&Health, &RemainingHealth, &Level), With<Player>>,
    localization: Res<Localization>,
    ui_font: Res<UiFont>,
    mut bar_materials: ResMut<Assets<BarMaterial>>,
) {
    log::info!("spawning the head-up display");

    let (health_bar_text, experience_bar_text) = match player_query.get_single() {
        Ok((health, remaining_health, level)) => {
            (
                format!("{} / {}", remaining_health.ceil(), health.ceil()),
                localization::experience_bar(level),
            )
        },
        Err(_) => ("? / ?".to_owned(), "?".into()),
    };

    commands
        .ui_builder(UiRoot)
        .container((Hud, NodeBundle::default(), StateScoped(InGame)), |hud| {
            // Spawn the health bar.
            hud.container(
                (
                    Name::new("Health Bar"),
                    HudHealthBar,
                    MaterialNodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            align_self: AlignSelf::Start,
                            justify_self: JustifySelf::Start,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Percent(15.00),
                            height: Val::Percent(5.00),
                            top: Val::Percent(3.00),
                            left: Val::Percent(1.50),
                            ..default()
                        },
                        material: bar_materials.add(BarMaterial::for_health()),
                        ..default()
                    },
                ),
                |health_bar| {
                    health_bar.spawn((
                        Name::new("Text"),
                        HudHealthBarText,
                        TextBundle {
                            text: Text::from_section(
                                health_bar_text,
                                TextStyle {
                                    color: HEALTH_BAR_TEXT_COLOR,
                                    font: ui_font.clone(),
                                    ..default()
                                },
                            )
                            .with_justify(JustifyText::Center),
                            ..default()
                        },
                        ScaledFontSize { base: HEALTH_BAR_TEXT_FONT_SIZE },
                    ));
                },
            );
            // Spawn the experience bar.
            hud.container(
                (
                    Name::new("Experience Bar"),
                    HudExperienceBar,
                    MaterialNodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            align_self: AlignSelf::Start,
                            justify_self: JustifySelf::Start,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Percent(15.00),
                            height: Val::Percent(5.00),
                            top: Val::Percent(8.50),
                            left: Val::Percent(1.50),
                            ..default()
                        },
                        material: bar_materials.add(BarMaterial::for_experience()),
                        ..default()
                    },
                ),
                |experience_bar| {
                    experience_bar.spawn((
                        Name::new("Text"),
                        HudExperienceBarText,
                        TextBundle {
                            text: Text::from_section(
                                experience_bar_text.get(&localization),
                                TextStyle {
                                    color: EXPERIENCE_BAR_TEXT_COLOR,
                                    font: ui_font.clone(),
                                    ..default()
                                },
                            )
                            .with_justify(JustifyText::Center),
                            ..default()
                        },
                        experience_bar_text,
                        ScaledFontSize { base: EXPERIENCE_BAR_TEXT_FONT_SIZE },
                    ));
                },
            );
        })
        .named("Head-up Display")
        .style()
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .z_index(ZIndex::Global(-200));
}


/// Updates the health bar of the player.
pub fn update_player_health_bar(
    player_query: Query<
        (&Health, &RemainingHealth),
        (With<Player>, Or<(Changed<Health>, Changed<RemainingHealth>)>),
    >,
    health_bar_query: Query<&Handle<BarMaterial>, With<HudHealthBar>>,
    mut health_bar_text_query: Query<&mut Text, With<HudHealthBarText>>,
    mut bar_materials: ResMut<Assets<BarMaterial>>,
) {
    let (player_health, player_remaining_health) = match player_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let health_bar_handle = match health_bar_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };
    let health_bar = match bar_materials.get_mut(health_bar_handle) {
        Some(asset) => asset,
        None => return,
    };

    let mut health_bar_text = match health_bar_text_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    health_bar.percent = (player_remaining_health.0 / player_health.0).clamp(0.00, 1.00);
    health_bar_text.sections[0].value =
        format!("{} / {}", player_remaining_health.ceil(), player_health.ceil());
}

/// Updates the experience bar of the player.
pub fn update_player_experience_bar(
    player_query: Query<
        (&Experience, &Level),
        (With<Player>, Or<(Changed<Experience>, Changed<Level>)>),
    >,
    experience_bar_query: Query<&Handle<BarMaterial>, With<HudExperienceBar>>,
    mut experience_bar_text_query: Query<&mut LocalizedText, With<HudExperienceBarText>>,
    mut bar_materials: ResMut<Assets<BarMaterial>>,
    experience_required_to_get_to_current_level: Res<ExperienceRequiredToGetToCurrentLevel>,
    experience_required_to_level_up: Res<ExperienceRequiredToLevelUp>,
) {
    let (player_experience, player_level) = match player_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let experience_bar_handle = match experience_bar_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };
    let experience_bar = match bar_materials.get_mut(experience_bar_handle) {
        Some(asset) => asset,
        None => return,
    };

    let mut experience_bar_text = match experience_bar_text_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let experience_collected =
        player_experience.0 - experience_required_to_get_to_current_level.0.0;

    let experience_required_for_level_up =
        experience_required_to_level_up.0.0 - experience_required_to_get_to_current_level.0.0;

    experience_bar.percent =
        (experience_collected / experience_required_for_level_up).clamp(0.00, 1.00) as f32;
    *experience_bar_text = localization::experience_bar(player_level);
}
