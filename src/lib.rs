use anyhow::{Context, Ok};
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    pub target: String,

    #[arg(short = 'r')]
    pub dir: bool,

    #[arg(short = 'e')]
    pub extension: bool,

    #[arg(short = 'd', default_value = "100")]
    pub max_depth: usize,

    #[arg(short = 'I')]
    pub ignore_dirs: Vec<String>,

    #[arg(short = 'i')]
    pub include_dirs: Vec<String>,

    #[arg(short = 'p')]
    pub path: Option<PathBuf>
}

impl Config {
    pub fn is_match(&self, filename: &str) -> bool {
        if self.extension {
            filename.ends_with(&self.target)
        } else {
            filename.contains(&self.target)
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
        let is_dir = entry.file_type().context("File Type Error")?.is_dir();

        if config.dir == is_dir && config.is_match(&name) {
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
