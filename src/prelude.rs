//! The prelude.

/// Use wrapping.
pub use color_eyre::eyre::WrapErr;

/// The Result type.
pub type Result<T = (), E = color_eyre::Report> = color_eyre::Result<T, E>;

// Return early.
pub use color_eyre::eyre::bail;

// Personal preference
pub use std::format as f;