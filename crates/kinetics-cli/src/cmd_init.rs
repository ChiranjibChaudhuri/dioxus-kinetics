use std::fs;
use std::path::PathBuf;

const INIT_MAIN_TEMPLATE: &str = include_str!("template/init_main.rs.tmpl");

pub fn run(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("name must not be empty".into());
    }
    let target = PathBuf::from(name);
    if target.exists() {
        return Err(format!(
            "{} already exists; pick a fresh directory name",
            target.display()
        ));
    }

    fs::create_dir_all(target.join("src"))
        .map_err(|e| format!("create_dir_all: {e}"))?;

    let cargo_toml = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
dioxus = "0.7"
kinetics = {{ path = "../crates/kinetics" }}
ui-composition = {{ path = "../crates/ui-composition" }}
"#,
        name = name,
    );
    fs::write(target.join("Cargo.toml"), cargo_toml)
        .map_err(|e| format!("write Cargo.toml: {e}"))?;
    fs::write(target.join("src/main.rs"), INIT_MAIN_TEMPLATE)
        .map_err(|e| format!("write main.rs: {e}"))?;

    println!("Created {name}/.");
    println!("Next steps:");
    println!("  cd {name}");
    println!("  cargo run");
    Ok(())
}
