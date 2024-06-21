use std::fs::{create_dir, create_dir_all, read_to_string, write};
use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use orion_lib::{run_contents, encode, decode, run_ast, setup_error_hooks};

use prelude::*;

use utils::cli::{Args, Commands};
use crate::utils::completer::complete;
use crate::utils::config::Config;

mod prelude;
mod utils;

fn main() -> Result {
    setup_error_hooks()?;

    let args = Args::parse();

    match args.command {
        Commands::Ior { file, braces } => {
            let contents = read_to_string(&file).with_context(|| f!("Could not read file `{file}`"))?;
            run_contents(contents, braces).with_context(|| "Could not run contents of file.")?;
        }
        Commands::Complete { shell } => complete(shell, &mut Args::command()).with_context(|| "Could not generate completions")?,
        Commands::New { name, path } => {
            let config = Config::default();

            if let Some(path) = path {
                let dir = path.join(name);

                create_dir_all(&dir).with_context(|| "Could not create project")?;
                write(dir.clone().join("Solar.yml"), serde_yaml::to_string(&config)?).with_context(|| "Could not write config.")?;

                let src = dir.join("src");
                create_dir(&src).with_context(|| "Could not create source directory.")?;

                write(src.join("main.or"), "$say(\"Hello, world!\");").with_context(|| "Could not create main file.")?;
            } else {
                create_dir(&name)?;

                write(PathBuf::new().join(&name).join("Solar.yml"), serde_yaml::to_string(&config)?)?;

                let src = PathBuf::new().join(&name).join("src");
                create_dir(&src)?;

                let build = PathBuf::new().join(&name).join("_build");
                create_dir(build)?;

                write(src.join("main.or"), "$say(\"Hello, world!\");")?;
            }
        }
        Commands::Run { braces, path, config , yaml} => {
            let config = config.unwrap_or(PathBuf::new().join("./Solar.yml"));

            let config: Config = serde_yaml::from_str(&read_to_string(config).with_context(|| "Could not read config.")?)?;

            let braces = if braces {
                true
            } else {
                config.braces
            };

            let (main, mainjit, mainhash) = if let Some(path) = path {
                let src = path.join("src");
                let build = path.join("_build");
                (src.join("main.or"), build.join("main.jit"), build.join("main.hash"))
            } else {
                let src = PathBuf::new().join("src");
                let build = PathBuf::new().join("_build");

                (src.join("main.or"), build.join("main.jit"), build.join("main.hash"))
            };

            let contents = read_to_string(main).with_context(|| "Could not read main file `main.rs`")?;

            let recompile = if mainhash.exists() {
                let hash = read_to_string(&mainhash).with_context(|| "Could not read hash file `main.hash`")?;

                if hash.replace(|c: char| c.to_string().trim().is_empty(), "")
                    != contents.replace(|c: char| c.to_string().trim().is_empty(), "") {
                    write(mainhash, &contents).with_context(|| "Could not write hash `main.hash`.")?;

                    true
                } else {
                    false
                }
            } else {
                println!("{}", mainhash.display());
                write(mainhash, &contents).with_context(|| "Could not write hash `main.hash`.")?;

                true
            };

            if recompile {
                write(&mainjit, encode(&contents, braces, yaml)?).with_context(|| "Could not write JIT file.")?;
            }

            let jit = read_to_string(mainjit).with_context(|| "Could not read JIT file.")?;
            let ast = decode(jit, yaml)?;

            run_ast(ast).with_context(|| "Could not run project.")?;
        }
        Commands::Build { braces, path, config, yaml } => {
            let config = config.unwrap_or(PathBuf::new().join("./Solar.yml"));

            let config: Config = serde_yaml::from_str(&read_to_string(config).with_context(|| "Could not read config.")?)?;

            let braces = if braces {
                true
            } else {
                config.braces
            };

            let (main, mainjit, mainhash) = if let Some(path) = path {
                let src = path.join("src");
                let build = path.join("_build");
                (src.join("main.or"), build.join("main.jit"), build.join("main.hash"))
            } else {
                let src = PathBuf::new().join("src");
                let build = PathBuf::new().join("_build");

                (src.join("main.or"), build.join("main.jit"), build.join("main.hash"))
            };

            let contents = read_to_string(main).with_context(|| "Could not read main file `main.rs`")?;

            let recompile = if mainhash.exists() {
                let hash = read_to_string(&mainhash).with_context(|| "Could not read hash file `main.hash`")?;

                if hash.replace(|c: char| c.to_string().trim().is_empty(), "")
                    != contents.replace(|c: char| c.to_string().trim().is_empty(), "") {
                    write(mainhash, &contents).with_context(|| "Could not write hash `main.hash`.")?;

                    true
                } else {
                    false
                }
            } else {
                println!("{}", mainhash.display());
                write(mainhash, &contents).with_context(|| "Could not write hash `main.hash`.")?;

                true
            };

            if recompile {
                write(mainjit, encode(&contents, braces, yaml)?).with_context(|| "Could not write JIT file.")?;
            }
        }
        Commands::Jit { jit, yaml } => {
            let jit = read_to_string(jit).with_context(|| "Could not read JIT file.")?;
            let ast = decode(jit, yaml)?;

            run_ast(ast).with_context(|| "Could not run project.")?;
        }
    }

    Ok(())
}
