use serde::{Serialize, Deserialize};

/// The orion configuration.
#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    /// Use braces?
    pub braces: bool,
}