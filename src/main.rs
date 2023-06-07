use anyhow::{Context, Ok};
use std::collections::HashSet;
use std::env;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

fn print_help() {
    println!("finr [PATTERN] [PATH?] [FLAGS...]");
    println!();
    println!("{:<20} Max recursion depth", "--max-depth | -d");
    println!("{:<20} Type of entry to search", "--type | -t");
    println!("{:<20} Directories to ignore", "--ignore | -i");
    println!("{:<20} Directories to include", "--include | -n");
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

                let dir = args_iter.next().unwrap().trim().to_string();

                if dir.is_empty() {
                    anyhow::bail!("Invalid value for --ignore argument: \"{}\"", dir);
                }

                config.ignore_dirs.insert(dir);
            }
        }

        if arg == "--include" || arg == "-n" {
            while let Some(current) = args_iter.peek() {
                if current.starts_with('-') {
                    break;
                }

                let dir = args_iter.next().unwrap().trim().to_string();

                if dir.is_empty() {
                    anyhow::bail!("Invalid value for --include argument: \"{}\"", dir);
                }

                config.include_dirs.insert(dir);
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

#[derive(Debug)]
pub struct Config {
    pub target: String,
    pub entry_type: EntryType,
    pub is_extension: bool,
    pub max_depth: usize,
    pub ignore_dirs: HashSet<String>,
    pub include_dirs: HashSet<String>,
}

impl Default for Config {
    fn default() -> Self {
        let ignore_dirs: HashSet<_> = [
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
            entry_type: EntryType::File,
            is_extension: false,
            max_depth: 100,
            ignore_dirs,
            include_dirs: HashSet::default(),
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

        if config.entry_type.is_dir() == is_dir && config.is_match(&name) {
            result.push(entry.path());
        }

        if is_dir {
            if !config.include_dirs.contains(&name)
                && (name.starts_with('.') || config.ignore_dirs.contains(&name))
            {
                continue;
            }

            find(entry.path(), depth + 1, config, result)?;
        }
    }

    Ok(())
}
