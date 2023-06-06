use anyhow::{Context, Ok};
use std::env;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

fn print_help() {
    println!("finr [PATTERN] [PATH?] [FLAGS...]");
    println!();
    println!("{:<20} Max recursion depth", "--max-depth | -d");
    println!("{:<20} Type of entry to search", "--type | -t");
    println!("{:<20} Directories to ignore", "--ignore | -i");
    println!(
        "{:<20} Use [PATTERN] to search at the end",
        "--extension | -e"
    );
}

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

            config.entry_type = match arg.as_str() {
                "file" | "f" => Ok(EntryType::File),
                "directory" | "d" => Ok(EntryType::Directory),
                _ => anyhow::bail!(
                    "Invalid argument \"{}\" for --type flag. Valid arguments [file | directory]",
                    arg
                ),
            }?;
        }

        if arg == "--ignore" || arg == "-i" {
            while let Some(current) = args_iter.peek() {
                if current.starts_with('-') {
                    break;
                }

                let dir = args_iter
                    .next()
                    .ok_or(anyhow::anyhow!("Missing argument for --ignore flag"))?
                    .trim()
                    .to_string();

                if dir.is_empty() {
                    anyhow::bail!("Invalid empty directory \"{}\"", dir);
                }

                config.ignore_dirs.push(dir);
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

    let mut result = FindResult::default();
    find(path, 0, &config, &mut result)?;

    for e in result.entries.iter() {
        println!("{}", e);
    }

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub target: String,
    pub entry_type: EntryType,
    pub is_extension: bool,
    pub max_depth: usize,
    pub ignore_dirs: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        let ignore_dirs: Vec<_> = [
            "node_modules",
            "target",
            ".git",
            ".cargo",
            ".rustup",
            ".npm",
            ".ssh",
        ]
        .into_iter()
        .map(|item| item.to_string())
        .collect();

        Self {
            target: String::new(),
            entry_type: EntryType::File,
            is_extension: false,
            max_depth: 100,
            ignore_dirs,
        }
    }
}

impl Config {
    fn is_match(&self, filename: &str) -> bool {
        if self.is_extension {
            filename.ends_with(&self.target)
        } else {
            filename.contains(&self.target)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum EntryType {
    File = 0,
    Directory = 1,
}

impl EntryType {
    pub fn is_dir(&self) -> bool {
        use EntryType::*;
        match self {
            File => false,
            Directory => true,
        }
    }
}

#[derive(Debug)]
pub struct FindResult {
    pub entries: Vec<String>,
}

impl Default for FindResult {
    fn default() -> Self {
        Self {
            entries: Vec::with_capacity(6),
        }
    }
}

pub fn find<P: AsRef<Path>>(
    root: P,
    depth: usize,
    config: &Config,
    result: &mut FindResult,
) -> anyhow::Result<()> {
    if depth > config.max_depth {
        return Ok(());
    }

    let entries = read_dir(root)?;

    for entry in entries.into_iter().filter_map(Result::ok) {
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry.file_type().context("File type Error")?.is_dir();

        if config.entry_type.is_dir() == is_dir && config.is_match(&name) {
            result.entries.push(entry.path().display().to_string());
        }

        if is_dir {
            if config.ignore_dirs.contains(&name) {
                continue;
            }

            find(entry.path(), depth + 1, config, result)?;
        }
    }

    Ok(())
}
