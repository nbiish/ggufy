use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn help_runs() {
    let mut cmd = Command::cargo_bin("ggufy").unwrap();
    cmd.arg("--help");
    let assert = cmd.assert();
    assert.success();
}

#[test]
fn dry_run_llama_hf() {
    let mut cmd = Command::cargo_bin("ggufy").unwrap();
    cmd.args(["--dry-run", "-c", "hf", "owner/repo", "--", "--ctx-size", "4096"]);
    let assert = cmd.assert();
    assert.success();
}