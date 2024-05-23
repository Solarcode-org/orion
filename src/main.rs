use std::fs::read_to_string;

use clap::{CommandFactory, Parser};
use orion_lib::run_contents;

use prelude::*;

use utils::cli::{Args, Commands};
use crate::utils::completer::complete;

mod prelude;
mod utils;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Run { file } => {
            let contents = read_to_string(&file).with_context(|| f!("Could not read file `{file}`"))?;
            run_contents(contents)?;
        }
        Commands::Complete { shell } => complete(shell, &mut Args::command())?,
    }

    Ok(())
}
