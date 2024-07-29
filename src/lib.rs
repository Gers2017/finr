use anyhow::{Context, Ok};
use fxhash::FxHashSet as HashSet;
use regex::Regex;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
pub mod args;

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum MatchMode {
    Contains = 0,
    Start = 1,
    End = 2,
    Regex = 3,
}

#[derive(Debug)]
pub struct Config {
    pub target: String,
    pub is_dir: bool,
    pub match_mode: MatchMode,
    pub regex: Option<Regex>,
    pub max_depth: usize,
    pub exclude: HashSet<String>,
    pub include: HashSet<String>,
}

impl Default for Config {
    fn default() -> Self {
        let exclude = HashSet::from_iter(
            [
                "node_modules",
                "target",
                "debug",
                "__pycache__",
                "cache",
                "output",
                "bower_components",
                "web_modules",
                "out",
                "dist",
                "coverage",
                ".git",
                ".cargo",
                ".rustup",
                ".npm",
                ".ssh",
            ]
            .into_iter()
            .map(|s| s.to_string()),
        );

        Self {
            target: String::default(),
            is_dir: false,
            match_mode: MatchMode::Contains,
            regex: None,
            max_depth: 100,
            exclude,
            include: HashSet::default(),
        }
    }
}

pub fn get_match_fn(config: &Config) -> impl Fn(&str, &Config) -> bool {
    match config.match_mode {
        MatchMode::Contains => |name: &str, config: &Config| name.contains(&config.target),
        MatchMode::Start => |name: &str, config: &Config| name.starts_with(&config.target),
        MatchMode::End => |name: &str, config: &Config| name.ends_with(&config.target),
        MatchMode::Regex => |name: &str, config: &Config| {
            config
                .regex
                .as_ref()
                .map(|r| r.is_match(name))
                .unwrap_or_default()
        },
    }
}

pub fn find<P: AsRef<Path>>(
    root: P,
    depth: usize,
    config: &Config,
    result: &mut Vec<PathBuf>,
    match_fn: &impl Fn(&str, &Config) -> bool,
) -> anyhow::Result<()> {
    if depth > config.max_depth {
        return Ok(());
    }

    for entry in read_dir(root)?.flatten() {
        let name = entry.file_name();
        let name = name.to_str();

        if name.is_none() {
            continue;
        }

        let name = name.unwrap();
        let is_dir = entry.file_type().context("File type Error")?.is_dir();

        if config.is_dir == is_dir && match_fn(name, config) {
            result.push(entry.path());
        }

        if is_dir {
            if !config.include.contains(name)
                && (name.starts_with('.') || config.exclude.contains(name))
            {
                continue;
            }

            find(entry.path(), depth + 1, config, result, match_fn)?;
        }
    }

    Ok(())
}

pub fn print_help() {
    println!("finr [TARGET] [PATH?] [FLAGS...]");
    println!();
    println!(
        "{:<20} Use [TARGET] as a regular expression (regex) to match files or directories",
        "--regex | -R"
    );
    println!(
        "{:<20} Use [TARGET] to match at the start of files or directories",
        "--start | -s"
    );
    println!(
        "{:<20} Use [TARGET] to match at the end of files or directories",
        "--end | -e"
    );
    println!(
        "{:<20} The search is case-insensitive, this only works with --regex flag",
        "--ignore-case | -i"
    );
    println!(
        "{:<20} Maximum depth of search. By default is set to 100",
        "--max-depth | -d"
    );
    println!(
        "{:<20} Type of search. Possible values: f | file | directory | d (By default is set to file)",
        "--type | -t"
    );
    println!(
        "{:<20} Directories to exclude in the search. Expects names, not paths",
        "--exclude | -E"
    );
    println!(
        "{:<20} Directories to include in the search. Expects names, not paths",
        "--include | -I"
    );
}
