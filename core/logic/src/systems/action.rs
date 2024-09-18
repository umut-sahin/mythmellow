//! Action systems.

use crate::prelude::*;


/// Pauses the game when the application loses it's focus.
pub fn pause_on_losing_focus(
    mut window_focused_reader: EventReader<WindowFocused>,
    general_settings: Res<Persistent<GeneralSettings>>,
    game_state: Res<State<GameState>>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for event in window_focused_reader.read() {
        if !event.focused
            && general_settings.pause_on_losing_focus
            && game_state.get() == &GameState::Playing
        {
            game_state_stack.push(GameState::Paused);
            next_game_state.set(GameState::Transition);
            break;
        }
    }
}

/// Toggles the window mode between fullscreen and windowed.
#[cfg(not(target_family = "wasm"))]
pub fn toggle_fullscreen(
    global_action_state: Res<ActionState<GlobalAction>>,
    mut window_state_query: Query<&mut Persistent<WindowState>, With<PrimaryWindow>>,
) {
    if global_action_state.just_pressed(&GlobalAction::ToggleFullscreen) {
        log::info!("toggle fullscreen action is triggered");
        window_state_query
            .single_mut()
            .update(|window_state| {
                window_state.mode = match window_state.mode {
                    WindowMode::Windowed => {
                        log::info!("enabling fullscreen");
                        WindowMode::BorderlessFullscreen
                    },
                    _ => {
                        log::info!("disabling fullscreen");
                        WindowMode::Windowed
                    },
                };
            })
            .ok();
    }
}

/// Toggles the window mode of the browser between fullscreen and windowed.
#[cfg(target_family = "wasm")]
pub fn toggle_fullscreen(global_action_state: Res<ActionState<GlobalAction>>) {
    if global_action_state.just_pressed(&GlobalAction::ToggleFullscreen) {
        log::info!("toggle fullscreen action is triggered");

        let window = match web_sys::window() {
            Some(window) => window,
            None => {
                log::error!("unable to get the window to toggle fullscreen");
                return;
            },
        };

        let document = match window.document() {
            Some(document) => document,
            None => {
                log::error!("unable to get the document to toggle fullscreen");
                return;
            },
        };

        let element = match document.document_element() {
            Some(element) => element,
            None => {
                log::error!("unable to get the document element to toggle fullscreen");
                return;
            },
        };

        if let Err(error) = element.request_fullscreen() {
            log::error!("unable to toggle fullscreen ({:?})", error);
        } else {
            log::info!("toggling fullscreen");
        }
    }
}

/// Toggles the diagnostics overlay.
pub fn toggle_diagnostics_overlay(
    diagnostics_overlay_state: Res<State<DiagnosticsOverlayState>>,
    global_action_state: Res<ActionState<GlobalAction>>,
    mut next_diagnostics_overlay_state: ResMut<NextState<DiagnosticsOverlayState>>,
) {
    if global_action_state.just_pressed(&GlobalAction::ToggleDiagnosticsOverlay) {
        log::info!("toggle diagnostics overlay action is triggered");
        next_diagnostics_overlay_state.set(match diagnostics_overlay_state.get() {
            DiagnosticsOverlayState::Enabled => {
                log::info!("disabling the diagnostics overlay");
                DiagnosticsOverlayState::Disabled
            },
            DiagnosticsOverlayState::Disabled => {
                log::info!("enabling the diagnostics overlay");
                DiagnosticsOverlayState::Enabled
            },
        });
    }
}

/// Toggles the physics gizmos.
#[cfg(feature = "development")]
pub fn toggle_physics_gizmos(
    global_action_state: Res<ActionState<GlobalAction>>,
    mut general_settings: ResMut<Persistent<GeneralSettings>>,
    mut gizmo_config_store: ResMut<GizmoConfigStore>,
) {
    if global_action_state.just_pressed(&GlobalAction::TogglePhysicsGizmos) {
        general_settings
            .update(|general_settings| {
                log::info!("toggle physics gizmos action is triggered");
                if general_settings.enable_physics_gizmos {
                    log::info!("disabling the physics gizmos");
                } else {
                    log::info!("enabling the physics gizmos");
                }
                general_settings.enable_physics_gizmos = !general_settings.enable_physics_gizmos;
            })
            .ok();

        let (physics_gizmos_config, _) = gizmo_config_store.config_mut::<PhysicsGizmos>();
        physics_gizmos_config.enabled = general_settings.enable_physics_gizmos;
    }
}


/// Pauses the game.
pub fn pause(
    mut game_action_state_query: Query<&mut ActionState<GameAction>, With<Player>>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut game_action_state) = game_action_state_query.get_single_mut() {
        if game_action_state.just_pressed(&GameAction::Pause) {
            log::info!("pause action is triggered");
            game_state_stack.push(GameState::Paused);
            next_game_state.set(GameState::Transition);
            game_action_state.disable();
        }
    }
}

/// Opens the market.
pub fn open_market(
    game_action_state_query: Query<&ActionState<GameAction>, With<Player>>,
    game_mode_registry: Res<GameModeRegistry>,
    selected_game_mode_index: Res<GameModeIndex>,
    market_configuration: Res<MarketConfiguration>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(game_action_state) = game_action_state_query.get_single() {
        if game_action_state.just_pressed(&GameAction::OpenMarket) {
            log::info!("open market action is triggered");
            if market_configuration.can_be_opened_by_player {
                game_state_stack.push(GameState::Market);
                next_game_state.set(GameState::Transition);
            } else {
                log::warn!(
                    "market cannot be opened by the player in {:?} mode",
                    game_mode_registry[selected_game_mode_index.0].id(),
                );
            }
        }
    }
}

/// Closes the market.
pub fn close_market(
    game_action_state_query: Query<&ActionState<GameAction>, With<Player>>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(game_action_state) = game_action_state_query.get_single() {
        if game_action_state.just_pressed(&GameAction::OpenMarket) {
            log::info!("close market action is triggered");
            game_state_stack.pop();
            next_game_state.set(GameState::Transition);
        }
    }
}
