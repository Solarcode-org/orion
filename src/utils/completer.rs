use std::io;
use clap::Command;
use clap_complete::generate;
use clap_complete::Shell::{Bash, Zsh, Fish, PowerShell, Elvish};
use crate::prelude::*;

pub fn complete(shell: String, command: &mut Command) -> Result<()> {
    match shell.to_lowercase().as_str() {
        "bash" => generate(Bash, command, "orion", &mut io::stdout()),
        "zsh" => generate(Zsh, command, "orion", &mut io::stdout()),
        "fish" => generate(Fish, command, "orion", &mut io::stdout()),
        "powershell" => generate(PowerShell, command, "orion", &mut io::stdout()),
        "elvish" => generate(Elvish, command, "orion", &mut io::stdout()),
        shell => bail!("Error: Invalid shell `{}`!", shell),
    };

    Ok(())
}
