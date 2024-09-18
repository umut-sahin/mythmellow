//! Localization assets.

use crate::prelude::*;


/// Asset for the supported locales.
#[derive(Asset, Clone, Debug, Deref, Deserialize, Resource, Serialize, TypePath)]
pub struct SupportedLocales {
    /// List of supported locales.
    pub locales: Vec<LanguageIdentifier>,
}

impl SupportedLocales {
    /// List of the default supported locale identifiers.
    pub const DEFAULT: &'static [&'static str] = &["en-US", "tr"];
}

impl Default for SupportedLocales {
    fn default() -> SupportedLocales {
        SupportedLocales {
            locales: SupportedLocales::DEFAULT
                .iter()
                .map(|language| language.parse().expect("invalid language identifier constant"))
                .collect(),
        }
    }
}

/// Asset loader of the supported locales asset.
#[derive(Default)]
pub struct SupportedLocalesLoader;

impl AssetLoader for SupportedLocalesLoader {
    type Asset = SupportedLocales;
    type Settings = ();
    type Error = SupportedLocalesLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<SupportedLocales, SupportedLocalesLoaderError> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).await?;
        let asset = toml::from_str(&buffer)?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}

/// Errors of loading supported locales asset.
#[derive(Debug, Error)]
pub enum SupportedLocalesLoaderError {
    /// Unable to read the asset.
    #[error("unable to read the asset ({0})")]
    ReadingFailed(#[from] std::io::Error),

    /// Asset format was not correct.
    #[error("asset format was not correct ({0})")]
    ParsingFailed(#[from] toml::de::Error),
}

/// Handle to the supported locales asset.
#[derive(Debug, Reflect, Resource)]
pub struct SupportedLocalesHandle(pub Handle<SupportedLocales>);
