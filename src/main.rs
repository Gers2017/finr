use anyhow::Ok;
use finr::args::ParseResult;
use finr::{args, find};
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let ParseResult { config, path } = args::parse()?;
    let mut result = Vec::with_capacity(6);

    // https://nnethercote.github.io/perf-book/io.html
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    find(path, 0, &config, &mut result)?;

    for e in result.iter() {
        writeln!(lock, "{}", e.display())?;
    }

    Ok(())
}
