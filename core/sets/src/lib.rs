//! System sets of the game.

mod gameplay;
mod initialization;
mod loading;
mod restart;

pub use {
    gameplay::GameplaySystems,
    initialization::InitializationSystems,
    loading::LoadingSystems,
    restart::RestartSystems,
};
