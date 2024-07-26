use {
    mythmallow_core_assets::all::*,
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::localization::*,
};

/// Plugin for managing the localization of the application.
pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<LocaleAssets>();
        app.register_type::<LocaleResourceHandles>();
        app.register_type::<LocalesFolderHandle>();
        app.register_type::<SupportedLocalesHandle>();

        // Register components.
        app.register_type::<LocalizedText>();

        // Get general settings.
        let general_settings = app.world_mut().resource::<Persistent<GeneralSettings>>();

        // Create locale resources.
        let locale_assets = LocaleAssets::default();
        let default_locale = DefaultLocale::default();
        let current_locale = match general_settings.desired_locale() {
            Some(locale) => Locale::new(locale),
            None => {
                log::error!(
                    "requested locale {:?} is not valid, defaulting to {:?}",
                    general_settings.locale,
                    default_locale.identifier().to_string(),
                );
                Locale::new(default_locale.identifier().clone())
            },
        };

        // Insert locale resources.
        app.insert_resource(locale_assets);
        app.insert_resource(default_locale);
        app.insert_resource(current_locale);

        // Initialize assets.
        app.init_asset::<SupportedLocales>();

        // Initialize asset loaders.
        app.init_asset_loader::<SupportedLocalesLoader>();

        // Add systems
        app.add_systems(Startup, load_supported_locales);
        app.add_systems(OnEnter(LocalizationState::Loading), load_locale_assets);
        app.add_systems(
            Update,
            create_locales_folder.run_if(in_state(LocalizationState::Loading)).run_if(
                |asset_server: Res<AssetServer>,
                 locale_assets: Option<Res<LocaleResourceHandles>>| {
                    if let Some(locale_assets) = locale_assets {
                        locale_assets.iter().all(|handle| {
                            matches!(
                                asset_server.get_load_state(handle),
                                None | Some(LoadState::Loaded | LoadState::Failed(_)),
                            )
                        })
                    } else {
                        false
                    }
                },
            ),
        );
        app.add_systems(
            Update,
            setup_localization
                .run_if(in_state(LocalizationState::Loading))
                .run_if(resource_exists::<LocalesFolderHandle>),
        );
        app.add_systems(
            OnEnter(LocalizationState::Ready),
            start_application.run_if(in_state(AppState::LoadingLocalization)).run_if(
                |asset_server: Res<AssetServer>,
                 supported_locales_handle: Res<SupportedLocalesHandle>| {
                    match asset_server.get_load_state(&supported_locales_handle.0) {
                        None => false,
                        Some(state) => {
                            match state {
                                LoadState::NotLoaded => false,
                                LoadState::Loading => false,
                                LoadState::Loaded => true,
                                LoadState::Failed(_) => true,
                            }
                        },
                    }
                },
            ),
        );
        app.add_systems(OnEnter(LocalizationState::Ready), update_all_localized_texts);
        app.add_systems(
            Last,
            update_changed_localized_texts.run_if(in_state(LocalizationState::Ready)),
        );

        // Add fluent plugin.
        app.add_plugins(FluentPlugin);
    }
}
