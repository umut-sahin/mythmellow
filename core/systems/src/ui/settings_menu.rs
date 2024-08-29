//! Settings menu systems.

use {
    crate::utility::*,
    mythmallow_core_assets::all::*,
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_localizations::ui as localization,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
};


/// Spawns the settings menu.
pub fn spawn_settings_menu(
    mut commands: Commands,
    localization: Res<Localization>,
    ui_font: Res<UiFont>,
) {
    log::info!("spawning the settings menu");
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            // Spawn language settings.
            column
                .container(NodeBundle::default(), |container| {
                    // Spawn previous button.
                    container
                        .change_setting_button(
                            LocalizedText::Constant { text: "<".into() },
                            localization.deref(),
                            ui_font.clone(),
                        )
                        .named("Previous Button")
                        .entity_commands()
                        .observe(on_previous_language_button_click);

                    // Spawn setting name.
                    container
                        .setting_name(
                            localization::language_setting_name(),
                            localization.deref(),
                            ui_font.clone(),
                        )
                        .named("Name");

                    // Spawn setting value.
                    container
                        .setting_value(
                            localization::language_setting_value(),
                            localization.deref(),
                            ui_font.clone(),
                        )
                        .named("Value");

                    // Spawn next button.
                    container
                        .change_setting_button(
                            LocalizedText::Constant { text: ">".into() },
                            localization.deref(),
                            ui_font.clone(),
                        )
                        .named("Next Button")
                        .entity_commands()
                        .observe(on_next_language_button_click);
                })
                .named("Language Setting")
                .style()
                .width(Val::Percent(80.00))
                .height(Val::Percent(25.00))
                .column_gap(Val::Percent(1.00))
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::Center);

            // Spawn back button.
            column
                .menu_button(localization::back_button(), localization.deref(), ui_font.clone())
                .named("Back Button")
                .insert(WidgetSelected::now())
                .entity_commands()
                .observe(on_back_button_click);
        })
        .named("Settings Menu")
        .insert(StateScoped(InSettingsMenu))
        .style()
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .row_gap(Val::Percent(1.50));
}


/// Sets the locale to the previous supported locale.
pub fn on_previous_language_button_click(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    supported_locales: Res<SupportedLocales>,
    locale: Res<Locale>,
    registered_systems: Res<RegisteredSystems>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("previous language button is clicked");

    let new_locale_position = supported_locales
        .iter()
        .position(|supported_locale| supported_locale == &locale.requested)
        .map(|position| if position == 0 { supported_locales.len() - 1 } else { position - 1 })
        .unwrap_or(0);

    commands.run_system_with_input(
        registered_systems.localization.set_locale,
        supported_locales[new_locale_position].clone(),
    )
}

/// Sets the locale to the next supported locale.
pub fn on_next_language_button_click(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    supported_locales: Res<SupportedLocales>,
    locale: Res<Locale>,
    registered_systems: Res<RegisteredSystems>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("next language button is clicked");

    let new_locale_position = supported_locales
        .iter()
        .position(|supported_locale| supported_locale == &locale.requested)
        .map(|position| if position >= supported_locales.len() - 1 { 0 } else { position + 1 })
        .unwrap_or(0);

    commands.run_system_with_input(
        registered_systems.localization.set_locale,
        supported_locales[new_locale_position].clone(),
    )
}

/// Transitions to the main menu.
pub fn on_back_button_click(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    app_state: ResMut<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    game_state_stack: Option<ResMut<GameStateStack>>,
    next_game_state: Option<ResMut<NextState<GameState>>>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("back button is clicked");

    commands.insert_resource(RestorePreviouslySelectedWidget);
    match app_state.get() {
        AppState::Game => {
            log::info!("transitioning to the pause menu");
            game_state_stack.unwrap().pop();
            next_game_state.unwrap().set(GameState::Transition);
        },
        _ => {
            log::info!("transitioning to the main menu");
            next_app_state.set(AppState::MainMenu);
        },
    }
}
