use anyhow::{Context, Ok};
use std::collections::HashSet;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Out {
    pub target: String,
    pub path: PathBuf,
    pub max_depth: usize,
    pub is_dir: bool,
    pub ignore: HashSet<String>,
    pub include: HashSet<String>,
    pub extension: bool,
}

impl Out {
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
    config: &Out,
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
                && (name.starts_with('.') || config.ignore.contains(&name))
            {
                continue;
            }

            find(entry.path(), depth + 1, config, result)?;
        }
    }

    Ok(())
}
