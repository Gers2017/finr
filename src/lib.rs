use anyhow::{Context, Ok};
use fxhash::FxHashSet as HashSet;
use regex::Regex;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub mod args;

pub const DEFAULT_EXCLUDE_LIST: [&str; 11] = [
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
];

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
    pub ignore_case: bool,
    pub include_hidden: bool,
    pub exclude: HashSet<String>,
    pub include: HashSet<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target: String::default(),
            is_dir: false,
            match_mode: MatchMode::Contains,
            regex: None,
            max_depth: 100,
            ignore_case: false,
            include_hidden: false,
            exclude: HashSet::default(),
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

pub struct FindResult {
    pub entries: Vec<PathBuf>,
    pub entries_analyzed: usize,
    pub skipped_entries: usize,
}

impl Default for FindResult {
    fn default() -> Self {
        FindResult {
            entries: Vec::new(),
            entries_analyzed: 0,
            skipped_entries: 0,
        }
    }
}

pub fn find<P: AsRef<Path>>(
    root: P,
    depth: usize,
    config: &Config,
    result: &mut FindResult,
    match_fn: &impl Fn(&str, &Config) -> bool,
) -> anyhow::Result<()> {
    if depth > config.max_depth {
        return Ok(());
    }

    for entry in read_dir(root)?.flatten() {
        let name = entry.file_name().to_str().map(|s| s.to_owned());

        if name.is_none() {
            continue;
        }

        let mut name = name.unwrap();
        let is_dir = entry.file_type().context("File type Error")?.is_dir();

        if config.ignore_case {
            name = name.to_lowercase();
        }

        if config.is_dir == is_dir && match_fn(&name, config) {
            result.entries.push(entry.path());
        }

        if is_dir {
            if !config.include.contains(&name) {
                if (name.starts_with('.') && !config.include_hidden)
                    || config.exclude.contains(&name)
                {
                    result.skipped_entries += 1;
                    continue;
                }
            }

            find(entry.path(), depth + 1, config, result, match_fn)?;
        }

        result.entries_analyzed += 1;
    }

    Ok(())
}

pub fn print_help() {
    println!("finr [TARGET] [PATH?] [FLAGS...]");
    println!();
    println!(
        "{:<24} Use [TARGET] as a regular expression (regex) to match files or directories",
        "--regex | -R"
    );
    println!(
        "{:<24} Use [TARGET] to match at the start of files or directories",
        "--start | -s"
    );
    println!(
        "{:<24} Use [TARGET] to match at the end of files or directories",
        "--end | -e"
    );
    println!(
        "{:<24} The search is case-insensitive. Before processing, transform the name of the entry into lowercase",
        "--ignore-case | -i"
    );
    println!(
        "{:<24} Maximum depth of search. By default is set to 100",
        "--max-depth | -d"
    );
    println!(
        "{:<24} Type of search. Possible values: f | file | directory | d (By default is set to file)",
        "--type | -t"
    );
    println!(
        "{:<24} Include all hidden directories in the search. Turned off by default",
        "--include-hidden | -H"
    );
    println!(
        "{:<24} Directories to exclude in the search. Expects names, not paths",
        "--exclude | -E"
    );
    println!(
        "{:<24} Directories to include in the search. Expects names, not paths",
        "--include | -I"
    );
}
