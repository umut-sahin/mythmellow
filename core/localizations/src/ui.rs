//! User interface localizations.

use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
};


/// Gets the localized text of the play buttons.
pub fn play_button() -> LocalizedText {
    LocalizedText::Localized { key: "ui-play-button", args: smallvec![], fallback: "Play".into() }
}

/// Gets the localized text of the settings buttons.
pub fn settings_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-settings-button",
        args: smallvec![],
        fallback: "Settings".into(),
    }
}

/// Gets the localized text of the quit buttons.
pub fn quit_button() -> LocalizedText {
    LocalizedText::Localized { key: "ui-quit-button", args: smallvec![], fallback: "Quit".into() }
}


/// Gets the localized text of the language setting names.
pub fn language_setting_name() -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-language-setting-name",
        args: smallvec![],
        fallback: "Language:".into(),
    }
}

/// Gets the localized text of the language setting values.
pub fn language_setting_value() -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-language-setting-value",
        args: smallvec![],
        fallback: "???".into(),
    }
}


/// Gets the localized text of the back buttons.
pub fn back_button() -> LocalizedText {
    LocalizedText::Localized { key: "ui-back-button", args: smallvec![], fallback: "Back".into() }
}


/// Gets the localized text of the no modes title.
pub fn no_modes_menu_title() -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-menu-title-no-modes",
        args: smallvec![],
        fallback: "No game modes are available to be played!".into(),
    }
}

/// Gets the localized text of the no players title.
pub fn no_players_menu_title() -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-menu-title-no-players",
        args: smallvec![],
        fallback: "No players are available to be played!".into(),
    }
}

/// Gets the localized text of the no enemies title.
pub fn no_enemies_menu_title() -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-menu-title-no-enemies",
        args: smallvec![],
        fallback: "No enemies are available to be played!".into(),
    }
}


/// Gets the localized text of the resume buttons.
pub fn resume_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-resume-button",
        args: smallvec![],
        fallback: "Resume".into(),
    }
}

/// Gets the localized text of the restart buttons.
pub fn restart_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-restart-button",
        args: smallvec![],
        fallback: "Restart".into(),
    }
}

/// Gets the localized text of the return to main menu buttons.
pub fn return_to_main_menu_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-return-to-main-menu-button",
        args: smallvec![],
        fallback: "Return to main menu".into(),
    }
}

/// Gets the localized text of the quit to desktop buttons.
pub fn quit_to_desktop_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-quit-to-desktop-button",
        args: smallvec![],
        fallback: "Quit to desktop".into(),
    }
}


/// Gets the localized text of the experience bars.
pub fn experience_bar(level: &Level) -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-experience-bar",
        args: smallvec![("level", format_smolstr!("{}", level.0))],
        fallback: format!("Level {}", level.0).into(),
    }
}


/// Gets the localized text of the won title.
pub fn won_menu_title() -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-menu-title-won",
        args: smallvec![],
        fallback: "You won!".into(),
    }
}

/// Gets the localized text of the lost title.
pub fn lost_menu_title() -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-menu-title-lost",
        args: smallvec![],
        fallback: "You lost!".into(),
    }
}

/// Gets the localized text of the play again buttons.
pub fn play_again_button() -> LocalizedText {
    LocalizedText::Localized {
        key: "ui-play-again-button",
        args: smallvec![],
        fallback: "Play again".into(),
    }
}

/// Gets the localized text of the retry buttons.
pub fn retry_button() -> LocalizedText {
    LocalizedText::Localized { key: "ui-retry-button", args: smallvec![], fallback: "Retry".into() }
}
