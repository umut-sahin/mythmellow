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
                        .insert(SettingsMenuPreviousLanguageButton);

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
                        .insert(SettingsMenuNextLanguageButton);
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
                .insert(SettingsMenuBackButton)
                .insert(WidgetSelected::now());
        })
        .named("Settings Menu")
        .insert((SettingsMenu, StateScoped(InSettingsMenu)))
        .style()
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .row_gap(Val::Percent(1.50));
}


/// Sets the locale to the previous supported locale.
pub fn previous_language_button_interaction(
    mut commands: Commands,
    mut previous_language_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<SettingsMenuPreviousLanguageButton>),
    >,
    supported_locales: Res<SupportedLocales>,
    locale: Res<Locale>,
    registered_systems: Res<RegisteredSystems>,
) {
    if let Ok(mut button) = previous_language_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("previous language button is clicked");

            let new_locale_position = supported_locales
                .iter()
                .position(|supported_locale| supported_locale == &locale.requested)
                .map(
                    |position| {
                        if position == 0 { supported_locales.len() - 1 } else { position - 1 }
                    },
                )
                .unwrap_or(0);

            commands.run_system_with_input(
                registered_systems.localization.set_locale,
                supported_locales[new_locale_position].clone(),
            )
        });
    }
}

/// Sets the locale to the next supported locale.
pub fn next_language_button_interaction(
    mut commands: Commands,
    mut next_language_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<SettingsMenuNextLanguageButton>),
    >,
    supported_locales: Res<SupportedLocales>,
    locale: Res<Locale>,
    registered_systems: Res<RegisteredSystems>,
) {
    if let Ok(mut button) = next_language_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("next language button is clicked");

            let new_locale_position = supported_locales
                .iter()
                .position(|supported_locale| supported_locale == &locale.requested)
                .map(
                    |position| {
                        if position >= supported_locales.len() - 1 { 0 } else { position + 1 }
                    },
                )
                .unwrap_or(0);

            commands.run_system_with_input(
                registered_systems.localization.set_locale,
                supported_locales[new_locale_position].clone(),
            )
        });
    }
}

/// Transitions to the main menu.
pub fn back_button_interaction(
    mut commands: Commands,
    mut back_button_query: Query<&mut Widget, (Changed<Widget>, With<SettingsMenuBackButton>)>,
    app_state: ResMut<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    game_state_stack: Option<ResMut<GameStateStack>>,
    next_game_state: Option<ResMut<NextState<GameState>>>,
) {
    if let Ok(mut button) = back_button_query.get_single_mut() {
        button.on_click(|| {
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
        });
    }
}
