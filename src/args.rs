use anyhow::{Context, Ok};
use regex::Regex;

use crate::{print_help, Config};
use std::{env, path::PathBuf};

pub struct ParseResult {
    pub config: Config,
    pub path: PathBuf,
}

/// Parses command line arguments.
///
/// If the result is Ok and the inner value is None, either no arguments were given or the --help flag was present.
///
/// If the result is Ok and the inner value is Some, then the parsing was successful.
pub fn parse() -> anyhow::Result<Option<ParseResult>> {
    let mut args_iter = env::args().skip(1).peekable();
    let mut config = Config::default();
    let mut path = env::current_dir()?;

    if let Some(target) = args_iter.next() {
        if target == "--help" {
            print_help();
            return Ok(None);
        }

        config.target = target;
    } else {
        print_help();
        return Ok(None);
    }

    if let Some(arg) = args_iter.peek() {
        if !arg.starts_with('-') {
            path = PathBuf::from(arg);
            // skip path
            args_iter.next();
        }
    }

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "--regex" | "-R" => {
                config.pattern = Some(Regex::new(&config.target)?);
            }

            "--max-depth" | "-d" => {
                config.max_depth = args_iter
                    .next()
                    .ok_or(anyhow::anyhow!("Missing argument for --max-depth flag"))?
                    .parse::<usize>()
                    .context("Argument for --max-depth should be a positive integer")?;
            }

            "--type" | "-t" => {
                let arg = args_iter
                    .next()
                    .ok_or(anyhow::anyhow!("Missing Argument for --type flag"))?
                    .to_lowercase();

                config.is_dir = match arg.as_str() {
                    "file" | "f" => Ok(false),
                    "directory" | "d" => Ok(true),
                    _ => anyhow::bail!(
                    "Invalid argument \"{}\" for --type flag. Valid arguments [file | directory]",
                    arg
                ),
                }?;
            }

            "--exclude" | "-E" => {
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

            "--include" | "-i" => {
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

            "--extension" | "-e" => config.is_extension = true,

            "--help" | "-h" => {
                print_help();
                return Ok(None);
            }

            _ => {
                anyhow::bail!("Unknown flag: \"{}\"", arg)
            }
        }
    }

    Ok(Some(ParseResult { config, path }))
}
