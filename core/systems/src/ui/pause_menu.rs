//! Pause menu systems.

use {
    crate::utility::*,
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_localizations::ui as localization,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
};

/// Spawns the pause menu.
pub fn spawn_pause_menu(
    mut commands: Commands,
    localization: Res<Localization>,
    ui_font: Res<UiFont>,
) {
    log::info!("spawning the pause menu");
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            // Spawn resume button.
            column
                .menu_button(localization::resume_button(), localization.deref(), ui_font.clone())
                .named("Resume Button")
                .insert(PauseMenuResumeButton)
                .insert(WidgetSelected::now());

            // Spawn restart button.
            column
                .menu_button(localization::restart_button(), localization.deref(), ui_font.clone())
                .named("Restart Button")
                .insert(PauseMenuRestartButton);

            // Spawn settings button.
            column
                .menu_button(localization::settings_button(), localization.deref(), ui_font.clone())
                .named("Settings Button")
                .insert(PauseMenuSettingsButton);

            // Spawn return to main menu button.
            column
                .menu_button(
                    localization::return_to_main_menu_button(),
                    localization.deref(),
                    ui_font.clone(),
                )
                .named("Return To Main Menu Button")
                .insert(PauseMenuReturnToMainMenuButton);

            // Spawn quit to desktop button.
            column
                .menu_button(
                    localization::quit_to_desktop_button(),
                    localization.deref(),
                    ui_font.clone(),
                )
                .named("Quit Button")
                .insert(PauseMenuQuitToDesktopButton);
        })
        .named("Pause Menu")
        .insert((PauseMenu, StateScoped(GameState::Paused)))
        .style()
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .row_gap(Val::Percent(1.50));
}


/// Resumes the game.
pub fn resume_button_interaction(
    mut resume_button_query: Query<&mut Widget, (Changed<Widget>, With<PauseMenuResumeButton>)>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut button) = resume_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("resume button is clicked");

            game_state_stack.pop();
            next_game_state.set(GameState::Transition);
        });
    }
}

/// Restarts the game.
pub fn restart_button_interaction(
    mut restart_button_query: Query<&mut Widget, (Changed<Widget>, With<PauseMenuRestartButton>)>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut button) = restart_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("restart button is clicked");

            game_state_stack.clear();
            game_state_stack.push(GameState::Restart);
            next_game_state.set(GameState::Transition);
        });
    }
}

/// Transitions to the settings menu.
pub fn settings_button_interaction(
    mut commands: Commands,
    mut settings_button_query: Query<&mut Widget, (Changed<Widget>, With<PauseMenuSettingsButton>)>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
    registered_systems: Res<RegisteredSystems>,
) {
    if let Ok(mut button) = settings_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("settings button is clicked");

            commands.run_system(registered_systems.widget.save_selected_widget);
            game_state_stack.push(GameState::SettingsMenu);
            next_game_state.set(GameState::Transition);
        });
    }
}

/// Transitions to the main menu.
pub fn return_to_main_menu_button_interaction(
    mut return_to_main_menu_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<PauseMenuReturnToMainMenuButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(mut button) = return_to_main_menu_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("return to main menu button is clicked");
            log::info!("transitioning to the main menu");

            next_app_state.set(AppState::MainMenu);
        });
    }
}

#[cfg(not(target_family = "wasm"))]
/// Quits the application.
pub fn quit_button_interaction(
    mut quit_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<PauseMenuQuitToDesktopButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok(mut button) = quit_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("quit button is clicked");
            log::info!("closing the application");

            app_exit_event_writer.send(AppExit::Success);
        });
    }
}

#[cfg(target_family = "wasm")]
/// Quits the application.
pub fn quit_button_interaction(
    mut quit_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<PauseMenuQuitToDesktopButton>),
    >,
) {
    if let Ok(mut button) = quit_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("quit button is clicked");
            log::info!("closing the tab");

            let window = match web_sys::window() {
                Some(window) => window,
                None => {
                    log::error!("unable to get the window to close");
                    return;
                },
            };
            if let Err(error) = window.close() {
                log::error!("unable to close the window ({:?})", error);
            }
        });
    }
}
