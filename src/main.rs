use crate::cli::{Args, Commands};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};
use color_eyre::Result;
use orion_lib::run_contents;
use std::fs::read_to_string;
use std::io;
use std::process::exit;

mod cli;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Run { file } => {
            let contents = read_to_string(file)?;
            run_contents(contents)?;
        }
        Commands::Complete { shell } => match shell.to_lowercase().as_str() {
            "bash" => generate(Bash, &mut Args::command(), "orion", &mut io::stdout()),
            "zsh" => generate(Zsh, &mut Args::command(), "orion", &mut io::stdout()),
            "fish" => generate(Fish, &mut Args::command(), "orion", &mut io::stdout()),
            "powershell" => generate(PowerShell, &mut Args::command(), "orion", &mut io::stdout()),
            "elvish" => generate(Elvish, &mut Args::command(), "orion", &mut io::stdout()),
            shell => {
                eprintln!("Error: Invalid shell `{}`!", shell);
                exit(1);
            }
        },
    }

    Ok(())
}
