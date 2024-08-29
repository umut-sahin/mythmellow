//! Utility systems.

use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
};


/// Resource for the registered systems.
#[derive(Debug, Resource)]
pub struct RegisteredSystems {
    /// Configuration systems.
    pub configuration: RegisteredConfigurationSystems,

    /// Leveling systems.
    pub leveling: RegisteredLevelingSystems,

    /// Localization systems.
    pub localization: RegisteredLocalizationSystems,

    /// Widget systems.
    pub widget: RegisteredWidgetSystems,
}

impl RegisteredSystems {
    /// Initializes the registered systems resource from its definition.
    pub fn initialize(app: &mut App) {
        let systems = app.world_mut().spawn(Name::new("Registered Systems")).id();

        let configuration = RegisteredConfigurationSystems::register(app, systems);
        let leveling = RegisteredLevelingSystems::register(app, systems);
        let localization = RegisteredLocalizationSystems::register(app, systems);
        let widget = RegisteredWidgetSystems::register(app, systems);

        app.insert_resource(RegisteredSystems { configuration, leveling, localization, widget });
    }
}

impl RegisteredSystems {
    fn attach<I>(
        app: &mut App,
        systems: Entity,
        system: SystemId<I>,
        name: impl Into<Cow<'static, str>>,
    ) {
        let system = system.entity();
        if let Some(mut systems) = app.world_mut().get_entity_mut(systems) {
            systems.add_child(system);
            if let Some(mut system) = app.world_mut().get_entity_mut(system) {
                system.insert(Name::new(name));
            }
        }
    }
}


/// Container for the registered configuration systems.
#[derive(Debug)]
pub struct RegisteredConfigurationSystems {
    /// Registered [start_in_game](crate::configuration::start_in_game) system.
    pub start_in_game: SystemId<()>,
}

impl RegisteredConfigurationSystems {
    /// Registers appropriate localization systems.
    pub fn register(app: &mut App, registered_systems: Entity) -> RegisteredConfigurationSystems {
        use crate::configuration::*;

        let start_in_game = app.world_mut().register_system(start_in_game);
        RegisteredSystems::attach(app, registered_systems, start_in_game, "start_in_game");

        RegisteredConfigurationSystems { start_in_game }
    }
}


/// Container for the registered localization systems.
#[derive(Debug)]
pub struct RegisteredLocalizationSystems {
    /// Registered [set_locale](crate::localization::set_locale) system.
    pub set_locale: SystemId<LanguageIdentifier>,
}

impl RegisteredLocalizationSystems {
    /// Registers appropriate localization systems.
    pub fn register(app: &mut App, registered_systems: Entity) -> RegisteredLocalizationSystems {
        use crate::localization::*;

        let set_locale = app.world_mut().register_system(set_locale);
        RegisteredSystems::attach(app, registered_systems, set_locale, "set_locale");

        RegisteredLocalizationSystems { set_locale }
    }
}


/// Container for the registered widget systems.
#[derive(Debug)]
pub struct RegisteredWidgetSystems {
    /// Registered [save_selected_widget](crate::ui::widget::save_selected_widget) system.
    pub save_selected_widget: SystemId,

    /// Registered [restore_selected_widget](crate::ui::widget::restore_selected_widget) system.
    pub restore_selected_widget: SystemId,
}

impl RegisteredWidgetSystems {
    /// Registers appropriate widget systems.
    pub fn register(app: &mut App, registered_systems: Entity) -> RegisteredWidgetSystems {
        use crate::ui::widget::*;

        let save_selected_widget = app.world_mut().register_system(save_selected_widget);
        RegisteredSystems::attach(
            app,
            registered_systems,
            save_selected_widget,
            "save_selected_widget",
        );

        let restore_selected_widget = app.world_mut().register_system(restore_selected_widget);
        RegisteredSystems::attach(
            app,
            registered_systems,
            restore_selected_widget,
            "restore_selected_widget",
        );

        RegisteredWidgetSystems { save_selected_widget, restore_selected_widget }
    }
}


