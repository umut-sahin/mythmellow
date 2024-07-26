//! Actions of the game.

mod game;
mod global;
mod menu;

pub use {
    game::GameAction,
    global::GlobalAction,
    menu::MenuAction,
};
