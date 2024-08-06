use anyhow::Ok;
use finr::{
    args::{self, ParseResult},
    find, get_match_fn, FindResult,
};
use std::env;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1).peekable();
    let ParseResult { config, path } = args::parse(args)?;
    let mut result = FindResult::default();

    // https://nnethercote.github.io/perf-book/io.html
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let match_fn = get_match_fn(&config);
    find(path, 0, &config, &mut result, &match_fn)?;

    for e in result.entries.iter() {
        writeln!(lock, "{}", e.display())?;
    }

    Ok(())
}
