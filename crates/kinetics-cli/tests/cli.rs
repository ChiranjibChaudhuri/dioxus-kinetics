use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use tempfile::tempdir;

fn kinetics() -> Command {
    Command::cargo_bin("kinetics").expect("binary exists")
}

#[test]
fn help_lists_five_subcommands() {
    kinetics()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("init"))
        .stdout(contains("preview"))
        .stdout(contains("render"))
        .stdout(contains("lint"))
        .stdout(contains("doctor"));
}

#[test]
fn version_prints() {
    kinetics().arg("--version").assert().success();
}

#[test]
fn unknown_subcommand_returns_nonzero() {
    kinetics().arg("nope").assert().failure();
}

#[test]
fn init_creates_scaffolded_directory() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("hello");
    kinetics()
        .current_dir(dir.path())
        .args(["init", "hello"])
        .assert()
        .success();

    assert!(target.exists(), "target dir created");
    assert!(target.join("Cargo.toml").exists(), "Cargo.toml exists");
    assert!(target.join("src/main.rs").exists(), "main.rs exists");
    let main = fs::read_to_string(target.join("src/main.rs")).unwrap();
    assert!(main.contains("Scene"), "template references Scene");
}
