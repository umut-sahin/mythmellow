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
                .insert(WidgetSelected::now())
                .entity_commands()
                .observe(on_resume_button_clicked);

            // Spawn restart button.
            column
                .menu_button(localization::restart_button(), localization.deref(), ui_font.clone())
                .named("Restart Button")
                .entity_commands()
                .observe(on_restart_button_clicked);

            // Spawn settings button.
            column
                .menu_button(localization::settings_button(), localization.deref(), ui_font.clone())
                .named("Settings Button")
                .entity_commands()
                .observe(on_settings_button_clicked);

            // Spawn return to main menu button.
            column
                .menu_button(
                    localization::return_to_main_menu_button(),
                    localization.deref(),
                    ui_font.clone(),
                )
                .named("Return To Main Menu Button")
                .entity_commands()
                .observe(on_return_to_main_menu_button_clicked);

            // Spawn quit to desktop button.
            column
                .menu_button(
                    localization::quit_to_desktop_button(),
                    localization.deref(),
                    ui_font.clone(),
                )
                .named("Quit To Desktop Button")
                .entity_commands()
                .observe(on_quit_button_clicked);
        })
        .named("Pause Menu")
        .insert(StateScoped(GameState::Paused))
        .style()
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .row_gap(Val::Percent(1.50));
}


/// Resumes the game.
pub fn on_resume_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("resume button is clicked");

    game_state_stack.pop();
    next_game_state.set(GameState::Transition);
}

/// Restarts the game.
pub fn on_restart_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("restart button is clicked");

    game_state_stack.clear();
    game_state_stack.push(GameState::Restart);
    next_game_state.set(GameState::Transition);
}

/// Transitions to the settings menu.
pub fn on_settings_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
    registered_systems: Res<RegisteredSystems>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("settings button is clicked");

    commands.run_system(registered_systems.widget.save_selected_widget);
    game_state_stack.push(GameState::SettingsMenu);
    next_game_state.set(GameState::Transition);
}

/// Transitions to the main menu.
pub fn on_return_to_main_menu_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("return to main menu button is clicked");
    log::info!("transitioning to the main menu");

    next_app_state.set(AppState::MainMenu);
}

/// Quits the application.
pub fn on_quit_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("quit button is clicked");
    log::info!("closing the application");

    quit(&mut app_exit_event_writer);
}
