//! Diagnostics overlay systems.

use {
    mythmallow_core_components::all::*,
    mythmallow_core_constants::ui::widget::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
};


/// Spawns the diagnostics overlay.
pub fn spawn_diagnostics_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    diagnostics: Res<DiagnosticsStore>,
    ui_font: Option<Res<UiFont>>,
    mut general_settings: ResMut<Persistent<GeneralSettings>>,
) {
    log::info!("spawning the diagnostics overlay");

    let font = match ui_font {
        Some(font) => font.clone(),
        None => {
            log::info!("loading the ui font");
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            commands.insert_resource(UiFont(font.clone()));
            font
        },
    };

    let entity_count = match diagnostics
        .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|entity_count| entity_count.smoothed())
    {
        Some(entity_count) => format!("{entity_count:.0}"),
        None => "N/A".to_owned(),
    };
    let fps = match diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.smoothed())
    {
        Some(fps) => format!("{fps:.0}"),
        None => "N/A".to_owned(),
    };
    let frame_time = match diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|frame_time| frame_time.smoothed())
    {
        Some(frame_time) => format!("{frame_time:.3}"),
        None => "N/A".to_owned(),
    };

    commands
        .ui_builder(UiRoot)
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(35.00),
                height: Val::Percent(12.00),
                bottom: Val::Percent(1.00),
                left: Val::Percent(1.00),
                ..default()
            },
            ..default()
        })
        .named("Diagnostics Overlay")
        .spawn((
            Name::new("Text"),
            DiagnosticsOverlayText,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "Entities: ".into(),
                        style: TextStyle {
                            color: DIAGNOSTICS_OVERLAY_METRIC_COLOR,
                            font: font.clone(),
                            ..default()
                        },
                    },
                    TextSection {
                        value: entity_count,
                        style: TextStyle {
                            color: DIAGNOSTICS_OVERLAY_VALUE_COLOR,
                            font: font.clone(),
                            ..default()
                        },
                    },
                    TextSection {
                        value: "\nFPS: ".into(),
                        style: TextStyle {
                            color: DIAGNOSTICS_OVERLAY_METRIC_COLOR,
                            font: font.clone(),
                            ..default()
                        },
                    },
                    TextSection {
                        value: fps,
                        style: TextStyle {
                            color: DIAGNOSTICS_OVERLAY_VALUE_COLOR,
                            font: font.clone(),
                            ..default()
                        },
                    },
                    TextSection {
                        value: "\nFrame Time: ".into(),
                        style: TextStyle {
                            color: DIAGNOSTICS_OVERLAY_METRIC_COLOR,
                            font: font.clone(),
                            ..default()
                        },
                    },
                    TextSection {
                        value: frame_time,
                        style: TextStyle {
                            color: DIAGNOSTICS_OVERLAY_VALUE_COLOR,
                            font: font.clone(),
                            ..default()
                        },
                    },
                ]),
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(1.00),
                    left: Val::Percent(1.00),
                    ..default()
                },
                ..default()
            },
            ScaledFontSize { base: DIAGNOSTICS_OVERLAY_FONT_SIZE },
        ))
        .style()
        .z_index(ZIndex::Global(100));

    general_settings.show_diagnostics_overlay = true;
    general_settings.persist().ok();
}

/// Despawns the diagnostics overlay.
pub fn despawn_diagnostics_overlay(
    mut commands: Commands,
    query: Query<Entity, With<DiagnosticsOverlayText>>,
    mut general_settings: ResMut<Persistent<GeneralSettings>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    log::info!("despawning the diagnostics overlay");

    general_settings.show_diagnostics_overlay = false;
    general_settings.persist().ok();
}


/// Updates the diagnostics overlay.
pub fn update_diagnostics_overlay(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<DiagnosticsOverlayText>>,
) {
    let mut text = match query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let entity_count = match diagnostics
        .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|entity_count| entity_count.smoothed())
    {
        Some(entity_count) => format!("{entity_count:.0}"),
        None => "N/A".to_owned(),
    };
    let fps = match diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.smoothed())
    {
        Some(fps) => format!("{fps:.0}"),
        None => "N/A".to_owned(),
    };
    let frame_time = match diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|frame_time| frame_time.smoothed())
    {
        Some(frame_time) => format!("{frame_time:.3}"),
        None => "N/A".to_owned(),
    };

    text.sections[1].value = entity_count;
    text.sections[3].value = fps;
    text.sections[5].value = frame_time;
}
