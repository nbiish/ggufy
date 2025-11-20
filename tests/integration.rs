// Copyright © 2025 ᓂᐲᔥ ᐙᐸᓂᒥᑮ-ᑭᓇᐙᐸᑭᓯ (Nbiish Waabanimikii-Kinawaabakizi), also known legally as JUSTIN PAUL KENWABIKISE, professionally documented as Nbiish-Justin Paul Kenwabikise, Anishinaabek Dodem (Anishinaabe Clan): Animikii (Thunder), a descendant of Chief ᑭᓇᐙᐸᑭᓯ (Kinwaabakizi) of the Beaver Island Band, and an enrolled member of the sovereign Grand Traverse Band of Ottawa and Chippewa Indians. This work embodies Traditional Knowledge and Traditional Cultural Expressions. All rights reserved.

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
    cmd.args([
        "--dry-run",
        "-c",
        "hf",
        "owner/repo",
        "--",
        "--ctx-size",
        "4096",
    ]);
    let assert = cmd.assert();
    assert.success();
}
