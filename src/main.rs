use anyhow::Ok;
use bpaf::*;
use finr::{find, Out};
use std::collections::HashSet;
use std::env;

fn opts() -> anyhow::Result<OptionParser<Out>> {
    let target = positional::<String>("TARGET").help("Name to use in search");
    let path = positional("PATH")
        .help("Path to start search")
        .fallback(env::current_dir()?);
    let max_depth = short('m')
        .long("max-depth")
        .help("Max recursion depth. By default is 100")
        .argument::<usize>("M")
        .fallback(100);
    let is_dir = short('d')
        .long("dir")
        .help("Search for directories or files")
        .switch();
    let ignore = short('i')
        .long("ignore")
        .help("Directories to ignore. Expects a name not a path")
        .argument::<String>("IGNORE")
        .many()
        .parse(|x| {
            let mut h: HashSet<String> = [
                "node_modules",
                "target",
                ".git",
                ".cargo",
                ".rustup",
                ".npm",
                ".ssh",
                "__pycache__",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect();
            h.extend(x.into_iter());
            Ok(h)
        });

    let include = short('n')
        .long("include")
        .help("Directories to include. Expects a name not a path")
        .argument::<String>("INCLUDE")
        .many()
        .parse(|x| Ok(HashSet::from_iter(x.to_owned().into_iter())));
    let extension = short('e')
        .long("extension")
        .help("match at the end of the file or directory")
        .switch();

    Ok(construct!(Out {
        max_depth,
        is_dir,
        ignore,
        include,
        extension,
        target,
        path,
    })
    .to_options())
}

fn main() -> anyhow::Result<()> {
    let cli = opts()?.run();

    let mut result = Vec::with_capacity(6);
    find(&cli.path, 0, &cli, &mut result)?;

    for e in result.iter() {
        println!("{}", e.display());
    }

    Ok(())
}
