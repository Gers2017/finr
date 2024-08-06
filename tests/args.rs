use anyhow;
use finr::{args::*, MatchMode};

#[test]
fn basic_parse_test() -> anyhow::Result<()> {
    let args = vec![
        "main",
        ".",
        "--type",
        "FILE",
        "--max-depth",
        "300",
        "--regex",
        "--ignore-case",
        "--include-hidden",
        "--exclude",
        "foo",
        "bar",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .peekable();

    let ParseResult { config, .. } = parse(args)?;

    assert_eq!(config.target, String::from("main"));
    assert_eq!(config.is_dir, false);
    assert_eq!(config.max_depth, 300);
    assert_eq!(config.match_mode, MatchMode::Regex);
    assert_eq!(config.ignore_case, true);
    assert_eq!(config.include_hidden, true);
    assert!(config.exclude.contains("foo") && config.exclude.contains("bar"));

    Ok(())
}
