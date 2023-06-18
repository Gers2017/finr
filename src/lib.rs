use anyhow::{Context, Ok};
use fxhash::FxHashSet as HashSet;
use regex::Regex;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
pub mod args;

#[derive(Debug)]
pub struct Config {
    pub target: String,
    pub is_dir: bool,
    pub match_mode: u8,
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
            match_mode: 0u8,
            regex: None,
            max_depth: 100,
            exclude,
            include: HashSet::default(),
        }
    }
}

impl Config {
    pub fn is_match(&self, name: &str) -> bool {
        match self.match_mode {
            1 => name.starts_with(&self.target),
            2 => name.ends_with(&self.target),
            3 => {
                return self
                    .regex
                    .as_ref()
                    .map(|r| r.is_match(name))
                    .unwrap_or_default()
            }
            _ => name.len() >= self.target.len() && name.contains(&self.target),
        }
    }
}

pub fn find<P: AsRef<Path>>(
    root: P,
    depth: usize,
    config: &Config,
    result: &mut Vec<PathBuf>,
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

        if config.is_dir == is_dir && config.is_match(&name) {
            result.push(entry.path());
        }

        if is_dir {
            if !config.include.contains(name)
                && (name.starts_with('.') || config.exclude.contains(name))
            {
                continue;
            }

            find(entry.path(), depth + 1, config, result)?;
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
        "--include | -i"
    );
}
