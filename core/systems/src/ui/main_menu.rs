//! Main menu systems.

use {
    crate::utility::*,
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_localizations::ui as localization,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
};


/// Spawns the main menu.
pub fn spawn_main_menu(
    mut commands: Commands,
    localization: Res<Localization>,
    ui_font: Res<UiFont>,
) {
    log::info!("spawning the main menu");
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            // Spawn play button.
            column
                .menu_button(localization::play_button(), localization.deref(), ui_font.clone())
                .named("Play Button")
                .insert(MainMenuPlayButton)
                .insert(WidgetSelected::now());

            // Spawn settings button.
            column
                .menu_button(localization::settings_button(), localization.deref(), ui_font.clone())
                .named("Settings Button")
                .insert(MainMenuSettingsButton);

            // Spawn quit button.
            column
                .menu_button(localization::quit_button(), localization.deref(), ui_font.clone())
                .named("Quit Button")
                .insert(MainMenuQuitButton);
        })
        .named("Main Menu")
        .insert((MainMenu, StateScoped(AppState::MainMenu)))
        .style()
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .row_gap(Val::Percent(1.50));
}


/// Transitions to the game mode selection screen.
pub fn play_button_interaction(
    mut commands: Commands,
    mut play_button_query: Query<&mut Widget, (Changed<Widget>, With<MainMenuPlayButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
    registered_systems: Res<RegisteredSystems>,
) {
    if let Ok(mut button) = play_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("play button is clicked");
            log::info!("transitioning to the game mode selection screen");

            commands.run_system(registered_systems.widget.save_selected_widget);
            next_app_state.set(AppState::GameModeSelectionScreen);
        });
    }
}

/// Transitions to the settings menu.
pub fn settings_button_interaction(
    mut commands: Commands,
    mut settings_button_query: Query<&mut Widget, (Changed<Widget>, With<MainMenuSettingsButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
    registered_systems: Res<RegisteredSystems>,
) {
    if let Ok(mut button) = settings_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("settings button is clicked");
            log::info!("transitioning to the settings menu");

            commands.run_system(registered_systems.widget.save_selected_widget);
            next_app_state.set(AppState::SettingsMenu);
        });
    }
}

#[cfg(not(target_family = "wasm"))]
/// Quits the application.
pub fn quit_button_interaction(
    mut quit_button_query: Query<&mut Widget, (Changed<Widget>, With<MainMenuQuitButton>)>,
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
    mut quit_button_query: Query<&mut Widget, (Changed<Widget>, With<MainMenuQuitButton>)>,
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
