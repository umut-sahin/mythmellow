#![doc = include_str!("../README.md")]

/// Core of the game.
pub mod core {
    #[doc(inline)]
    pub use {
        mythmallow_core_actions as actions,
        mythmallow_core_assets as assets,
        mythmallow_core_bundles as bundles,
        mythmallow_core_commands as commands,
        mythmallow_core_components as components,
        mythmallow_core_constants as constants,
        mythmallow_core_dependencies as dependencies,
        mythmallow_core_events as events,
        mythmallow_core_interfaces as interfaces,
        mythmallow_core_localizations as localizations,
        mythmallow_core_plugins as plugins,
        mythmallow_core_registries as registries,
        mythmallow_core_resources as resources,
        mythmallow_core_sets as sets,
        mythmallow_core_states as states,
        mythmallow_core_systems as systems,
    };
}

/// Content of the game.
pub mod content {
    /// Enemies of the game.
    pub mod enemies {
        #[doc(inline)]
        pub use mythmallow_enemies_sweet as sweet;
    }

    /// Items of the game.
    pub mod items {
        #[doc(inline)]
        pub use mythmallow_items_greek as greek;
    }

    /// Modes of the game.
    pub mod modes {
        #[doc(inline)]
        pub use mythmallow_mode_survival as survival;
    }

    /// Perks of the game.
    pub mod perks {}

    /// Players of the game.
    pub mod players {
        #[doc(inline)]
        pub use mythmallow_players_greek as greek;
    }
}
