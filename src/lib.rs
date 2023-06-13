use anyhow::{Context, Ok};
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
pub mod args;

#[derive(Debug)]
pub struct Config {
    pub target: String,
    pub is_dir: bool,
    pub is_extension: bool,
    pub pattern: Option<Regex>,
    pub max_depth: usize,
    pub exclude: HashSet<String>,
    pub include: HashSet<String>,
}

impl Default for Config {
    fn default() -> Self {
        let exclude: HashSet<_> = [
            "node_modules",
            "target",
            ".git",
            ".cargo",
            ".rustup",
            ".npm",
            ".ssh",
            "__pycache__",
        ]
        .into_iter()
        .map(|item| item.to_string())
        .collect();

        Self {
            target: String::new(),
            is_dir: false,
            is_extension: false,
            pattern: None,
            max_depth: 100,
            exclude,
            include: HashSet::default(),
        }
    }
}

impl Config {
    pub fn is_match(&self, name: &str) -> bool {
        if self.is_extension {
            return name.ends_with(&self.target);
        }

        if let Some(ref pattern) = self.pattern {
            return pattern.is_match(name);
        }

        name.contains(&self.target)
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

    let entries = read_dir(root)?;

    for entry in entries.into_iter().filter_map(Result::ok) {
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry.file_type().context("File type Error")?.is_dir();

        if config.is_dir == is_dir && config.is_match(&name) {
            result.push(entry.path());
        }

        if is_dir {
            if !config.include.contains(&name)
                && (name.starts_with('.') || config.exclude.contains(&name))
            {
                continue;
            }

            find(entry.path(), depth + 1, config, result)?;
        }
    }

    Ok(())
}

pub fn print_help() {
    println!("finr [PATTERN] [PATH?] [FLAGS...]");
    println!();
    println!(
        "{:<20} Use [PATTERN] as a regex. Match files or directories with it",
        "--regex | -R"
    );
    println!(
        "{:<20} Use [PATTERN] to match at the end of files or directories",
        "--extension | -e"
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
        "{:<20} Directories to exclude in the search. Expects a name not a path",
        "--exclude | -E"
    );
    println!(
        "{:<20} Directories to include in the search. Expects a name not a path",
        "--include | -i"
    );
}
