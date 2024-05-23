//! The prelude.

/// Use wrapping.
pub use color_eyre::eyre::WrapErr;

/// The Result type.
pub type Result<T> = color_eyre::Result<T>;

/// Return early.
pub use color_eyre::eyre::bail;

// Personal preference
pub use std::format as f;