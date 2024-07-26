//! Configuration constants.

use mythmallow_core_dependencies::*;


/// Storage format of the configuration files.
pub const CONFIGURATION_STORAGE_FORMAT: StorageFormat = {
    #[cfg(not(target_family = "wasm"))]
    {
        StorageFormat::Toml
    }
    #[cfg(target_family = "wasm")]
    {
        StorageFormat::Json
    }
};
