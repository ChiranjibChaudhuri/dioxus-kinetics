use std::process::Command;

pub fn run() -> Result<(), String> {
    println!("kinetics doctor — toolchain check");
    println!();
    report_tool("rustc", &["--version"]);
    report_tool("cargo", &["--version"]);
    report_tool("dx", &["--version"]);
    report_tool("node", &["--version"]);
    report_tool("npx", &["--version"]);
    report_tool("playwright", &["--version"]);
    report_tool("ffmpeg", &["-version"]);
    println!();
    println!("(missing optional tools are not errors; install them to unlock");
    println!("`kinetics render --capture-png` and `--encode-mp4`.)");
    Ok(())
}

fn report_tool(cmd: &str, args: &[&str]) {
    let result = Command::new(cmd).args(args).output();
    match result {
        Ok(out) if out.status.success() => {
            let v = String::from_utf8_lossy(&out.stdout);
            let v = v.lines().next().unwrap_or(v.trim()).trim();
            println!("  {cmd:<12} {v}");
        }
        Ok(_) | Err(_) => {
            println!("  {cmd:<12} not found");
        }
    }
}
