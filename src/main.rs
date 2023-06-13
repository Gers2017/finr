use anyhow::{Context, Ok};
use finr::{find, print_help, Config};
use regex::Regex;
use std::env;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let mut args_iter = env::args().skip(1).peekable();
    let mut config = Config::default();

    if let Some(target) = args_iter.next() {
        if target == "--help" {
            print_help();
            return Ok(());
        }

        config.target = target;
    } else {
        print_help();
        return Ok(());
    }

    let mut path = env::current_dir()?;

    if let Some(arg) = args_iter.peek() {
        if !arg.starts_with('-') {
            path = PathBuf::from(arg);
        }
    }

    while let Some(arg) = args_iter.next() {
        if arg == "--regex" || arg == "-R" {
            config.pattern = Some(Regex::new(&config.target)?);
        }

        if arg == "--max-depth" || arg == "-d" {
            config.max_depth = args_iter
                .next()
                .ok_or(anyhow::anyhow!("Missing argument for --max-depth flag"))?
                .parse::<usize>()
                .context("Argument for --max-depth should be a positive integer")?;
        }

        if arg == "--type" || arg == "-t" {
            let arg = args_iter
                .next()
                .ok_or(anyhow::anyhow!("Missing Argument for --type flag"))?;

            config.is_dir = match arg.as_str() {
                "file" | "f" => Ok(false),
                "directory" | "d" => Ok(true),
                _ => anyhow::bail!(
                    "Invalid argument \"{}\" for --type flag. Valid arguments [file | directory]",
                    arg
                ),
            }?;
        }

        if arg == "--exclude" || arg == "-E" {
            while let Some(current) = args_iter.peek() {
                if current.starts_with('-') {
                    break;
                }

                let dir = args_iter.next().unwrap().trim().to_string();

                if dir.is_empty() {
                    anyhow::bail!("Invalid value for --exclude argument: \"{}\"", dir);
                }

                config.exclude.insert(dir);
            }
        }

        if arg == "--include" || arg == "-i" {
            while let Some(current) = args_iter.peek() {
                if current.starts_with('-') {
                    break;
                }

                let dir = args_iter.next().unwrap().trim().to_string();

                if dir.is_empty() {
                    anyhow::bail!("Invalid value for --include argument: \"{}\"", dir);
                }

                config.include.insert(dir);
            }
        }

        if arg == "--extension" || arg == "-e" {
            config.is_extension = true
        }

        if arg == "--help" {
            print_help();
            return Ok(());
        }
    }

    let mut result = Vec::with_capacity(6);
    find(path, 0, &config, &mut result)?;

    for e in result.iter() {
        println!("{}", e.display());
    }

    Ok(())
}
