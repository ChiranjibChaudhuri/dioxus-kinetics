use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

mod cmd_doctor;
mod cmd_init;
mod cmd_lint;
mod cmd_preview;
mod cmd_render;
mod scene_registry;

#[derive(Parser)]
#[command(name = "kinetics", version, about = "Author and render kinetics Scene compositions")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold a new kinetics example crate.
    Init {
        /// Directory name for the new example.
        name: String,
    },
    /// Run the dev server (dx serve --hot-reload).
    Preview {
        /// Build target. Currently only "gallery" is supported.
        #[arg(long, default_value = "gallery")]
        target: String,
    },
    /// Render a named scene to HTML / PNG / MP4.
    Render {
        /// Scene id (must exist in the SceneRegistry).
        #[arg(long)]
        scene: String,
        /// Output directory.
        #[arg(long)]
        out: PathBuf,
        /// Total frame count. Default: 60.
        #[arg(long, default_value_t = 60)]
        frames: u32,
        /// Frames per second. Default: 30.
        #[arg(long, default_value_t = 30)]
        fps: u32,
        /// Also capture PNGs via Playwright (graceful-skip if absent).
        #[arg(long)]
        capture_png: bool,
        /// Also encode an MP4 via FFmpeg (requires --capture-png).
        #[arg(long)]
        encode_mp4: bool,
    },
    /// Run fmt + clippy across the workspace.
    Lint,
    /// Print toolchain versions.
    Doctor,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Init { name } => cmd_init::run(&name),
        Commands::Preview { target } => cmd_preview::run(&target),
        Commands::Render {
            scene,
            out,
            frames,
            fps,
            capture_png,
            encode_mp4,
        } => cmd_render::run(&scene, &out, frames, fps, capture_png, encode_mp4),
        Commands::Lint => cmd_lint::run(),
        Commands::Doctor => cmd_doctor::run(),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kinetics: {e}");
            ExitCode::from(1)
        }
    }
}
