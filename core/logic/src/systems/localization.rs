//! Localization systems.

use crate::{
    prelude::*,
    systems::utility::*,
};


/// Loads supported locales asset.
pub fn load_supported_locales(mut commands: Commands, asset_server: Res<AssetServer>) {
    log::info!("loading supported locales");
    commands.insert_resource(SupportedLocalesHandle(asset_server.load("locales/supported.toml")));
}


/// Loads locale assets.
pub fn load_locale_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    locale: Res<Locale>,
    locale_assets: Res<LocaleAssets>,
) {
    let mut locale_resource_handles = Vec::with_capacity(locale_assets.len());

    log::info!("preferred locale is \"{}\"", locale.requested);
    for asset in locale_assets.iter() {
        let path = format!("locales/{}/{}", locale.requested, asset);
        log::info!("loading {}", path);
        locale_resource_handles.push(asset_server.load::<ResourceAsset>(path));
    }

    commands.insert_resource(LocaleResourceHandles(locale_resource_handles));
}

/// Creates locales folder.
pub fn create_locales_folder(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    locale: Res<Locale>,
    mut locale_resource_handles: ResMut<LocaleResourceHandles>,
    resource_assets: ResMut<Assets<ResourceAsset>>,
    mut bundle_assets: ResMut<Assets<BundleAsset>>,
    mut loaded_folders: ResMut<Assets<LoadedFolder>>,
) {
    let mut fluent_bundle = FluentBundle::new_concurrent(vec![locale.requested.clone()]);
    fluent_bundle.set_use_isolating(false);

    let mut handles = Vec::with_capacity(1 + locale_resource_handles.len());
    for resource in std::mem::take(locale_resource_handles.deref_mut()).0 {
        if asset_server.get_load_state(&resource) == Some(LoadState::Loaded) {
            handles.push(resource.clone().untyped());

            if let Some(resource_asset) = resource_assets.get(&resource) {
                if let Err(errors) = fluent_bundle.add_resource(resource_asset.0.clone()) {
                    for error in errors {
                        if let Some(path) = asset_server.get_path(&resource) {
                            log::warn!("in {}: {}", path, error);
                        } else {
                            log::warn!("in locales/{}/???: {}", locale.requested, error);
                        }
                    }
                }
            }
        }
    }

    let bundle_asset = BundleAsset(Arc::new(fluent_bundle));
    handles.push(bundle_assets.add(bundle_asset).untyped());

    commands.remove_resource::<LocaleResourceHandles>();
    commands.insert_resource(LocalesFolderHandle(loaded_folders.add(LoadedFolder { handles })));
}

/// Sets up localization.
pub fn setup_localization(
    mut commands: Commands,
    localization_builder: LocalizationBuilder,
    locales_folder_handle: Res<LocalesFolderHandle>,
    mut next_localization_state: ResMut<NextState<LocalizationState>>,
) {
    log::info!("loaded the preferred locale");

    let localization = localization_builder.build(locales_folder_handle.deref());
    commands.insert_resource(localization);
    next_localization_state.set(LocalizationState::Ready);

    commands.remove_resource::<LocalesFolderHandle>();
}

/// Starts the application.
pub fn start_application(
    mut commands: Commands,
    arguments: Res<Arguments>,
    supported_locales_handle: Res<SupportedLocalesHandle>,
    supported_locales_assets: Res<Assets<SupportedLocales>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    registered_systems: Res<RegisteredSystems>,
) {
    let supported_locales = match supported_locales_assets.get(&supported_locales_handle.0) {
        Some(supported_locales) => {
            if !supported_locales.is_empty() {
                log::info!("loaded supported locales");
                supported_locales.clone()
            } else {
                log::warn!(
                    "defaulting supported locales to {:?} as supported locales are empty",
                    SupportedLocales::DEFAULT,
                );
                SupportedLocales::default()
            }
        },
        _ => {
            log::warn!(
                "defaulting supported locales to {:?} as supported locales failed to load",
                SupportedLocales::DEFAULT
            );
            SupportedLocales::default()
        },
    };
    commands.insert_resource(supported_locales.clone());

    if arguments.start_in_game {
        commands.run_system(registered_systems.configuration.start_in_game);
        return;
    }

    // Transition to the main menu.
    log::info!("transitioning to the main menu");
    next_app_state.set(AppState::MainMenu)
}


/// Sets the locale of the application.
pub fn set_locale(
    In(new_locale): In<LanguageIdentifier>,
    mut general_settings: ResMut<Persistent<GeneralSettings>>,
    mut locale: ResMut<Locale>,
    mut next_localization_state: ResMut<NextState<LocalizationState>>,
) {
    if locale.requested == new_locale {
        return;
    }

    let new_locale_string = new_locale.to_string();
    log::info!("setting the locale to {:?}", new_locale_string);

    general_settings.locale = new_locale_string;
    general_settings.persist().ok();

    locale.requested = new_locale;
    next_localization_state.set(LocalizationState::Loading);
}


/// Updates all localized texts.
pub fn update_all_localized_texts(
    mut localized_text_query: Query<(&mut Text, &LocalizedText)>,
    localization: Res<Localization>,
) {
    for (mut text, localized_text) in localized_text_query.iter_mut() {
        text.sections[0].value = localized_text.get(&localization).to_string();
    }
}

/// Updates changed localized texts.
pub fn update_changed_localized_texts(
    mut localized_text_query: Query<(&mut Text, &LocalizedText), Changed<LocalizedText>>,
    localization: Res<Localization>,
) {
    for (mut text, localized_text) in localized_text_query.iter_mut() {
        text.sections[0].value = localized_text.get(&localization).to_string();
    }
}
