//! Main menu systems.

use {
    crate::{
        prelude::*,
        systems::utility::*,
    },
    localizations::ui as localization,
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
        .column(|root| {
            // Spawn the play button.
            root.menu_button(localization::play_button(), localization.deref(), ui_font.clone())
                .named("Play Button")
                .insert(WidgetSelected::now())
                .entity_commands()
                .observe(on_play_button_clicked);

            // Spawn the settings button.
            root.menu_button(
                localization::settings_button(),
                localization.deref(),
                ui_font.clone(),
            )
            .named("Settings Button")
            .entity_commands()
            .observe(on_settings_button_clicked);

            // Spawn the quit button.
            root.menu_button(localization::quit_button(), localization.deref(), ui_font.clone())
                .named("Quit Button")
                .entity_commands()
                .observe(on_quit_button_clicked);
        })
        .named("Main Menu")
        .insert(StateScoped(AppState::MainMenu))
        .style()
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .row_gap(Val::Percent(1.50));
}


/// Transitions to the game mode selection screen.
pub fn on_play_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut next_app_state: ResMut<NextState<AppState>>,
    registered_systems: Res<RegisteredSystems>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("play button is clicked");
    log::info!("transitioning to the game mode selection screen");

    commands.run_system(registered_systems.widget.save_selected_widget);
    next_app_state.set(AppState::GameModeSelectionScreen);
}

/// Transitions to the settings menu.
pub fn on_settings_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut next_app_state: ResMut<NextState<AppState>>,
    registered_systems: Res<RegisteredSystems>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("settings button is clicked");
    log::info!("transitioning to the settings menu");

    commands.run_system(registered_systems.widget.save_selected_widget);
    next_app_state.set(AppState::SettingsMenu);
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
