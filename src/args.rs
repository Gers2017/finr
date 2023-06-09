use anyhow::{Context, Ok};
use regex::RegexBuilder;

use crate::{print_help, Config};
use std::{env, path::PathBuf};

pub struct ParseResult {
    pub config: Config,
    pub path: PathBuf,
}

pub fn parse() -> anyhow::Result<ParseResult> {
    let mut args_iter = env::args().skip(1).peekable();
    let mut config = Config::default();
    let mut path = env::current_dir()?;
    let mut ignore_case = false;

    if let Some(target) = args_iter.next() {
        if target == "--help" {
            print_help();
            std::process::exit(0);
        }

        config.target = target;
    } else {
        print_help();
        std::process::exit(0);
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
            "--start" | "-s" => {
                config.match_mode = 1;
            }

            "--end" | "-e" => {
                config.match_mode = 2;
            }

            "--regex" | "-R" => {
                config.match_mode = 3;
            }

            "--ignore-case" | "-i" => {
                ignore_case = true;
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

            "--include" | "-I" => {
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

            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }

            _ => {
                anyhow::bail!("Unknown flag: \"{}\"", arg)
            }
        }
    }

    if config.match_mode == 3 {
        let regex = RegexBuilder::new(&config.target)
            .case_insensitive(ignore_case)
            .build()?;
        config.regex = Some(regex);
    }

    Ok(ParseResult { config, path })
}
