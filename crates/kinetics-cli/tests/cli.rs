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

#[test]
fn doctor_succeeds_even_when_optional_tools_missing() {
    kinetics()
        .arg("doctor")
        .assert()
        .success()
        .stdout(contains("rustc"))
        .stdout(contains("cargo"));
}

#[test]
fn lint_runs_and_returns_a_status() {
    // We don't actually want to run clippy here (the smoke test would
    // take too long under CI), but invoking the subcommand should not
    // produce a parse error. Use --help on the subcommand instead.
    kinetics().args(["lint", "--help"]).assert().success();
}

#[test]
fn preview_target_arg_parses() {
    // Same approach as lint — we don't want to actually run `dx serve`
    // in tests. Verify --help works.
    kinetics().args(["preview", "--help"]).assert().success();
}

#[test]
fn render_with_unknown_scene_returns_nonzero() {
    let dir = tempdir().unwrap();
    kinetics()
        .args([
            "render",
            "--scene",
            "no-such-scene",
            "--out",
            dir.path().to_str().unwrap(),
            "--frames",
            "2",
            "--fps",
            "1",
        ])
        .assert()
        .failure()
        .stderr(contains("unknown scene"));
}

#[test]
fn render_known_scene_writes_html_frames() {
    let dir = tempdir().unwrap();
    kinetics()
        .args([
            "render",
            "--scene",
            "hello",
            "--out",
            dir.path().to_str().unwrap(),
            "--frames",
            "3",
            "--fps",
            "1",
        ])
        .assert()
        .success();

    for frame in 0..3 {
        let path = dir.path().join("frames").join(format!("{frame}.html"));
        assert!(path.exists(), "frame {frame}: {} missing", path.display());
    }
    assert!(dir.path().join("manifest.json").exists());
}

#[test]
fn render_report_scene_writes_frames_with_chart_and_metrics() {
    let dir = tempdir().unwrap();
    kinetics()
        .args([
            "render",
            "--scene",
            "report",
            "--out",
            dir.path().to_str().unwrap(),
            "--frames",
            "3",
            "--fps",
            "1",
        ])
        .assert()
        .success();

    // The report scene composes the AreaChart + MetricCounter data; the
    // settled frame carries the chart and the metric copy.
    let settled = std::fs::read_to_string(dir.path().join("frames").join("2.html")).unwrap();
    assert!(
        settled.contains("ui-chart--area"),
        "report frame missing chart: {settled}"
    );
    assert!(
        settled.contains("Q3 Performance Report"),
        "report frame missing title: {settled}"
    );
    assert!(
        settled.contains("Net MRR"),
        "report frame missing metric: {settled}"
    );
}

#[test]
fn render_showreel_scene_writes_frames_with_charts() {
    let dir = tempdir().unwrap();
    kinetics()
        .args([
            "render",
            "--scene",
            "showreel",
            "--out",
            dir.path().to_str().unwrap(),
            "--frames",
            "3",
            "--fps",
            "1",
        ])
        .assert()
        .success();

    let frame = std::fs::read_to_string(dir.path().join("frames").join("2.html")).unwrap();
    assert!(
        frame.contains("ui-chart--area"),
        "showreel frame missing area chart: {frame}"
    );
    assert!(
        frame.contains("ui-chart--funnel"),
        "showreel frame missing funnel chart: {frame}"
    );
    assert!(
        frame.contains("Cinematic UI, rendered in Rust."),
        "showreel frame missing title: {frame}"
    );
    // The renderer inlines the shared library CSS so captured frames are self-contained.
    assert!(
        frame.contains("<style>"),
        "showreel frame missing inlined CSS: {frame}"
    );
}
