// Disable spawning command prompt on Windows outside development mode.
#![cfg_attr(not(feature = "development"), windows_subsystem = "windows")]

use mythmallow::{
    content::{
        enemies::sweet::prelude::*,
        items::greek::prelude::*,
        modes::survival::prelude::*,
        players::greek::prelude::*,
    },
    core::{
        dependencies::*,
        plugins::*,
        registries::all::*,
    },
};

fn main() -> AppExit {
    #[cfg(target_family = "wasm")]
    {
        // Enable stack traces for panics in web builds.
        console_error_panic_hook::set_once();
    }

    // Create the application.
    let mut app = App::new();

    // Add default plugins.
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Mythmallow".to_owned(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .build(),
    );

    // Add diagnostics plugins.
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(EntityCountDiagnosticsPlugin);

    // Enable the editor in development mode.
    #[cfg(feature = "development")]
    {
        // Add editor plugin.
        app.add_plugins(EditorPlugin::default());

        // Overwrite editor controls.
        let mut editor_controls = EditorControls::default_bindings();
        editor_controls.unbind(EditorAction::PlayPauseEditor);
        editor_controls.insert(
            EditorAction::PlayPauseEditor,
            EditorBinding {
                input: EditorUserInput::Chord(vec![
                    EditorButton::Keyboard(KeyCode::ControlLeft),
                    EditorButton::Keyboard(KeyCode::KeyE),
                ]),
                conditions: vec![EditorBindingCondition::ListeningForText(false)],
            },
        );
        app.insert_resource(editor_controls);
    }

    // Add core plugins.
    core(&mut app);

    // Make the primary window persistent in native builds.
    #[cfg(not(target_family = "wasm"))]
    {
        // Use core resources.
        use mythmallow::core::resources::all::*;

        // Find the primary window entity.
        let primary_window_entity = app
            .world_mut()
            .query_filtered::<Entity, With<PrimaryWindow>>()
            .get_single(app.world())
            .expect("fatal: unable to find the primary window entity");

        // Determine the persistent primary window state path.
        let arguments = app.world().resource::<Arguments>();
        let state_path = arguments.data_directory.join("state").join("window.toml");

        // Make the primary window persistent.
        app.world_mut().entity_mut(primary_window_entity).insert((
            Name::new("Primary Window"),
            PersistentWindowBundle {
                window: Window { title: "Mythmallow".to_owned(), ..Default::default() },
                state: Persistent::<WindowState>::builder()
                    .name("primary window state")
                    .format(StorageFormat::Toml)
                    .path(state_path)
                    .default(WindowState::borderless_fullscreen())
                    .revertible(true)
                    .revert_to_default_on_deserialization_errors(true)
                    .build()
                    .unwrap_or_else(|_| {
                        panic!("fatal: unable to initialize persistent primary window state")
                    }),
            },
        ));

        // Add persistent windows plugin.
        app.add_plugins(PersistentWindowsPlugin);
    }

    // Add content plugins.
    content(&mut app);

    // Log registry statistics.
    {
        let game_mode_registry = app.world().resource::<GameModeRegistry>();
        let number_of_game_modes = game_mode_registry.number_of_game_modes();
        log::info!(
            "{} game mode{} {} registered",
            number_of_game_modes,
            if number_of_game_modes == 1 { "" } else { "s" },
            if number_of_game_modes == 1 { "is" } else { "are" },
        );
    }
    {
        let player_registry = app.world_mut().resource::<PlayerRegistry>();
        let number_of_mythologies = player_registry.number_of_mythologies();
        let number_of_players = player_registry.number_of_players();
        log::info!(
            "{} player{} {} registered across {} mytholog{}",
            number_of_players,
            if number_of_players == 1 { "" } else { "s" },
            if number_of_players == 1 { "is" } else { "are" },
            number_of_mythologies,
            if number_of_mythologies == 1 { "y" } else { "ies" },
        );
    }
    {
        let enemy_registry = app.world().resource::<EnemyRegistry>();
        let number_of_enemy_packs = enemy_registry.number_of_packs();
        let number_of_enemies = enemy_registry.number_of_enemies();
        log::info!(
            "{} enem{} {} registered across {} enemy pack{}",
            number_of_enemies,
            if number_of_enemies == 1 { "y" } else { "ies" },
            if number_of_enemies == 1 { "is" } else { "are" },
            number_of_enemy_packs,
            if number_of_enemy_packs == 1 { "" } else { "s" },
        );
    }
    {
        let item_registry = app.world().resource::<ItemRegistry>();
        let number_of_items = item_registry.number_of_items();
        log::info!(
            "{} item{} {} registered",
            number_of_items,
            if number_of_items == 1 { "" } else { "s" },
            if number_of_items == 1 { "is" } else { "are" },
        );
    }

    // Start the application.
    log::info!("starting the application");
    app.run()
}

fn core(app: &mut App) {
    app.add_plugins(ConfigurationPlugin);
    app.add_plugins(LocalizationPlugin);
    app.add_plugins(UtilityPlugin);
    app.add_plugins(ConsolePlugin);
    app.add_plugins(StatePlugin);
    app.add_plugins(SetPlugin);
    app.add_plugins(CameraPlugin);
    app.add_plugins(ActionPlugin);
    app.add_plugins(PropertyPlugin);
    app.add_plugins(LevelingPlugin);
    app.add_plugins(PhysicsPlugin);
    app.add_plugins(MapPlugin);
    app.add_plugins(MovementPlugin);
    app.add_plugins(CombatPlugin);
    app.add_plugins(ModePlugin);
    app.add_plugins(ItemPlugin);
    app.add_plugins(InventoryPlugin);
    app.add_plugins(PlayerPlugin);
    app.add_plugins(EnemyPlugin);
    app.add_plugins(UiPlugin);
}

fn content(app: &mut App) {
    app.add_plugins(SurvivalModePlugin);
    app.add_plugins(GreekItemsPlugin);
    app.add_plugins(GreekPlayersPlugin);
    app.add_plugins(SweetEnemiesPlugin);
}
