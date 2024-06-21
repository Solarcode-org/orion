//! The CLI.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// The Orion CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Run an Orion File.
    Ior {
        /// The file to run.
        file: String,

        /// Whether to use braces or not
        #[arg(long)]
        braces: bool
    },
    /// Generate completions for a shell
    Complete {
        /// The shell to generate completions for.
        shell: String,
    },
    /// Generate a new Orion project.
    New {
        /// Name of the project.
        name: String,

        /// Where to generate the project.
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Run an Orion project.
    Run {
        /// Whether to use braces or not.
        #[arg(long)]
        braces: bool,

        /// Path to project.
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Path to config file.
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Compile to yaml format.
        #[arg(long)]
        yaml: bool
    },

    /// Compile an Orion project.
    Build {
        /// Whether to use braces or not.
        #[arg(long)]
        braces: bool,

        /// Path to project.
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Path to config file.
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Compile to yaml format.
        #[arg(long)]
        yaml: bool
    },

    /// Run an Orion JIT File.
    Jit {
        /// Path to JIT file.
        jit: PathBuf,

        /// Use YAML format.
        #[arg(long)]
        yaml: bool
    }
}
