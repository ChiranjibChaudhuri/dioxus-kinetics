use std::process::Command;

pub fn run() -> Result<(), String> {
    eprintln!("$ cargo fmt --all -- --check");
    let fmt_status = Command::new("cargo")
        .args(["fmt", "--all", "--", "--check"])
        .status()
        .map_err(|e| format!("failed to spawn cargo fmt: {e}"))?;
    if !fmt_status.success() {
        return Err(format!(
            "cargo fmt --check failed (exit {}). Run `cargo fmt --all` to fix.",
            fmt_status.code().unwrap_or(-1)
        ));
    }

    eprintln!("$ cargo clippy --workspace --all-targets -- -D warnings");
    let clippy_status = Command::new("cargo")
        .args([
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ])
        .status()
        .map_err(|e| format!("failed to spawn cargo clippy: {e}"))?;
    if !clippy_status.success() {
        return Err(format!(
            "clippy failed (exit {})",
            clippy_status.code().unwrap_or(-1)
        ));
    }
    Ok(())
}
