#![doc = include_str!("../README.md")]

/// Core of the game.
pub mod core {
    #[doc(inline)]
    pub use {
        mythmallow_core_data as data,
        mythmallow_core_dependencies as dependencies,
        mythmallow_core_logic as logic,
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
