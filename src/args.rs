use anyhow::{Context, Ok};
use regex::RegexBuilder;

use crate::{print_help, Config, MatchMode};
use std::{env, path::PathBuf};

pub struct ParseResult {
    pub config: Config,
    pub path: PathBuf,
}

pub fn parse<I: Iterator<Item = String>>(
    mut iter: std::iter::Peekable<I>,
) -> anyhow::Result<ParseResult> {
    let mut config = Config::default();
    let mut path = env::current_dir()?;

    if let Some(target) = iter.next() {
        if target == "--help" {
            print_help();
            std::process::exit(0);
        }

        config.target = target;
    } else {
        print_help();
        std::process::exit(0);
    }

    if let Some(arg) = iter.peek() {
        if !arg.starts_with('-') {
            path = PathBuf::from(arg);
            // skip path
            iter.next();
        }
    }

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--start" | "-s" => {
                config.match_mode = MatchMode::Start;
            }

            "--end" | "-e" => {
                config.match_mode = MatchMode::End;
            }

            "--regex" | "-R" => {
                config.match_mode = MatchMode::Regex;
            }

            "--ignore-case" | "-i" => {
                config.ignore_case = true;
            }

            "--max-depth" | "-d" => {
                config.max_depth = iter
                    .next()
                    .ok_or(anyhow::anyhow!("Missing argument for --max-depth flag"))?
                    .parse::<usize>()
                    .context("Argument for --max-depth should be a positive integer")?;
            }

            "--type" | "-t" => {
                let arg = iter
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
                while let Some(current) = iter.peek() {
                    if current.starts_with('-') {
                        break;
                    }

                    let dir = iter.next().unwrap().trim().to_string();

                    if dir.is_empty() {
                        anyhow::bail!("Invalid value for --exclude argument: \"{}\"", dir);
                    }

                    config.exclude.insert(dir);
                }
            }

            "--include" | "-I" => {
                while let Some(current) = iter.peek() {
                    if current.starts_with('-') {
                        break;
                    }

                    let dir = iter.next().unwrap().trim().to_string();

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

    if config.match_mode == MatchMode::Regex {
        let regex = RegexBuilder::new(&config.target)
            .case_insensitive(config.ignore_case)
            .build()?;
        config.regex = Some(regex);
    }

    Ok(ParseResult { config, path })
}
