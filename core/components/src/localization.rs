//! Localization components

use mythmallow_core_dependencies::*;


/// Component for the localized texts.
#[derive(Clone, Component, Debug, Reflect)]
pub enum LocalizedText {
    /// Localized text (e.g., `Quit` on locale `en-US`, `Kapat` on locale `tr`).
    Localized {
        /// Key of the localized text.
        key: &'static str,

        /// Arguments of the localized text.
        args: SmallVec<[(&'static str, SmolStr); 1]>,

        /// Fallback of the localized text.
        fallback: Cow<'static, str>,
    },
    /// Constant text (e.g., `<`, `+`).
    Constant {
        /// Value of the constant text.
        text: Cow<'static, str>,
    },
}

impl LocalizedText {
    /// Gets the localized text.
    pub fn get(&self, localization: &Localization) -> Cow<'static, str> {
        match self {
            LocalizedText::Localized { key, args, fallback } => {
                let mut fluent_args = FluentArgs::new();
                for (key, value) in args {
                    fluent_args.set(*key, value.as_str());
                }

                let request = Request::new(key).args(&fluent_args);
                localization.content(request).map(Cow::Owned).unwrap_or_else(|| fallback.clone())
            },
            LocalizedText::Constant { text } => text.clone(),
        }
    }

    /// Gets the fallback of the localized text.
    pub fn fallback(&self) -> Cow<'static, str> {
        match self {
            LocalizedText::Localized { fallback, .. } => fallback.clone(),
            LocalizedText::Constant { text } => text.clone(),
        }
    }
}

impl<T: ToString> From<T> for LocalizedText {
    fn from(value: T) -> LocalizedText {
        LocalizedText::Constant { text: Cow::Owned(value.to_string()) }
    }
}
