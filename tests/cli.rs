use anyhow::Ok;
use assert_cmd::prelude::*;
use assert_fs::{
    self,
    prelude::{FileTouch, PathChild},
};
use predicates::{prelude::*, str::contains};
use std::process::Command;

#[test]
fn search_files_regex() -> anyhow::Result<()> {
    let root = assert_fs::TempDir::new()?;

    let paths = [
        "user/code/projects/secrets/finr2/src/main.rs",
        "user/code/avocado/src/lib.rs",
        "user/code/gopkg/src/main.go",
        "user/code/gopkg/src/utils/mod.rs",
    ];

    for path in paths {
        root.child(path).touch()?;
    }

    let mut cmd = Command::cargo_bin("finr")?;
    cmd.arg(".+\\.rs").arg(root.path()).arg("-R");

    cmd.assert().success().stdout(
        contains("main.rs")
            .and(contains("lib.rs"))
            .and(contains("mod.rs")),
    );

    root.close()?;
    Ok(())
}
