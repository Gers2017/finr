use anyhow::Ok;
use finr::args::ParseResult;
use finr::{args, find};

fn main() -> anyhow::Result<()> {
    if let Some(ParseResult { config, path }) = args::parse()? {
        let mut result = Vec::with_capacity(6);
        find(path, 0, &config, &mut result)?;

        for e in result.iter() {
            println!("{}", e.display());
        }
    }

    Ok(())
}
