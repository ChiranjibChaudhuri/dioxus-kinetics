use std::process::Command;

pub fn run(target: &str) -> Result<(), String> {
    if target != "gallery" {
        return Err(format!(
            "preview --target={target} not yet supported; only \"gallery\" is wired in SP-4+5+6"
        ));
    }
    // Defer to `dx serve --hot-reload` on the component-gallery example.
    let status = Command::new("dx")
        .args(["serve", "--hot-reload"])
        .current_dir("examples/component-gallery")
        .status()
        .map_err(|e| format!("failed to spawn `dx`: {e}. Install dioxus-cli."))?;
    if !status.success() {
        return Err(format!("dx serve exited {}", status.code().unwrap_or(-1)));
    }
    Ok(())
}
