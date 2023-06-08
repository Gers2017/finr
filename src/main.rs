use std::env;

use anyhow::Ok;
use finr::{find, Config};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let mut config = Config::parse();

    if config.ignore_dirs.is_empty() {
        config.ignore_dirs = [
            "node_modules",
            "target",
            ".git",
            ".cargo",
            ".rustup",
            ".npm",
            ".ssh",
            "__pycache__",
        ].into_iter().map(ToString::to_string).collect()
    }

    let mut result = Vec::with_capacity(6);
    find(config.path.clone().unwrap_or(env::current_dir()?), 0, &config, &mut result)?;

    for e in result.iter() {
        println!("{}", e.display());
    }

    Ok(())
}
