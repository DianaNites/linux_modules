#![allow(unused_imports, unused_mut, unreachable_code)]
use std::str::from_utf8;

use anyhow::Result;
use assert_cmd::{prelude::*, Command};
use insta::assert_snapshot;

/// amdgpu is a complicated module that makes use of many features,
/// all of which we want to display correctly.
///
/// This test will inherently have unstable output, the goal is to make sure
/// the formatting is correct, not the contents.
// TODO: Figure out `insta` filters to ignore changing contents
#[test]
#[ignore = "Flaky"]
fn amdgpu_info() -> Result<()> {
    let mut cmd = Command::cargo_bin("nms")?;
    cmd //.
        .env_clear()
        .env("COLUMNS", "120")
        .args(["info", "amdgpu"]);
    let out = cmd.output()?;
    let out = from_utf8(&out.stdout)?;

    assert_snapshot!(out);

    Ok(())
}

#[test]
fn help() -> Result<()> {
    for arg in [
        &["--help"][..],
        &["list", "--help"][..],
        &["insert", "--help"][..],
        &["remove", "--help"][..],
        &["info", "--help"][..],
    ] {
        let mut cmd = Command::cargo_bin("nms")?;
        cmd.env_clear().args(arg);
        let out = cmd.output()?;
        let out = from_utf8(&out.stdout)?;

        assert_snapshot!(out);
    }

    Ok(())
}

#[test]
#[ignore = "Flaky"]
fn list() -> Result<()> {
    let mut cmd = Command::cargo_bin("nms")?;
    cmd.env_clear().env("COLUMNS", "120").args(["list"]);
    let out = cmd.output()?;
    let out = from_utf8(&out.stdout)?;

    assert_snapshot!(out);

    Ok(())
}
