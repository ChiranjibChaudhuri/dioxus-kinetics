use assert_cmd::Command;
use predicates::str::contains;

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
