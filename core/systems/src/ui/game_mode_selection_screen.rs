//! Game mode selection screen systems.

use {
    crate::utility::*,
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_localizations::ui as localization,
    mythmallow_core_registries::all::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
};


/// Spawns the game mode selection screen.
pub fn spawn_game_mode_selection_screen(
    mut commands: Commands,
    localization: Res<Localization>,
    ui_font: Res<UiFont>,
    game_mode_registry: Res<GameModeRegistry>,
) {
    log::info!("spawning the game mode selection screen");
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            if game_mode_registry.is_empty() {
                // Spawn no game modes title.
                column.menu_title(
                    localization::menu_title_no_modes(),
                    &localization,
                    ui_font.clone(),
                );
            } else {
                // Spawn game mode buttons.
                for (game_mode_index, game_mode) in game_mode_registry.iter().enumerate() {
                    let mut game_mode_button =
                        column.menu_button(game_mode.name(), localization.deref(), ui_font.clone());
                    game_mode_button
                        .named(format!("Game Mode Button [{}]", game_mode.id()))
                        .insert(GameModeSelectionScreenGameModeButton { game_mode_index });
                    if game_mode_index == 0 {
                        game_mode_button.insert(WidgetSelected::now());
                    }
                }
            }

            // Spawn back button.
            {
                let mut back_button = column.menu_button(
                    localization::back_button(),
                    localization.deref(),
                    ui_font.clone(),
                );
                back_button.named("Back Button").insert(GameModeSelectionScreenBackButton);
                if game_mode_registry.is_empty() {
                    back_button.insert(WidgetSelected::now());
                }
            }
        })
        .named("Game Mode Selection Screen")
        .insert((GameModeSelectionScreen, StateScoped(AppState::GameModeSelectionScreen)))
        .style()
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .row_gap(Val::Percent(1.50));
}


/// Transitions to the player selection screen.
pub fn game_mode_button_interaction(
    mut commands: Commands,
    mut game_mode_button_query: Query<
        (&mut Widget, &GameModeSelectionScreenGameModeButton),
        Changed<Widget>,
    >,
    game_mode_registry: Res<GameModeRegistry>,
    mut next_app_state: ResMut<NextState<AppState>>,
    registered_systems: Res<RegisteredSystems>,
) {
    if let Ok((mut button, metadata)) = game_mode_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!(
                "{:?} game mode button is clicked",
                game_mode_registry[metadata.game_mode_index].id()
            );
            log::info!("transitioning to the player selection screen");

            commands.insert_resource(GameModeIndex(metadata.game_mode_index));

            commands.run_system(registered_systems.widget.save_selected_widget);
            next_app_state.set(AppState::PlayerSelectionScreen);
        });
    }
}

/// Transitions to the main menu.
pub fn back_button_interaction(
    mut commands: Commands,
    mut back_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<GameModeSelectionScreenBackButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(mut button) = back_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("back button is clicked");
            log::info!("transitioning to the main menu");

            commands.insert_resource(RestorePreviouslySelectedWidget);
            next_app_state.set(AppState::MainMenu);
        });
    }
}