/// Container for the registered leveling systems.
#[derive(Debug)]
pub struct RegisteredLevelingSystems {
    /// Registered [set_level](crate::leveling::set_level) system.
    pub set_level: SystemId<Level>,
}

impl RegisteredLevelingSystems {
    /// Registers appropriate leveling systems.
    pub fn register(app: &mut App, registered_systems: Entity) -> RegisteredLevelingSystems {
        use crate::leveling::*;

        let set_level = app.world_mut().register_system(set_level);
        RegisteredSystems::attach(app, registered_systems, set_level, "set_level");

        RegisteredLevelingSystems { set_level }
    }
}


/// Run condition for the console being closed.
pub fn console_is_not_open(console_state: Res<ConsoleState>) -> bool {
    !console_state.open
}

/// Run condition for god mode being enabled.
pub fn god_mode_is_enabled(god_mode: Res<GodMode>) -> bool {
    god_mode.is_enabled
}

/// Run condition for god mode being disabled.
pub fn god_mode_is_disabled(god_mode: Res<GodMode>) -> bool {
    !god_mode.is_enabled
}


/// System to remove a resource.
pub fn remove_resource<R: Resource>(mut commands: Commands) {
    commands.remove_resource::<R>();
}

/// System to reset a resource.
pub fn reset_resource<R: Resource + Default>(mut commands: Commands) {
    commands.init_resource::<R>();
}


/// Reduces, and eventually clears, the cooldowns.
pub fn cooldown<T: Send + Sync + 'static>(
    mut commands: Commands,
    time: Res<Time>,
    mut cooldown_query: Query<(Entity, &mut Cooldown<T>)>,
) {
    for (entity, mut cooldown) in cooldown_query.iter_mut() {
        cooldown.timer.tick(time.delta());
        if cooldown.timer.finished() {
            commands.entity(entity).remove::<Cooldown<T>>();
        }
    }
}


/// Finds the first obstacle from a position along a direction within a distance units.
pub fn find_obstacle(
    spatial_query: &SpatialQuery,
    position: &Position,
    direction: &Vec2,
    distance: f32,
) -> Option<RayHitData> {
    spatial_query.cast_ray(
        position.xy(),
        Dir2::new(*direction).ok()?,
        distance,
        false,
        SpatialQueryFilter::from_mask([Layer::MapObstacle]),
    )
}

/// Finds the enemies in range from a position ordered by their distance.
pub fn find_enemies_in_range_sorted_by_distance(
    spatial_query: &SpatialQuery,
    position: &Position,
    area: &Collider,
    enemy_hit_box_query: &Query<&Position, With<EnemyHitBox>>,
) -> Vec<(Entity, Position, f32)> {
    let intersections = spatial_query.shape_intersections(
        area,
        position.xy(),
        0.00,
        SpatialQueryFilter::from_mask([Layer::EnemyHitBox]),
    );

    let mut enemies_in_range = intersections
        .iter()
        .filter_map(|&enemy_hit_box_entity| {
            enemy_hit_box_query
                .get(enemy_hit_box_entity)
                .map(|&enemy_hit_box_position| {
                    (
                        enemy_hit_box_entity,
                        enemy_hit_box_position,
                        enemy_hit_box_position.distance(position.xy()),
                    )
                })
                .ok()
        })
        .collect::<Vec<_>>();

    enemies_in_range.sort_by(|(_, _, distance1), (_, _, distance2)| {
        distance1.partial_cmp(distance2).unwrap_or(Ordering::Equal)
    });

    enemies_in_range
}


/// Quits the application.
#[cfg(not(target_family = "wasm"))]
pub fn quit(app_exit_event_writer: &mut EventWriter<AppExit>) {
    app_exit_event_writer.send(AppExit::Success);
}

/// Quits the application.
#[cfg(target_family = "wasm")]
pub fn quit(_app_exit_event_writer: &mut EventWriter<AppExit>) {
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
}
