//! Localization resources.

use crate::prelude::*;


/// Resource for the default locale of the application in the current platform.
#[derive(Clone, Deref, Resource)]
pub struct DefaultLocale(pub LanguageIdentifier);

impl DefaultLocale {
    /// Gets the identifier of the default locale.
    pub fn identifier(&self) -> &LanguageIdentifier {
        &self.0
    }
}

impl Default for DefaultLocale {
    fn default() -> DefaultLocale {
        DefaultLocale(
            sys_locale::get_locale()
                .unwrap_or_else(|| "en-US".to_owned())
                .parse::<LanguageIdentifier>()
                .unwrap_or(
                    "en-US"
                        .parse::<LanguageIdentifier>()
                        .expect("invalid language identifier constant"),
                ),
        )
    }
}


/// Resource for the locations of locale assets.
#[derive(Debug, Default, Deref, DerefMut, Resource, Reflect)]
pub struct LocaleAssets(pub Vec<&'static str>);


/// Resource for the handles to locale assets.
#[derive(Clone, Default, Deref, DerefMut, Reflect, Resource)]
pub struct LocaleResourceHandles(pub Vec<Handle<ResourceAsset>>);


/// Resource for the handle to locales folder.
#[derive(Clone, Default, Deref, Reflect, Resource)]
pub struct LocalesFolderHandle(pub Handle<LoadedFolder>);
