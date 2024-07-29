use anyhow::Ok;
use finr::args::ParseResult;
use finr::{args, find, get_match_fn};
use std::env;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1).peekable();
    let ParseResult { config, path } = args::parse(args)?;
    let mut result = Vec::with_capacity(6);

    // https://nnethercote.github.io/perf-book/io.html
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let match_fn = get_match_fn(&config);
    find(path, 0, &config, &mut result, &match_fn)?;

    for e in result.iter() {
        writeln!(lock, "{}", e.display())?;
    }

    Ok(())
}
