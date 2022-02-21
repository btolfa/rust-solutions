use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn die_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}