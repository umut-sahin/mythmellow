//! Player selection screen systems.

use {
    crate::utility::*,
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_localizations::ui as localization,
    mythmallow_core_registries::all::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
};


/// Spawns the player selection screen.
pub fn spawn_player_selection_screen(
    mut commands: Commands,
    localization: Res<Localization>,
    ui_font: Res<UiFont>,
    player_registry: Res<PlayerRegistry>,
) {
    log::info!("spawning the player selection screen");
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            if player_registry.is_empty() {
                // Spawn no players title.
                column.menu_title(
                    localization::menu_title_no_players(),
                    &localization,
                    ui_font.clone(),
                );
            } else {
                // Spawn player buttons.
                for (mythology_index, entry) in player_registry.iter().enumerate() {
                    for (player_index, player) in entry.players.iter().enumerate() {
                        let mut player_button = column.menu_button(
                            player.name(),
                            localization.deref(),
                            ui_font.clone(),
                        );
                        player_button.named(format!("Player Button [{}]", player.id())).insert(
                            PlayerSelectionScreenPlayerButton { mythology_index, player_index },
                        );
                        if mythology_index == 0 && player_index == 0 {
                            player_button.insert(WidgetSelected::now());
                        }
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
                back_button.named("Back Button").insert(PlayerSelectionScreenBackButton);
                if player_registry.is_empty() {
                    back_button.insert(WidgetSelected::now());
                }
            }
        })
        .named("Player Selection Screen")
        .insert((PlayerSelectionScreen, StateScoped(AppState::PlayerSelectionScreen)))
        .style()
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .row_gap(Val::Percent(1.50));
}


/// Transitions to the enemy selection screen.
pub fn player_button_interaction(
    mut commands: Commands,
    mut player_button_query: Query<
        (&mut Widget, &PlayerSelectionScreenPlayerButton),
        Changed<Widget>,
    >,
    player_registry: Res<PlayerRegistry>,
    mut next_app_state: ResMut<NextState<AppState>>,
    registered_systems: Res<RegisteredSystems>,
) {
    if let Ok((mut button, metadata)) = player_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!(
                "{:?} player button is clicked",
                player_registry[metadata.mythology_index].players[metadata.player_index].id(),
            );
            log::info!("transitioning to the enemy selection screen");

            commands.insert_resource(MythologyIndex(metadata.mythology_index));
            commands.insert_resource(PlayerIndex(metadata.player_index));

            commands.run_system(registered_systems.widget.save_selected_widget);
            next_app_state.set(AppState::EnemySelectionScreen);
        });
    }
}

/// Transitions to the game mode selection screen.
pub fn back_button_interaction(
    mut commands: Commands,
    mut back_button_query: Query<
        &mut Widget,
        (Changed<Widget>, With<PlayerSelectionScreenBackButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(mut button) = back_button_query.get_single_mut() {
        button.on_click(|| {
            log::info!("back button is clicked");
            log::info!("transitioning to the game mode selection screen");

            commands.insert_resource(RestorePreviouslySelectedWidget);
            next_app_state.set(AppState::GameModeSelectionScreen);
        });
    }
}
