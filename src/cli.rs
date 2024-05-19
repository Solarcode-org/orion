use clap::{Parser, Subcommand};

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
    Run {
        #[arg(help = "The file to run.")]
        file: String,
    },
    /// Generate completions for a shell
    Complete {
        #[arg(help = "The shell to generate completions for.")]
        shell: String,
    },
}
