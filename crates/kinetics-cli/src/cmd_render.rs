use std::path::Path;

use crate::scene_registry;

pub fn run(
    scene: &str,
    out: &Path,
    frames: u32,
    fps: u32,
    capture_png: bool,
    encode_mp4: bool,
    capture_pdf: bool,
) -> Result<(), String> {
    let spec = scene_registry::lookup(scene).ok_or_else(|| {
        format!(
            "unknown scene `{scene}` — available: {}",
            scene_registry::SCENES
                .iter()
                .map(|s| s.id)
                .collect::<Vec<_>>()
                .join(", ")
        )
    })?;
    scene_registry::run_render(spec, out, frames, fps, capture_png, encode_mp4, capture_pdf)
}
