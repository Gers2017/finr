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

#[test]
fn search_files_simple() -> anyhow::Result<()> {
    let root = assert_fs::TempDir::new()?;

    let paths = [
        "user/code/rust/secrets/src/monkey-types.rs",
        "user/code/foo/bar/moon.rs",
        "user/code/typescript/src/modules/monkey-provider.ts",
        "user/code/cli-monkey.go",
    ];

    for path in paths {
        root.child(path).touch()?;
    }

    let mut cmd = Command::cargo_bin("finr")?;
    cmd.arg("monkey").arg(root.path());

    cmd.assert().success().stdout(
        contains("monkey-types.rs")
            .and(contains("cli-monkey.go"))
            .and(contains("monkey-provider.ts")),
    );

    Ok(())
}

#[test]
fn search_ignore_case() -> anyhow::Result<()> {
    let root = assert_fs::TempDir::new()?;

    let paths = [
        "user/code/foo/bar/delete.ts",
        "user/code/foo/buzz/DELETE.rs",
        "user/code/bar/foo/hei/src/modules/DELeTe.go",
        "user/code/Hi-DeleteMe.txt",
    ];

    for path in paths {
        root.child(path).touch()?;
    }

    let mut cmd = Command::cargo_bin("finr")?;
    cmd.arg(".?delete.?")
        .arg(root.path())
        .arg("-R")
        .arg("--ignore-case");

    cmd.assert().success().stdout(
        contains("delete.ts")
            .and(contains("DELETE.rs"))
            .and(contains("DELeTe.go"))
            .and(contains("Hi-DeleteMe.txt")),
    );

    Ok(())
}

#[test]
fn empty_arguments() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("finr")?;
    cmd.assert().success();
    Ok(())
}

#[test]
fn uppercase_type_argument() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("finr")?;
    cmd.arg("main.ts").arg("--type").arg("DIRECTORY");
    cmd.assert().success();
    Ok(())
}

#[test]
fn missing_type_argument() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("finr")?;
    cmd.arg(".+\\.ts").arg("--regex").arg("--type");
    cmd.assert().failure();
    Ok(())
}

#[test]
fn invalid_type_argument() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("finr")?;
    cmd.arg(".+\\.ts").arg("--regex").arg("--type").arg("TXT");
    cmd.assert().failure();
    Ok(())
}

#[test]
fn invalid_max_depth_argument() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("finr")?;
    cmd.arg("main.go").arg("--max-depth").arg("not-a-number");
    cmd.assert().failure();
    Ok(())
}

#[test]
fn unknown_flag() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("finr")?;
    cmd.arg(".+\\.ts")
        .arg("--regex")
        .arg("--min-depth")
        .arg("50")
        .arg("--typo");
    cmd.assert().failure().stderr(contains("Unknown flag"));
    Ok(())
}
