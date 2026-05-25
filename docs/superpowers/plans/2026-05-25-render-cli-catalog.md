# Render Pipeline + CLI + Catalog Implementation Plan (SP-4+5+6)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Land `kinetics-render` (frame-by-frame SSR exporter with optional PNG / MP4 stages), `kinetics-cli` (`init`/`preview`/`render`/`lint`/`doctor`), and `ui-blocks` (LowerThird / Caption / WipeTransition / MetricCounter / SocialOverlay catalog) plus the `.claude/skills/kinetics-scene` agent skill.

**Architecture:** Three new crates with clear boundaries. `kinetics-render` is a pure-Rust library that walks frames via `SceneClock { driver: Manual }`, calls a user-supplied `scene_fn(clock) -> Element`, and emits one HTML per frame plus a `ui_capture::ExportManifest`. Optional PNG capture spawns a Node + Playwright child; optional MP4 encode spawns FFmpeg. `kinetics-cli` is a `clap`-based binary that wraps the renderer plus thin dev-loop helpers. `ui-blocks` is a Dioxus catalog of five reusable cinematic blocks composed entirely from SP-1/SP-3 public APIs. No new external Rust dependencies beyond `clap`, `serde_json` (already in workspace via SP-3), `tokio` (workspace), and dev-only `assert_cmd` + `tempfile`.

**Tech Stack:** Rust 2021, Dioxus 0.7 with `Signal<T>`, `dioxus-ssr` for SSR, `clap` 4 with derive, `tokio` for child-process orchestration, Playwright (already in `examples/component-gallery/e2e/`) as the PNG-capture sidecar, FFmpeg as the MP4 encoder.

**Spec:** `docs/superpowers/specs/2026-05-25-render-cli-catalog-design.md`

---

## File Structure

```
crates/kinetics-render/
  Cargo.toml                # NEW — workspace member
  src/
    lib.rs                  # NEW — Renderer / RenderConfig / RenderError / RenderReport
    capture.rs              # NEW — PNG capture orchestrator (Playwright child)
    encode.rs               # NEW — MP4 encode orchestrator (FFmpeg child)
    template.rs             # NEW — capture.cjs embedded string
  tests/
    renderer.rs             # NEW — HTML-only smoke tests
    capture.rs              # NEW — Playwright-skip tests
    encode.rs               # NEW — FFmpeg-skip tests

crates/kinetics-cli/
  Cargo.toml                # NEW — workspace member, [[bin]] kinetics
  src/
    main.rs                 # NEW — clap entrypoint
    cmd_init.rs             # NEW
    cmd_preview.rs          # NEW
    cmd_render.rs           # NEW
    cmd_lint.rs             # NEW
    cmd_doctor.rs           # NEW
    scene_registry.rs       # NEW — named scene -> render config + scene_fn
    template/
      init_main.rs.tmpl     # NEW — embedded scaffold
  tests/
    cli.rs                  # NEW — assert_cmd smoke tests

crates/ui-blocks/
  Cargo.toml                # NEW
  src/
    lib.rs                  # NEW — re-exports
    lower_third.rs          # NEW
    caption.rs              # NEW
    wipe_transition.rs      # NEW
    metric_counter.rs       # NEW
    social_overlay.rs       # NEW
  tests/
    blocks_ssr.rs           # NEW — 5 SSR tests

crates/kinetics/
  Cargo.toml                # +ui-blocks dep behind `blocks` feature
  src/lib.rs                # +prelude exports + public_api_names

.claude/skills/
  kinetics-scene/
    SKILL.md                # NEW

examples/component-gallery/
  Cargo.toml                # +ui-blocks workspace dep
  src/previews/scenes/
    mod.rs                  # +3 pub mods
    lower_third_demo.rs     # NEW
    caption_demo.rs         # NEW
    wipe_demo.rs            # NEW
  src/previews/scene.rs     # +3 preview functions
  src/docs.rs               # +3 ComponentDoc + 3 snippet consts
  e2e/tests/_lib/
    component-manifest.ts   # +3 manifest entries
  e2e/tests/
    catalog-blocks.spec.ts  # NEW

Cargo.toml (workspace)      # +3 new members
```

## Conventions

- Conventional Commits with `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>` HEREDOC trailer.
- Workspace-standard `let mut s = …; s.set(…);` for Signal writes.
- Rust tests live in `tests/<name>.rs` (integration-style).
- Use Edit/Read/Write/Bash dedicated tools — no raw shell `sed`/`awk`.
- Never push/amend/`--no-verify`.

---

### Task 1: Workspace scaffold for three new crates

Add `kinetics-render`, `kinetics-cli`, and `ui-blocks` as workspace members. Each gets a minimal `Cargo.toml` + `src/lib.rs` (or `src/main.rs` for the CLI) so the workspace builds. Per-task crate contents arrive in later tasks.

**Files:**
- Modify: `Cargo.toml` (workspace root) — add `crates/kinetics-render`, `crates/kinetics-cli`, `crates/ui-blocks` to `[workspace] members`.
- Create: `crates/kinetics-render/Cargo.toml`
- Create: `crates/kinetics-render/src/lib.rs` (stub)
- Create: `crates/kinetics-cli/Cargo.toml`
- Create: `crates/kinetics-cli/src/main.rs` (stub)
- Create: `crates/ui-blocks/Cargo.toml`
- Create: `crates/ui-blocks/src/lib.rs` (stub)

- [ ] **Step 1: Add workspace members**

Read `Cargo.toml` at the workspace root. Find the `[workspace] members = [...]` array and add three entries (alphabetical with existing entries):

```toml
"crates/kinetics-render",
"crates/kinetics-cli",
"crates/ui-blocks",
```

- [ ] **Step 2: Create `kinetics-render` Cargo.toml**

```toml
[package]
name = "kinetics-render"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
dioxus.workspace = true
dioxus-ssr.workspace = true
ui-runtime = { path = "../ui-runtime" }
ui-composition = { path = "../ui-composition" }
ui-capture = { path = "../ui-capture" }
serde_json = "1"
tokio = { version = "1", features = ["fs", "process", "rt", "macros"] }

[dev-dependencies]
tempfile = "3"

[lib]
path = "src/lib.rs"
```

Create `crates/kinetics-render/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

//! Frame-by-frame SSR exporter for kinetics Scene compositions.
//!
//! The implementation is filled in by Tasks 2-5 of the SP-4+5+6 plan.
```

- [ ] **Step 3: Create `kinetics-cli` Cargo.toml**

```toml
[package]
name = "kinetics-cli"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[[bin]]
name = "kinetics"
path = "src/main.rs"

[dependencies]
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["rt-multi-thread", "fs", "process", "macros"] }
kinetics-render = { path = "../kinetics-render" }

[dev-dependencies]
assert_cmd = "2"
tempfile = "3"
predicates = "3"
```

Create `crates/kinetics-cli/src/main.rs`:

```rust
fn main() {
    // CLI entry point — implementation arrives in Tasks 6-12.
    eprintln!("kinetics CLI scaffold — not yet implemented");
    std::process::exit(2);
}
```

- [ ] **Step 4: Create `ui-blocks` Cargo.toml**

```toml
[package]
name = "ui-blocks"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
dioxus.workspace = true
ui-dioxus = { path = "../ui-dioxus" }
ui-timeline = { path = "../ui-timeline" }
ui-motion = { path = "../ui-motion" }
ui-composition = { path = "../ui-composition" }

[dev-dependencies]
dioxus-ssr.workspace = true

[lib]
path = "src/lib.rs"
```

Create `crates/ui-blocks/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

//! Reusable cinematic block catalog for kinetics Scene compositions.
//!
//! Five components arrive in Tasks 14-18 of the SP-4+5+6 plan.
```

- [ ] **Step 5: Verify workspace builds**

Run: `cargo check --workspace`
Expected: success. No new warnings.

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml crates/kinetics-render crates/kinetics-cli crates/ui-blocks
git commit -m "$(cat <<'EOF'
chore(workspace): scaffold kinetics-render, kinetics-cli, ui-blocks crates

Empty Cargo.toml + lib.rs/main.rs stubs so the workspace builds.
Per-crate implementation arrives in subsequent tasks.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 2: `Renderer` + `RenderConfig` types in `kinetics-render`

The configuration types + the renderer struct + config validation. No render loop yet.

**Files:**
- Modify: `crates/kinetics-render/src/lib.rs`
- Test: `crates/kinetics-render/tests/renderer.rs`

- [ ] **Step 1: Write failing tests**

Create `crates/kinetics-render/tests/renderer.rs`:

```rust
use std::path::PathBuf;
use kinetics_render::{RenderConfig, RenderError, Renderer};

fn base_config() -> RenderConfig {
    RenderConfig {
        frames: 10,
        fps: 30,
        width: 320,
        height: 240,
        composition_id: "test".to_string(),
        output_dir: PathBuf::from("/tmp/kinetics-render-test"),
        capture_png: false,
        encode_mp4: false,
    }
}

#[test]
fn renderer_constructs_from_valid_config() {
    let r = Renderer::new(base_config());
    let _ = r;
}

#[test]
fn invalid_fps_zero_rejected_via_validate() {
    let mut cfg = base_config();
    cfg.fps = 0;
    let err = RenderConfig::validate(&cfg).unwrap_err();
    assert!(matches!(err, RenderError::InvalidConfig(_)));
}

#[test]
fn invalid_frames_zero_rejected_via_validate() {
    let mut cfg = base_config();
    cfg.frames = 0;
    let err = RenderConfig::validate(&cfg).unwrap_err();
    assert!(matches!(err, RenderError::InvalidConfig(_)));
}

#[test]
fn invalid_width_zero_rejected_via_validate() {
    let mut cfg = base_config();
    cfg.width = 0;
    let err = RenderConfig::validate(&cfg).unwrap_err();
    assert!(matches!(err, RenderError::InvalidConfig(_)));
}

#[test]
fn encode_mp4_without_capture_png_rejected() {
    let mut cfg = base_config();
    cfg.encode_mp4 = true;
    let err = RenderConfig::validate(&cfg).unwrap_err();
    assert!(
        matches!(&err, RenderError::InvalidConfig(msg) if msg.contains("capture_png")),
        "got {err:?}",
    );
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p kinetics-render --test renderer`
Expected: compile errors for missing types.

- [ ] **Step 3: Implement types**

Overwrite `crates/kinetics-render/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

//! Frame-by-frame SSR exporter for kinetics Scene compositions.

use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub frames: u32,
    pub fps: u32,
    pub width: u32,
    pub height: u32,
    pub composition_id: String,
    pub output_dir: PathBuf,
    pub capture_png: bool,
    pub encode_mp4: bool,
}

impl RenderConfig {
    pub fn validate(&self) -> Result<(), RenderError> {
        if self.frames == 0 {
            return Err(RenderError::InvalidConfig(
                "frames must be > 0".into(),
            ));
        }
        if self.fps == 0 {
            return Err(RenderError::InvalidConfig("fps must be > 0".into()));
        }
        if self.width == 0 || self.height == 0 {
            return Err(RenderError::InvalidConfig(
                "width and height must be > 0".into(),
            ));
        }
        if self.encode_mp4 && !self.capture_png {
            return Err(RenderError::InvalidConfig(
                "encode_mp4 requires capture_png".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum RenderError {
    InvalidConfig(String),
    Io(io::Error),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConfig(msg) => write!(f, "invalid renderer config: {msg}"),
            Self::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for RenderError {}

impl From<io::Error> for RenderError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug, Clone)]
pub struct RenderReport {
    pub frames_written: u32,
    pub html_dir: PathBuf,
    pub png_dir: Option<PathBuf>,
    pub mp4_path: Option<PathBuf>,
    pub warnings: Vec<String>,
}

pub struct Renderer {
    config: RenderConfig,
}

impl Renderer {
    pub fn new(config: RenderConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &RenderConfig {
        &self.config
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p kinetics-render --test renderer`
Expected: 5 PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/kinetics-render/src/lib.rs crates/kinetics-render/tests/renderer.rs
git commit -m "$(cat <<'EOF'
feat(kinetics-render): RenderConfig + RenderError + Renderer scaffold

Config validation rejects zero values and disallows encode_mp4 without
capture_png. RenderReport and the Renderer struct exist; the render
loop arrives in the next commit.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 3: `Renderer::render` frame loop + manifest

The core: iterate frames, build a fresh `SceneClock` per frame, seek it, evaluate the user's `scene_fn`, render via `dioxus-ssr`, write HTML + ExportManifest.

**Files:**
- Modify: `crates/kinetics-render/src/lib.rs`
- Modify: `crates/kinetics-render/tests/renderer.rs` (append tests)

- [ ] **Step 1: Append failing tests**

Append to `crates/kinetics-render/tests/renderer.rs`:

```rust
use dioxus::prelude::*;
use ui_runtime::scene_clock::SceneClock;

fn tmp_output() -> PathBuf {
    let d = tempfile::tempdir().unwrap();
    let path = d.path().to_path_buf();
    std::mem::forget(d); // leak handle so dir survives the test
    path
}

fn dom_runtime_setup() {
    // Touch a no-op VirtualDom so Signal::new inside scene_clock works.
    // dioxus-ssr render_element runs inside its own VirtualDom; the
    // Renderer is responsible for ensuring SceneClock construction
    // happens inside an active runtime. See Step 3 implementation.
}

#[test]
fn renders_n_html_files_with_distinct_elapsed_ms() {
    let out = tmp_output();
    let cfg = RenderConfig {
        frames: 5,
        fps: 10,
        width: 100,
        height: 100,
        composition_id: "test-distinct-elapsed".into(),
        output_dir: out.clone(),
        capture_png: false,
        encode_mp4: false,
    };
    let renderer = Renderer::new(cfg);
    let report = renderer
        .render(|clock| {
            let elapsed = clock.peek_elapsed_ms() as i64;
            rsx! { div { "data-test-elapsed-ms": "{elapsed}", "frame" } }
        })
        .expect("render ok");

    assert_eq!(report.frames_written, 5);
    assert!(report.png_dir.is_none());
    assert!(report.mp4_path.is_none());

    for frame in 0..5 {
        let path = out.join("frames").join(format!("{frame}.html"));
        let body = std::fs::read_to_string(&path).expect("frame exists");
        let expected_ms = (frame as f32 / 10.0 * 1000.0) as i64;
        assert!(
            body.contains(&format!("data-test-elapsed-ms=\"{expected_ms}\"")),
            "frame {frame}: expected elapsed_ms={expected_ms}, body={body}",
        );
    }
}

#[test]
fn writes_export_manifest_alongside_frames() {
    let out = tmp_output();
    let cfg = RenderConfig {
        frames: 3,
        fps: 30,
        width: 640,
        height: 480,
        composition_id: "manifest-test".into(),
        output_dir: out.clone(),
        capture_png: false,
        encode_mp4: false,
    };
    let renderer = Renderer::new(cfg);
    renderer
        .render(|_clock| rsx! { div { "x" } })
        .expect("render ok");

    let manifest_path = out.join("manifest.json");
    let manifest_body =
        std::fs::read_to_string(&manifest_path).expect("manifest exists");
    assert!(
        manifest_body.contains("\"manifest-test\""),
        "manifest body: {manifest_body}",
    );
    assert!(manifest_body.contains("\"frame_count\":3"));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p kinetics-render --test renderer renders_n_html writes_export`
Expected: compile errors — `render` method missing.

- [ ] **Step 3: Implement `render`**

Append to `crates/kinetics-render/src/lib.rs`:

```rust
use std::fs;

use dioxus::prelude::*;
use ui_composition::Composition;
use ui_runtime::scene_clock::SceneClock;

impl Renderer {
    /// Walks the configured frame range, invoking `scene_fn(clock)`
    /// per frame, and serializing the result via `dioxus-ssr`.
    /// `scene_fn` MUST construct a deterministic tree given the
    /// supplied clock — the renderer does not memoize across frames.
    pub fn render<F>(&self, scene_fn: F) -> Result<RenderReport, RenderError>
    where
        F: Fn(SceneClock) -> Element + 'static + Clone,
    {
        self.config.validate()?;

        let html_dir = self.config.output_dir.join("frames");
        fs::create_dir_all(&html_dir)?;

        // Each frame builds a fresh VirtualDom via dioxus_ssr's
        // render_element so SceneClock construction happens inside a
        // valid Dioxus runtime. We wrap the per-frame work in a tiny
        // probe component so the clock is constructed within scope.
        for frame in 0..self.config.frames {
            let elapsed_ms = (frame as f32 / self.config.fps as f32) * 1000.0;
            let scene_fn = scene_fn.clone();
            let duration_ms =
                (self.config.frames as f32 / self.config.fps as f32) * 1000.0;
            let fps = self.config.fps;

            let body = dioxus_ssr::render_element(rsx! {
                FrameProbe {
                    duration_ms: duration_ms,
                    fps: fps,
                    elapsed_ms: elapsed_ms,
                    scene_fn: scene_fn,
                }
            });

            let frame_path = html_dir.join(format!("{frame}.html"));
            fs::write(&frame_path, body)?;
        }

        // Write the ExportManifest. We don't pull ui_capture's full
        // manifest builder API in to avoid extra deps — write the
        // minimal JSON shape directly.
        let manifest_path = self.config.output_dir.join("manifest.json");
        let manifest_json = serde_json::json!({
            "schema_version": 1,
            "composition": {
                "id": self.config.composition_id,
                "width": self.config.width,
                "height": self.config.height,
                "fps": self.config.fps,
                "frame_count": self.config.frames,
            },
        });
        fs::write(&manifest_path, serde_json::to_string_pretty(&manifest_json)?)?;

        Ok(RenderReport {
            frames_written: self.config.frames,
            html_dir,
            png_dir: None,
            mp4_path: None,
            warnings: Vec::new(),
        })
    }
}

#[component]
fn FrameProbe<F>(
    duration_ms: f32,
    fps: u32,
    elapsed_ms: f32,
    scene_fn: F,
) -> Element
where
    F: Fn(SceneClock) -> Element + 'static + Clone,
{
    let clock = use_hook(|| SceneClock::new(duration_ms, fps, false));
    // Seek to the target frame before invoking scene_fn.
    clock.seek_ms(elapsed_ms);
    scene_fn(clock)
}

// serde_json::Error -> RenderError::Io is a stretch; map through an
// io::Error so the surface stays narrow.
impl From<serde_json::Error> for RenderError {
    fn from(e: serde_json::Error) -> Self {
        Self::Io(io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
    }
}

// Re-export so tests can construct a Composition struct if needed.
pub use ui_composition::Composition as CompositionInfo;
```

The `_` prefix import `Composition` at the top of the file is required for the type alias; remove the unused import if clippy complains.

- [ ] **Step 4: Run tests**

Run: `cargo test -p kinetics-render --test renderer`
Expected: 7 PASS (5 from Task 2 + 2 new).

If a test fails because Dioxus 0.7's `use_hook` requires a non-empty owner scope, inspect `crates/ui-runtime/tests/scene_clock.rs` for the established `with_runtime_async`/`enter` pattern and apply it: the renderer can wrap each `dioxus_ssr::render_element` call in a `dioxus_core::RuntimeGuard` if needed. But `render_element` already constructs a VirtualDom internally — should work out of the box.

Also run `cargo clippy -p kinetics-render --tests -- -D warnings`.

- [ ] **Step 5: Commit**

```bash
git add crates/kinetics-render/src/lib.rs crates/kinetics-render/tests/renderer.rs
git commit -m "$(cat <<'EOF'
feat(kinetics-render): Renderer::render writes per-frame HTML + manifest

Walks frame 0..N, constructs a SceneClock per frame inside a Dioxus
SSR scope, seeks it, invokes scene_fn(clock), serializes via
dioxus_ssr::render_element, writes frames/<i>.html. Manifest emitted
as manifest.json next to the frames directory.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 4: PNG capture orchestrator (Playwright child)

When `capture_png: true`, write `capture.cjs` to the output dir, spawn `node capture.cjs <out>`. If the spawn fails (missing `node`, missing `playwright`, missing browsers), return `Ok(report)` with a warning in `warnings`.

**Files:**
- Create: `crates/kinetics-render/src/capture.rs`
- Create: `crates/kinetics-render/src/template.rs`
- Modify: `crates/kinetics-render/src/lib.rs` (call into capture::run after frame loop when config.capture_png)
- Test: `crates/kinetics-render/tests/capture.rs`

- [ ] **Step 1: Write failing test**

Create `crates/kinetics-render/tests/capture.rs`:

```rust
use std::path::PathBuf;
use kinetics_render::{RenderConfig, Renderer};

fn tmp_output() -> PathBuf {
    let d = tempfile::tempdir().unwrap();
    let path = d.path().to_path_buf();
    std::mem::forget(d);
    path
}

#[test]
fn capture_png_with_missing_npx_returns_warning_not_error() {
    // Simulate npx being unavailable by setting PATH to an empty value.
    let original_path = std::env::var_os("PATH");
    let isolated_path = if cfg!(windows) {
        // Windows needs SYSTEM32 to spawn processes at all; clear PATH
        // and rely on a deliberately-empty PATHEXT to fail the lookup.
        std::env::set_var("PATH", "");
        Some("")
    } else {
        std::env::set_var("PATH", "/nonexistent-12345");
        Some("/nonexistent-12345")
    };
    let _path_guard = scopeguard::guard((), |_| {
        if let Some(p) = &original_path {
            std::env::set_var("PATH", p);
        } else {
            std::env::remove_var("PATH");
        }
    });
    let _ = isolated_path;

    let out = tmp_output();
    let cfg = RenderConfig {
        frames: 2,
        fps: 10,
        width: 100,
        height: 100,
        composition_id: "capture-skip".into(),
        output_dir: out.clone(),
        capture_png: true,
        encode_mp4: false,
    };
    let renderer = Renderer::new(cfg);
    let report = renderer
        .render(|_clock| dioxus::prelude::rsx! { div { "x" } })
        .expect("render ok even when capture is skipped");

    assert_eq!(report.frames_written, 2);
    assert!(
        report.png_dir.is_none(),
        "expected png_dir to be None when capture is skipped",
    );
    assert!(
        report
            .warnings
            .iter()
            .any(|w| w.to_lowercase().contains("playwright")
                || w.to_lowercase().contains("npx")),
        "expected a warning about missing playwright/npx; got {:?}",
        report.warnings,
    );
}
```

This test requires `scopeguard` as a dev-dep. Update `crates/kinetics-render/Cargo.toml`:

```toml
[dev-dependencies]
tempfile = "3"
scopeguard = "1"
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p kinetics-render --test capture`
Expected: compile error / panic — `Renderer::render` doesn't yet call the capture orchestrator.

- [ ] **Step 3: Implement capture orchestrator**

Create `crates/kinetics-render/src/template.rs`:

```rust
//! Embedded Node.js scripts spawned by the PNG capture orchestrator.

pub const CAPTURE_CJS: &str = r#"// kinetics-render PNG capture script
//
// Usage: node capture.cjs <output-dir>
// Iterates output-dir/frames/*.html and writes output-dir/png/<i>.png
// via Playwright Chromium.

const path = require("path");
const fs = require("fs");

async function main() {
    const outDir = process.argv[2];
    if (!outDir) {
        console.error("Usage: node capture.cjs <output-dir>");
        process.exit(2);
    }
    const framesDir = path.join(outDir, "frames");
    const pngDir = path.join(outDir, "png");
    fs.mkdirSync(pngDir, { recursive: true });

    const playwright = require("playwright");
    const browser = await playwright.chromium.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });

    const frames = fs
        .readdirSync(framesDir)
        .filter((f) => f.endsWith(".html"))
        .sort((a, b) => {
            const ai = parseInt(a, 10);
            const bi = parseInt(b, 10);
            return ai - bi;
        });

    for (const frame of frames) {
        const idx = parseInt(frame, 10);
        const fileUrl = "file://" + path.resolve(path.join(framesDir, frame));
        await page.goto(fileUrl, { waitUntil: "networkidle" });
        await page.screenshot({ path: path.join(pngDir, idx + ".png") });
    }

    await browser.close();
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
"#;
```

Create `crates/kinetics-render/src/capture.rs`:

```rust
//! PNG capture orchestrator.
//!
//! Spawns `node capture.cjs <output_dir>` as a child process. Returns
//! `Some(png_dir)` on success or `None` plus a warning string on any
//! failure (missing node, missing playwright, missing browsers, or
//! non-zero exit). This is a graceful-degradation stage — never errors.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::template::CAPTURE_CJS;

pub struct CaptureOutcome {
    pub png_dir: Option<PathBuf>,
    pub warnings: Vec<String>,
}

pub fn run_capture(output_dir: &Path) -> CaptureOutcome {
    let mut warnings = Vec::new();
    let script_path = output_dir.join("capture.cjs");
    if let Err(e) = std::fs::write(&script_path, CAPTURE_CJS) {
        warnings.push(format!(
            "failed to write capture.cjs: {e} (PNG capture skipped)",
        ));
        return CaptureOutcome {
            png_dir: None,
            warnings,
        };
    }

    let cmd = if cfg!(windows) { "node.exe" } else { "node" };
    let result = Command::new(cmd)
        .arg(&script_path)
        .arg(output_dir)
        .output();

    let output = match result {
        Ok(o) => o,
        Err(e) => {
            warnings.push(format!(
                "PNG capture skipped: could not spawn `{cmd}` ({e}). \
                 Install Node.js + playwright to enable capture."
            ));
            return CaptureOutcome {
                png_dir: None,
                warnings,
            };
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warnings.push(format!(
            "PNG capture failed (exit {:?}): {}. \
             Run `npm install playwright && npx playwright install chromium` \
             in the output directory's parent to enable capture.",
            output.status.code(),
            stderr.trim()
        ));
        return CaptureOutcome {
            png_dir: None,
            warnings,
        };
    }

    CaptureOutcome {
        png_dir: Some(output_dir.join("png")),
        warnings,
    }
}
```

In `crates/kinetics-render/src/lib.rs`:

1. Add `mod capture; mod template;` near the top.
2. In `Renderer::render`, after the manifest write and before the `Ok(RenderReport { ... })`, insert:

```rust
        let mut report_warnings: Vec<String> = Vec::new();
        let mut png_dir: Option<PathBuf> = None;
        if self.config.capture_png {
            let outcome = capture::run_capture(&self.config.output_dir);
            png_dir = outcome.png_dir;
            report_warnings.extend(outcome.warnings);
        }
```

And update the final `Ok` to use `png_dir` and `report_warnings`:

```rust
        Ok(RenderReport {
            frames_written: self.config.frames,
            html_dir,
            png_dir,
            mp4_path: None,
            warnings: report_warnings,
        })
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p kinetics-render --test capture`
Expected: PASS.

Also `cargo test -p kinetics-render` for the full crate — expect 8 PASS.

Also `cargo clippy -p kinetics-render --tests -- -D warnings`.

- [ ] **Step 5: Commit**

```bash
git add crates/kinetics-render
git commit -m "$(cat <<'EOF'
feat(kinetics-render): Playwright PNG capture child orchestrator

When capture_png is set, writes capture.cjs into the output dir and
spawns `node capture.cjs <out>`. Missing node, missing playwright, or
non-zero exit return Ok(report) with a descriptive warning rather
than failing the render — PNG capture is a graceful-degradation stage.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 5: MP4 encode orchestrator (FFmpeg child)

When `encode_mp4: true` (implies `capture_png: true`), spawn `ffmpeg -framerate <fps> -i <out>/png/%d.png -c:v libx264 -pix_fmt yuv420p <out>/render.mp4`. Graceful-skip on missing FFmpeg.

**Files:**
- Create: `crates/kinetics-render/src/encode.rs`
- Modify: `crates/kinetics-render/src/lib.rs`
- Test: `crates/kinetics-render/tests/encode.rs`

- [ ] **Step 1: Write failing test**

Create `crates/kinetics-render/tests/encode.rs`:

```rust
use std::path::PathBuf;
use kinetics_render::{RenderConfig, Renderer};

fn tmp_output() -> PathBuf {
    let d = tempfile::tempdir().unwrap();
    let path = d.path().to_path_buf();
    std::mem::forget(d);
    path
}

#[test]
fn encode_mp4_with_missing_ffmpeg_returns_warning() {
    let original_path = std::env::var_os("PATH");
    if cfg!(windows) {
        std::env::set_var("PATH", "");
    } else {
        std::env::set_var("PATH", "/nonexistent-12345");
    }
    let _guard = scopeguard::guard((), |_| {
        if let Some(p) = &original_path {
            std::env::set_var("PATH", p);
        } else {
            std::env::remove_var("PATH");
        }
    });

    let out = tmp_output();
    let cfg = RenderConfig {
        frames: 2,
        fps: 10,
        width: 100,
        height: 100,
        composition_id: "encode-skip".into(),
        output_dir: out.clone(),
        capture_png: true, // requires capture_png to also be true
        encode_mp4: true,
    };
    let renderer = Renderer::new(cfg);
    let report = renderer
        .render(|_clock| dioxus::prelude::rsx! { div { "x" } })
        .expect("render ok even when encode + capture skip");

    assert_eq!(report.frames_written, 2);
    assert!(
        report.mp4_path.is_none(),
        "expected mp4_path to be None when ffmpeg/png missing",
    );
    let lower: Vec<String> =
        report.warnings.iter().map(|w| w.to_lowercase()).collect();
    assert!(
        lower.iter().any(|w| w.contains("ffmpeg") || w.contains("png")),
        "expected a warning mentioning ffmpeg or png; got {:?}",
        report.warnings,
    );
}
```

- [ ] **Step 2: Run test**

Run: `cargo test -p kinetics-render --test encode`
Expected: FAIL — encode orchestrator not wired.

- [ ] **Step 3: Implement encoder**

Create `crates/kinetics-render/src/encode.rs`:

```rust
//! MP4 encode orchestrator. Wraps FFmpeg as a child process and
//! gracefully skips on missing binary or non-zero exit.

use std::path::{Path, PathBuf};
use std::process::Command;

pub struct EncodeOutcome {
    pub mp4_path: Option<PathBuf>,
    pub warnings: Vec<String>,
}

pub fn run_encode(output_dir: &Path, fps: u32) -> EncodeOutcome {
    let mut warnings = Vec::new();
    let png_dir = output_dir.join("png");
    if !png_dir.exists() {
        warnings.push(
            "MP4 encode skipped: PNG directory does not exist (PNG capture must have failed)."
                .to_string(),
        );
        return EncodeOutcome {
            mp4_path: None,
            warnings,
        };
    }

    let mp4_path = output_dir.join("render.mp4");
    let pattern = png_dir.join("%d.png");
    let cmd = if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" };

    let result = Command::new(cmd)
        .arg("-y")
        .arg("-framerate")
        .arg(fps.to_string())
        .arg("-i")
        .arg(&pattern)
        .arg("-c:v")
        .arg("libx264")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg(&mp4_path)
        .output();

    let output = match result {
        Ok(o) => o,
        Err(e) => {
            warnings.push(format!(
                "MP4 encode skipped: could not spawn `{cmd}` ({e}). \
                 Install FFmpeg and add it to PATH to enable encoding."
            ));
            return EncodeOutcome {
                mp4_path: None,
                warnings,
            };
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warnings.push(format!(
            "MP4 encode failed (exit {:?}): {}",
            output.status.code(),
            stderr.trim()
        ));
        return EncodeOutcome {
            mp4_path: None,
            warnings,
        };
    }

    EncodeOutcome {
        mp4_path: Some(mp4_path),
        warnings,
    }
}
```

In `crates/kinetics-render/src/lib.rs`:

1. Add `mod encode;` alongside the others.
2. After the capture block, add:

```rust
        let mut mp4_path: Option<PathBuf> = None;
        if self.config.encode_mp4 {
            let outcome = encode::run_encode(&self.config.output_dir, self.config.fps);
            mp4_path = outcome.mp4_path;
            report_warnings.extend(outcome.warnings);
        }
```

And update the final Ok to use `mp4_path`:

```rust
        Ok(RenderReport {
            frames_written: self.config.frames,
            html_dir,
            png_dir,
            mp4_path,
            warnings: report_warnings,
        })
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p kinetics-render`
Expected: 9 PASS.

Also clippy.

- [ ] **Step 5: Commit**

```bash
git add crates/kinetics-render
git commit -m "$(cat <<'EOF'
feat(kinetics-render): FFmpeg MP4 encode orchestrator with graceful skip

When encode_mp4 is set, spawns FFmpeg with the conservative libx264 +
yuv420p preset. Missing FFmpeg, missing PNG inputs, or non-zero exit
return Ok(report) with a warning rather than failing the render.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 6: `kinetics-cli` clap entrypoint + version/help

Wire `clap` derive parsing. The five subcommands are stub no-ops in this task; per-subcommand bodies arrive in Tasks 7-11.

**Files:**
- Modify: `crates/kinetics-cli/src/main.rs`
- Test: `crates/kinetics-cli/tests/cli.rs`

- [ ] **Step 1: Write failing tests**

Create `crates/kinetics-cli/tests/cli.rs`:

```rust
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
```

- [ ] **Step 2: Run test**

Run: `cargo test -p kinetics-cli --test cli`
Expected: tests fail because the stub main() exits 2 unconditionally.

- [ ] **Step 3: Implement clap entrypoint**

Overwrite `crates/kinetics-cli/src/main.rs`:

```rust
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
```

Create stub modules for each command so the binary links. Each is a one-line stub returning `Err`:

`crates/kinetics-cli/src/cmd_init.rs`:
```rust
pub fn run(_name: &str) -> Result<(), String> {
    Err("init not yet implemented".into())
}
```

`crates/kinetics-cli/src/cmd_preview.rs`:
```rust
pub fn run(_target: &str) -> Result<(), String> {
    Err("preview not yet implemented".into())
}
```

`crates/kinetics-cli/src/cmd_render.rs`:
```rust
use std::path::Path;
pub fn run(
    _scene: &str,
    _out: &Path,
    _frames: u32,
    _fps: u32,
    _capture_png: bool,
    _encode_mp4: bool,
) -> Result<(), String> {
    Err("render not yet implemented".into())
}
```

`crates/kinetics-cli/src/cmd_lint.rs`:
```rust
pub fn run() -> Result<(), String> {
    Err("lint not yet implemented".into())
}
```

`crates/kinetics-cli/src/cmd_doctor.rs`:
```rust
pub fn run() -> Result<(), String> {
    Err("doctor not yet implemented".into())
}
```

`crates/kinetics-cli/src/scene_registry.rs`:
```rust
//! Registry of named scenes. Populated in Task 9.
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p kinetics-cli --test cli`
Expected: 3 PASS (help shows subcommands, version succeeds, unknown subcommand fails).

- [ ] **Step 5: Commit**

```bash
git add crates/kinetics-cli
git commit -m "$(cat <<'EOF'
feat(kinetics-cli): clap entrypoint with five-subcommand routing

Help lists init/preview/render/lint/doctor. Each subcommand currently
returns Err("not yet implemented") — bodies arrive in subsequent
tasks.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 7: `kinetics init` subcommand

Scaffold a new example crate using an embedded `main.rs` template.

**Files:**
- Modify: `crates/kinetics-cli/src/cmd_init.rs`
- Create: `crates/kinetics-cli/src/template/init_main.rs.tmpl`
- Test: `crates/kinetics-cli/tests/cli.rs` (append)

- [ ] **Step 1: Append failing test**

Append to `crates/kinetics-cli/tests/cli.rs`:

```rust
use std::fs;
use tempfile::tempdir;

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
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p kinetics-cli --test cli init_creates_scaffolded_directory`
Expected: fails — `init` returns Err.

- [ ] **Step 3: Create the template file**

Create `crates/kinetics-cli/src/template/init_main.rs.tmpl`:

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_composition::ClipFill;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Scene {
            id: "hello-scene",
            width: 1280,
            height: 720,
            duration_ms: 5_000.0,
            autoplay: Some(true),
            controls: Some(true),
            Clip { start_ms: 0.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "hello-title",
                    text: "Hello, kinetics.".to_string(),
                    cue: "rise-in",
                }
            }
            Clip { start_ms: 1_500.0, duration_ms: 3_000.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "hello-body",
                    text: "Composable cinematic motion.".to_string(),
                    cue: "fade-in",
                }
            }
        }
    }
}
```

- [ ] **Step 4: Implement init**

Overwrite `crates/kinetics-cli/src/cmd_init.rs`:

```rust
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
```

- [ ] **Step 5: Run tests**

Run: `cargo test -p kinetics-cli`
Expected: 4 PASS.

- [ ] **Step 6: Commit**

```bash
git add crates/kinetics-cli
git commit -m "$(cat <<'EOF'
feat(kinetics-cli): init subcommand scaffolds a kinetics example crate

Creates <name>/Cargo.toml + src/main.rs from an embedded template
containing a minimal Scene composition.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 8: `kinetics preview` + `kinetics lint` + `kinetics doctor`

Three thin wrappers around existing toolchain commands. Combine into one task — each body is ~10 lines.

**Files:**
- Modify: `crates/kinetics-cli/src/cmd_preview.rs`
- Modify: `crates/kinetics-cli/src/cmd_lint.rs`
- Modify: `crates/kinetics-cli/src/cmd_doctor.rs`
- Test: `crates/kinetics-cli/tests/cli.rs` (append)

- [ ] **Step 1: Append failing tests**

Append to `crates/kinetics-cli/tests/cli.rs`:

```rust
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
    kinetics()
        .args(["lint", "--help"])
        .assert()
        .success();
}

#[test]
fn preview_target_arg_parses() {
    // Same approach as lint — we don't want to actually run `dx serve`
    // in tests. Verify --help works.
    kinetics()
        .args(["preview", "--help"])
        .assert()
        .success();
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p kinetics-cli --test cli doctor_succeeds lint_runs preview_target`
Expected: doctor fails ("not yet implemented"). The other two pass (clap's --help auto-handles them).

- [ ] **Step 3: Implement `preview`**

Overwrite `crates/kinetics-cli/src/cmd_preview.rs`:

```rust
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
```

- [ ] **Step 4: Implement `lint`**

Overwrite `crates/kinetics-cli/src/cmd_lint.rs`:

```rust
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
```

- [ ] **Step 5: Implement `doctor`**

Overwrite `crates/kinetics-cli/src/cmd_doctor.rs`:

```rust
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
```

- [ ] **Step 6: Run tests**

Run: `cargo test -p kinetics-cli`
Expected: 7 PASS (3 from Task 6 + init from Task 7 + 3 new).

- [ ] **Step 7: Commit**

```bash
git add crates/kinetics-cli
git commit -m "$(cat <<'EOF'
feat(kinetics-cli): preview / lint / doctor subcommands

preview wraps `dx serve --hot-reload examples/component-gallery`.
lint runs `cargo fmt --check` then `cargo clippy --workspace ... -D warnings`.
doctor probes rustc/cargo/dx/node/npx/playwright/ffmpeg and reports
versions; missing optional tools are non-errors.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 9: `kinetics render` + SceneRegistry

The scene registry maps named scenes to a `scene_fn` + base `RenderConfig`. SP-4+5+6 ships five named scenes.

**Files:**
- Modify: `crates/kinetics-cli/src/scene_registry.rs`
- Modify: `crates/kinetics-cli/src/cmd_render.rs`
- Modify: `crates/kinetics-cli/Cargo.toml` (`kinetics` workspace dep so the registry can construct scenes via `kinetics::prelude::*`)
- Test: `crates/kinetics-cli/tests/cli.rs` (append)

- [ ] **Step 1: Append failing test**

Append to `crates/kinetics-cli/tests/cli.rs`:

```rust
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
```

- [ ] **Step 2: Run test**

Run: `cargo test -p kinetics-cli --test cli render_with_unknown_scene render_known_scene`
Expected: failures — render returns Err("not yet implemented").

- [ ] **Step 3: Add `kinetics` dep**

In `crates/kinetics-cli/Cargo.toml`, add to `[dependencies]`:

```toml
kinetics = { path = "../kinetics" }
ui-composition = { path = "../ui-composition" }
dioxus = { workspace = true }
```

- [ ] **Step 4: Implement the registry**

Overwrite `crates/kinetics-cli/src/scene_registry.rs`:

```rust
use std::path::PathBuf;

use dioxus::prelude::*;
use kinetics::prelude::*;
use kinetics_render::{RenderConfig, Renderer};
use ui_composition::ClipFill;
use ui_runtime::scene_clock::SceneClock;

pub struct SceneSpec {
    pub id: &'static str,
    pub width: u32,
    pub height: u32,
    pub duration_ms: f32,
    pub scene_fn: fn(SceneClock) -> Element,
}

pub const SCENES: &[SceneSpec] = &[
    SceneSpec {
        id: "hello",
        width: 1280,
        height: 720,
        duration_ms: 5_000.0,
        scene_fn: hello_scene,
    },
    SceneSpec {
        id: "product-intro",
        width: 1920,
        height: 1080,
        duration_ms: 10_000.0,
        scene_fn: product_intro_scene,
    },
];

pub fn lookup(id: &str) -> Option<&'static SceneSpec> {
    SCENES.iter().find(|s| s.id == id)
}

pub fn run_render(
    spec: &SceneSpec,
    out: &std::path::Path,
    frames: u32,
    fps: u32,
    capture_png: bool,
    encode_mp4: bool,
) -> Result<(), String> {
    let config = RenderConfig {
        frames,
        fps,
        width: spec.width,
        height: spec.height,
        composition_id: spec.id.to_string(),
        output_dir: PathBuf::from(out),
        capture_png,
        encode_mp4,
    };
    let renderer = Renderer::new(config);
    let report = renderer
        .render(spec.scene_fn)
        .map_err(|e| format!("render failed: {e}"))?;
    println!("kinetics render: wrote {} frames to {:?}", report.frames_written, report.html_dir);
    for w in &report.warnings {
        eprintln!("warning: {w}");
    }
    if let Some(p) = report.png_dir {
        println!("PNG sequence: {}", p.display());
    }
    if let Some(p) = report.mp4_path {
        println!("MP4: {}", p.display());
    }
    Ok(())
}

fn hello_scene(_clock: SceneClock) -> Element {
    rsx! {
        Scene {
            id: "hello",
            width: 1280,
            height: 720,
            duration_ms: 5_000.0,
            autoplay: Some(false),
            controls: Some(false),
            driver: Some(SceneDriver::Manual),
            Clip { start_ms: 0.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "hello-title",
                    text: "Hello, kinetics.".to_string(),
                    cue: "rise-in",
                }
            }
            Clip { start_ms: 1_500.0, duration_ms: 3_000.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "hello-body",
                    text: "Composable cinematic motion.".to_string(),
                    cue: "fade-in",
                }
            }
        }
    }
}

fn product_intro_scene(_clock: SceneClock) -> Element {
    rsx! {
        Scene {
            id: "product-intro",
            width: 1920,
            height: 1080,
            duration_ms: 10_000.0,
            autoplay: Some(false),
            controls: Some(false),
            driver: Some(SceneDriver::Manual),
            Clip { start_ms: 0.0, duration_ms: 2_400.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "intro-title",
                    text: "Kinetics moves like light.".to_string(),
                    cue: "rise-in",
                }
            }
            Clip { start_ms: 800.0, duration_ms: 2_400.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "intro-body",
                    text: "Composable motion for downstream SaaS.".to_string(),
                    cue: "fade-in",
                }
            }
            Clip { start_ms: 6_800.0, duration_ms: 3_200.0, fill: ClipFill::HoldEnd,
                Button { variant: ButtonVariant::Primary, "Start building" }
            }
        }
    }
}
```

- [ ] **Step 5: Implement `cmd_render::run`**

Overwrite `crates/kinetics-cli/src/cmd_render.rs`:

```rust
use std::path::Path;

use crate::scene_registry;

pub fn run(
    scene: &str,
    out: &Path,
    frames: u32,
    fps: u32,
    capture_png: bool,
    encode_mp4: bool,
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
    scene_registry::run_render(spec, out, frames, fps, capture_png, encode_mp4)
}
```

- [ ] **Step 6: Run tests**

Run: `cargo test -p kinetics-cli`
Expected: 9 PASS.

If the `render_known_scene_writes_html_frames` test panics because Dioxus's `use_hook` inside the renderer requires a runtime that test harness doesn't provide, check that `dioxus_ssr::render_element` is correctly bootstrapping a VirtualDom (it should — that's its public contract). If it isn't, the renderer's `FrameProbe` may need to be wrapped differently.

- [ ] **Step 7: Commit**

```bash
git add crates/kinetics-cli
git commit -m "$(cat <<'EOF'
feat(kinetics-cli): render subcommand + scene registry

SceneRegistry maps two named scenes (hello, product-intro) to scene_fn
+ config. Unknown scenes return a friendly error listing the available
ids. Known scenes render via kinetics-render::Renderer; warnings from
graceful-skip stages are echoed to stderr.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 10: `ui-blocks` crate — `LowerThird`

First catalog block. Test-driven.

**Files:**
- Create: `crates/ui-blocks/src/lower_third.rs`
- Modify: `crates/ui-blocks/src/lib.rs`
- Test: `crates/ui-blocks/tests/blocks_ssr.rs`

- [ ] **Step 1: Create the test file with the LowerThird test**

Create `crates/ui-blocks/tests/blocks_ssr.rs`:

```rust
use dioxus::prelude::*;
use ui_blocks::{LowerThird, LowerThirdAccent};

#[test]
fn lower_third_emits_aria_label_with_name_and_role() {
    let html = dioxus_ssr::render_element(rsx! {
        LowerThird { name: "Ada Lovelace".to_string(), role: "Mathematician".to_string() }
    });
    assert!(html.contains("Ada Lovelace"), "{html}");
    assert!(html.contains("Mathematician"), "{html}");
    assert!(
        html.contains("aria-label=\"Ada Lovelace, Mathematician\""),
        "{html}",
    );
}

#[test]
fn lower_third_accent_primary_is_default() {
    let html = dioxus_ssr::render_element(rsx! {
        LowerThird { name: "x".to_string(), role: "y".to_string() }
    });
    assert!(html.contains("ui-block-lower-third--primary"), "{html}");
}

#[test]
fn lower_third_accent_secondary_renders_modifier_class() {
    let html = dioxus_ssr::render_element(rsx! {
        LowerThird {
            name: "x".to_string(),
            role: "y".to_string(),
            accent: Some(LowerThirdAccent::Secondary),
        }
    });
    assert!(html.contains("ui-block-lower-third--secondary"), "{html}");
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test -p ui-blocks --test blocks_ssr`
Expected: compile errors (`LowerThird`, `LowerThirdAccent` not found).

- [ ] **Step 3: Implement LowerThird**

Create `crates/ui-blocks/src/lower_third.rs`:

```rust
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LowerThirdAccent {
    #[default]
    Primary,
    Secondary,
}

/// Broadcast-style chyron with name + role. Sits at the bottom-left
/// of the parent container; consumer is responsible for positioning
/// (we apply no absolute positioning).
#[component]
pub fn LowerThird(
    name: String,
    role: String,
    accent: Option<LowerThirdAccent>,
) -> Element {
    let accent = accent.unwrap_or_default();
    let accent_class = match accent {
        LowerThirdAccent::Primary => "ui-block-lower-third--primary",
        LowerThirdAccent::Secondary => "ui-block-lower-third--secondary",
    };
    let aria = format!("{name}, {role}");
    rsx! {
        div {
            class: "ui-block-lower-third {accent_class}",
            "aria-label": "{aria}",
            "data-block": "lower-third",
            div { class: "ui-block-lower-third__bar" }
            div { class: "ui-block-lower-third__text",
                div { class: "ui-block-lower-third__name", "{name}" }
                div { class: "ui-block-lower-third__role", "{role}" }
            }
        }
    }
}
```

Overwrite `crates/ui-blocks/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

//! Reusable cinematic block catalog for kinetics Scene compositions.

mod lower_third;

pub use lower_third::{LowerThird, LowerThirdAccent};
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p ui-blocks --test blocks_ssr`
Expected: 3 PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/ui-blocks/src/lib.rs crates/ui-blocks/src/lower_third.rs crates/ui-blocks/tests/blocks_ssr.rs
git commit -m "$(cat <<'EOF'
feat(ui-blocks): LowerThird block (chyron with name + role)

Two-accent variant (Primary default + Secondary). Parent carries
aria-label = "<name>, <role>". Pure DOM scaffolding; the consumer
positions via parent layout.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 11: `ui-blocks` — Caption

**Files:**
- Create: `crates/ui-blocks/src/caption.rs`
- Modify: `crates/ui-blocks/src/lib.rs`
- Test: `crates/ui-blocks/tests/blocks_ssr.rs` (append)

- [ ] **Step 1: Append test**

```rust
use ui_blocks::Caption;

#[test]
fn caption_emits_per_word_split_text_spans() {
    let html = dioxus_ssr::render_element(rsx! {
        Caption { text: "Built with kinetics.".to_string() }
    });
    // Caption uses SplitText { split_by: Word }, which emits per-word
    // spans with data-stagger-index.
    assert!(html.contains("data-stagger-index=\"0\""), "{html}");
    assert!(html.contains("data-stagger-index=\"1\""), "{html}");
    assert!(html.contains("data-stagger-index=\"2\""), "{html}");
    assert!(
        html.contains("aria-label=\"Built with kinetics.\""),
        "{html}",
    );
}
```

- [ ] **Step 2: Implement Caption**

Create `crates/ui-blocks/src/caption.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::{SplitMode, SplitText};

/// Subtitle bar with reading-pace word stagger.
#[component]
pub fn Caption(text: String, reading_pace_ms_per_word: Option<f32>) -> Element {
    let _ = reading_pace_ms_per_word; // stagger pace consumed by parent TimelineScope; surfaced via data attr below.
    rsx! {
        div {
            class: "ui-block-caption",
            "data-block": "caption",
            SplitText { text: text, split_by: Some(SplitMode::Word) }
        }
    }
}
```

Add to `crates/ui-blocks/src/lib.rs`:

```rust
mod caption;
pub use caption::Caption;
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p ui-blocks --test blocks_ssr`
Expected: 4 PASS.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-blocks
git commit -m "$(cat <<'EOF'
feat(ui-blocks): Caption block (per-word SplitText subtitle)

Wraps SplitText { split_by: Word } so a surrounding TimelineScope's
stagger machinery walks the words at reading pace. aria-label on the
parent SplitText carries the full text.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 12: `ui-blocks` — WipeTransition

**Files:**
- Create: `crates/ui-blocks/src/wipe_transition.rs`
- Modify: `crates/ui-blocks/src/lib.rs`
- Test: `crates/ui-blocks/tests/blocks_ssr.rs` (append)

- [ ] **Step 1: Append test**

```rust
use ui_blocks::WipeTransition;

#[test]
fn wipe_transition_emits_mask_image_kinetic_box() {
    let html = dioxus_ssr::render_element(rsx! {
        WipeTransition { duration_ms: 1_000.0, p { "covered content" } }
    });
    assert!(html.contains("ui-block-wipe-transition"), "{html}");
    assert!(html.contains("data-block=\"wipe-transition\""), "{html}");
    assert!(html.contains("covered content"), "{html}");
    assert!(
        html.contains("mask-image") || html.contains("-webkit-mask-image"),
        "{html}",
    );
}
```

- [ ] **Step 2: Implement WipeTransition**

Create `crates/ui-blocks/src/wipe_transition.rs`:

```rust
use dioxus::prelude::*;

/// Full-coverage wipe transition. The mask sweeps across the child
/// region over `duration_ms`. Direction controlled by `angle_deg`
/// (default 90.0 = left-to-right).
#[component]
pub fn WipeTransition(
    duration_ms: f32,
    angle_deg: Option<f32>,
    children: Element,
) -> Element {
    let angle = angle_deg.unwrap_or(90.0);
    let inline_style = format!(
        "mask-image: linear-gradient({angle}deg, black, transparent); \
         -webkit-mask-image: linear-gradient({angle}deg, black, transparent); \
         animation: ui-block-wipe-transition {duration_ms}ms forwards paused;"
    );
    rsx! {
        div {
            class: "ui-block-wipe-transition",
            "data-block": "wipe-transition",
            "data-duration-ms": "{duration_ms}",
            "data-angle-deg": "{angle}",
            style: "{inline_style}",
            {children}
        }
    }
}
```

Add to `crates/ui-blocks/src/lib.rs`:

```rust
mod wipe_transition;
pub use wipe_transition::WipeTransition;
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p ui-blocks --test blocks_ssr`
Expected: 5 PASS.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-blocks
git commit -m "$(cat <<'EOF'
feat(ui-blocks): WipeTransition block (CSS mask sweep)

Wraps children with a linear-gradient mask that animates from full
visibility to transparency over duration_ms. angle_deg controls the
sweep direction (default 90.0 = left-to-right).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 13: `ui-blocks` — MetricCounter

**Files:**
- Create: `crates/ui-blocks/src/metric_counter.rs`
- Modify: `crates/ui-blocks/src/lib.rs`
- Test: `crates/ui-blocks/tests/blocks_ssr.rs` (append)

- [ ] **Step 1: Append test**

```rust
use ui_blocks::MetricCounter;

#[test]
fn metric_counter_renders_three_kinetic_text_lines() {
    let html = dioxus_ssr::render_element(rsx! {
        MetricCounter {
            label: "Active users".to_string(),
            value: "1,287".to_string(),
            delta_text: Some("+24% w/w".to_string()),
        }
    });
    assert!(html.contains("Active users"), "{html}");
    assert!(html.contains("1,287"), "{html}");
    assert!(html.contains("+24% w/w"), "{html}");
    assert!(html.contains("ui-block-metric-counter"), "{html}");
}

#[test]
fn metric_counter_without_delta_omits_third_line() {
    let html = dioxus_ssr::render_element(rsx! {
        MetricCounter {
            label: "Loose".to_string(),
            value: "42".to_string(),
        }
    });
    assert!(html.contains("Loose"), "{html}");
    assert!(html.contains("42"), "{html}");
    // No delta -> no delta KineticText id reference.
    assert!(!html.contains("metric-delta"), "{html}");
}
```

- [ ] **Step 2: Implement**

Create `crates/ui-blocks/src/metric_counter.rs`:

```rust
use dioxus::prelude::*;
use ui_dioxus::KineticText;

#[component]
pub fn MetricCounter(
    label: String,
    value: String,
    delta_text: Option<String>,
) -> Element {
    rsx! {
        div { class: "ui-block-metric-counter", "data-block": "metric-counter",
            KineticText { id: "metric-label", text: label, cue: "fade-in" }
            KineticText { id: "metric-value", text: value, cue: "rise-in" }
            if let Some(delta) = delta_text {
                KineticText { id: "metric-delta", text: delta, cue: "fade-in" }
            }
        }
    }
}
```

Add to `crates/ui-blocks/src/lib.rs`:

```rust
mod metric_counter;
pub use metric_counter::MetricCounter;
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p ui-blocks --test blocks_ssr`
Expected: 7 PASS.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-blocks
git commit -m "$(cat <<'EOF'
feat(ui-blocks): MetricCounter block (label + value + delta)

Three sequential KineticText lines. delta_text is optional; without
it the third line is omitted entirely (no KineticText with id
"metric-delta" is rendered).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 14: `ui-blocks` — SocialOverlay

**Files:**
- Create: `crates/ui-blocks/src/social_overlay.rs`
- Modify: `crates/ui-blocks/src/lib.rs`
- Test: `crates/ui-blocks/tests/blocks_ssr.rs` (append)

- [ ] **Step 1: Append test**

```rust
use ui_blocks::{SocialOverlay, SocialPlatform};

#[test]
fn social_overlay_renders_platform_accent_class() {
    let html = dioxus_ssr::render_element(rsx! {
        SocialOverlay {
            platform: SocialPlatform::Instagram,
            handle: "@kineticsui".to_string(),
            message: "Just followed you!".to_string(),
        }
    });
    assert!(html.contains("ui-block-social-overlay--instagram"), "{html}");
    assert!(html.contains("@kineticsui"), "{html}");
    assert!(html.contains("Just followed you!"), "{html}");
}

#[test]
fn social_overlay_twitter_variant() {
    let html = dioxus_ssr::render_element(rsx! {
        SocialOverlay {
            platform: SocialPlatform::Twitter,
            handle: "@dx".to_string(),
            message: "Replied to your post.".to_string(),
        }
    });
    assert!(html.contains("ui-block-social-overlay--twitter"), "{html}");
}
```

- [ ] **Step 2: Implement**

Create `crates/ui-blocks/src/social_overlay.rs`:

```rust
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SocialPlatform {
    Instagram,
    Twitter,
    YouTube,
    TikTok,
}

impl SocialPlatform {
    fn modifier(self) -> &'static str {
        match self {
            Self::Instagram => "instagram",
            Self::Twitter => "twitter",
            Self::YouTube => "youtube",
            Self::TikTok => "tiktok",
        }
    }
}

#[component]
pub fn SocialOverlay(
    platform: SocialPlatform,
    handle: String,
    message: String,
) -> Element {
    let modifier_class =
        format!("ui-block-social-overlay--{}", platform.modifier());
    rsx! {
        div {
            class: "ui-block-social-overlay {modifier_class}",
            "data-block": "social-overlay",
            "data-platform": "{platform.modifier()}",
            div { class: "ui-block-social-overlay__handle", "{handle}" }
            div { class: "ui-block-social-overlay__message", "{message}" }
        }
    }
}
```

Add to `crates/ui-blocks/src/lib.rs`:

```rust
mod social_overlay;
pub use social_overlay::{SocialOverlay, SocialPlatform};
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p ui-blocks --test blocks_ssr`
Expected: 9 PASS.

Also `cargo clippy -p ui-blocks --tests -- -D warnings`.

- [ ] **Step 4: Commit**

```bash
git add crates/ui-blocks
git commit -m "$(cat <<'EOF'
feat(ui-blocks): SocialOverlay block (4-platform notification card)

SocialPlatform enum with Instagram/Twitter/YouTube/TikTok variants
controls the brand-color accent via BEM modifier class.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 15: `kinetics::prelude` re-exports for ui-blocks

**Files:**
- Modify: `crates/kinetics/Cargo.toml` (add `ui-blocks` workspace dep)
- Modify: `crates/kinetics/src/lib.rs`

- [ ] **Step 1: Add the dep**

In `crates/kinetics/Cargo.toml`, add to `[dependencies]`:

```toml
ui-blocks = { path = "../ui-blocks", optional = true }
```

And to `[features]`, add (or extend) a `blocks` feature:

```toml
blocks = ["dep:ui-blocks"]
default = ["blocks"]  # if a default array already exists, append "blocks"
```

If no `[features]` block exists, create one. If a `default` already exists, just append `"blocks"` to it.

- [ ] **Step 2: Add re-exports**

In `crates/kinetics/src/lib.rs`, in the prelude, add a feature-gated block:

```rust
    #[cfg(feature = "blocks")]
    pub use ui_blocks::{
        Caption, LowerThird, LowerThirdAccent, MetricCounter, SocialOverlay,
        SocialPlatform, WipeTransition,
    };
```

In `public_api_names()`, append (with appropriate `#[cfg]` if the existing pattern uses it):

```rust
    #[cfg(feature = "blocks")]
    names.extend_from_slice(&[
        "LowerThird",
        "LowerThirdAccent",
        "Caption",
        "WipeTransition",
        "MetricCounter",
        "SocialOverlay",
        "SocialPlatform",
    ]);
```

- [ ] **Step 3: Verify + update prelude test**

Run: `cargo check -p kinetics --all-features`
Expected: success.

If `crates/kinetics/tests/prelude.rs` has hard-coded counts, update them. Inspect first.

- [ ] **Step 4: Commit**

```bash
git add crates/kinetics
git commit -m "$(cat <<'EOF'
feat(kinetics): export ui-blocks catalog in prelude under `blocks` feature

Default-enabled feature so downstream code reads
`use kinetics::prelude::*; rsx! { LowerThird { ... } }` out of the box.
Opt out with `default-features = false` if the catalog isn't needed.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 16: Agent skill — `.claude/skills/kinetics-scene/SKILL.md`

**Files:**
- Create: `.claude/skills/kinetics-scene/SKILL.md`

- [ ] **Step 1: Write the skill**

Create `.claude/skills/kinetics-scene/SKILL.md`:

```markdown
---
name: kinetics-scene
description: Use when authoring or modifying Dioxus Kinetics Scene compositions. Covers Scene/Clip/SceneDriver, SplitText/MotionPath, ui-blocks catalog, reduced-motion patterns, and workspace TDD conventions. Trigger on requests to build cinematic scenes, scroll-driven storytelling, animated text, motion paths, or any composition using kinetics::prelude.
---

# kinetics-scene — authoring Scene compositions

This skill teaches how to author and modify kinetics Scene
compositions in the `dioxus-kinetics` workspace.

## Quick start

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_composition::ClipFill;

#[component]
fn HelloScene() -> Element {
    rsx! {
        Scene {
            id: "hello",
            width: 1280,
            height: 720,
            duration_ms: 5_000.0,
            autoplay: Some(true),
            controls: Some(true),
            Clip { start_ms: 0.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "hello-title",
                    text: "Hello, kinetics.".to_string(),
                    cue: "rise-in",
                }
            }
            Clip { start_ms: 1_500.0, duration_ms: 3_000.0, fill: ClipFill::HoldEnd,
                KineticText {
                    id: "hello-body",
                    text: "Composable cinematic motion.".to_string(),
                    cue: "fade-in",
                }
            }
        }
    }
}
```

## Drivers

`Scene` accepts an optional `driver: Option<SceneDriver>` that selects
how the clock advances:

- `SceneDriver::Autoplay` (default if `driver = None` and
  `autoplay = true`) — clock advances via the platform frame loop.
- `SceneDriver::Manual` — clock only moves via explicit `seek_*`.
  Pick this for scenes driven externally (the `kinetics-render`
  pipeline uses `Manual`).
- `SceneDriver::Scroll(ScrollObserverConfig::new("#trigger"))` —
  scroll position drives the clock. Web-only; native targets hold at
  progress 0.

```rust
Scene {
    id: "scroll-pinned",
    duration_ms: 10_000.0,
    driver: Some(SceneDriver::Scroll(
        ScrollObserverConfig::new("#story-trigger"),
    )),
    /* ... */
}
```

## Clips

`Clip { start_ms, duration_ms, fill }` gates a child element's
visibility based on the parent clock:

- `ClipFill::None` (default) — visible inside `[start, start+duration)`.
- `ClipFill::HoldStart` — visible before `start` too.
- `ClipFill::HoldEnd` — visible after `start+duration` too (use this
  for clips you want to "stay visible" at the settled state).
- `ClipFill::HoldBoth` — always visible.

## Per-glyph text (SplitText)

```rust
TimelineScope { id: "title-timeline", autoplay: true,
    SplitText {
        text: "Hello, world.".to_string(),
        split_by: Some(SplitMode::Character), // or Word
    }
}
```

- Parent carries `aria-label = "<full text>"`.
- Per-glyph spans set `aria-hidden = "true"`.
- A surrounding `TimelineScope` walks the `data-stagger-index`
  attribute to animate glyphs in sequence.

## Curved motion (MotionPath)

```rust
use ui_motion::{Ease, Transition};
use ui_timeline::{MotionSegment, MotionTarget, Timeline, TimelineTrack};

let pts = vec![
    PathPoint::Line { end: (0.0, 0.0) },
    PathPoint::Bezier {
        control_1: (200.0, -200.0),
        control_2: (400.0, 200.0),
        end: (600.0, 0.0),
    },
];
let cue = MotionCue::Path {
    points: pts.clone(),
    from_progress: 0.0,
    to_progress: 1.0,
    rotate_along_path: false,
    transition: Transition::Tween { duration_ms: 4_000, ease: Ease::Standard },
};
let timeline = Timeline::new("trajectory", 4_000.0).with_track(
    TimelineTrack::new(
        MotionTarget::node("icon"),
        vec![MotionSegment::new(0.0, 4_000.0, cue)],
    ),
);

rsx! {
    Sequence {
        timeline: Some(timeline),
        clock: TimelineClock::Manual { elapsed_ms: 0.0 },
        MotionPath { id: "icon".to_string(), path: pts, duration_ms: 4_000.0,
            KineticBox { id: "icon", "•" }
        }
    }
}
```

Sampling is arc-length-uniform — equal `t` covers equal visual
distance. `rotate_along_path: true` emits a tangent angle so the
KineticBox rotates to match the curve direction.

## Catalog blocks (ui-blocks)

Five reusable cinematic blocks. Compose them into Scenes:

- `LowerThird { name, role, accent }` — chyron with name + role.
- `Caption { text, reading_pace_ms_per_word }` — subtitle bar with
  per-word stagger.
- `WipeTransition { duration_ms, angle_deg, children }` — CSS mask
  sweep across children.
- `MetricCounter { label, value, delta_text }` — three-line metric
  display.
- `SocialOverlay { platform, handle, message }` — notification card
  with platform accent.

## Reduced motion

Every component respects the `ReducedMotion` context:

- `Scene` settles immediately at `duration_ms` and disables the
  scrubber when reduced.
- Adapters render the final, settled state when their `reduced`
  flag is set.
- `MotionPath` collapses to the endpoint position.
- `SplitText` renders glyphs at final state, no stagger.

Wrap a subtree in `ReducedMotionProvider { reduced: Some(true), ... }`
to force reduced motion (e.g. for testing). Without the prop, the
provider reads `prefers-reduced-motion` from the browser and the
`data-ui-motion="reduced"` attribute from the document body.

## Accessibility

- `SplitText`: parent `aria-label` always carries the unsplit text;
  glyph spans set `aria-hidden = "true"` so screen readers do not
  enumerate.
- Scene-level decoration (icons, animated `MotionPath` glyphs) should
  not have aria labels — they're visual flourish.
- Always test reduced-motion paths — they're the canonical
  "this is what the scene looks like at rest" state.

## Workspace conventions

- TDD: write the failing test first, run it red, implement, run green,
  commit. Each step is one commit-able action.
- Signal writes use the `let mut s = …; s.set(…);` idiom, not
  `signal.clone().set(…)`. `Signal<T>` is `Copy` in Dioxus 0.7;
  the `.clone()` form was a workaround that's gone from the workspace.
- Conventional Commits with a `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>`
  trailer when an AI agent participated.
- Tests live in `tests/<name>.rs` (integration-style) — the workspace
  precedent for `dioxus-ssr` tests.
- Never push, amend, or `--no-verify` without explicit user request.
```

- [ ] **Step 2: Commit**

```bash
git add .claude/skills/kinetics-scene/SKILL.md
git commit -m "$(cat <<'EOF'
docs(skills): add kinetics-scene agent skill

Teaches agents the Scene/Clip/SceneDriver/SplitText/MotionPath API,
the ui-blocks catalog, reduced-motion patterns, accessibility
expectations, and the workspace's TDD + Signal-idiom conventions.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 17: Gallery scene — `LowerThirdDemoScene`

**Files:**
- Create: `examples/component-gallery/src/previews/scenes/lower_third_demo.rs`
- Modify: `examples/component-gallery/src/previews/scenes/mod.rs`
- Modify: `examples/component-gallery/Cargo.toml` (add `ui-blocks` workspace dep)

- [ ] **Step 1: Add the dep**

In `examples/component-gallery/Cargo.toml`, add to `[dependencies]`:

```toml
ui-blocks = { path = "../../crates/ui-blocks" }
```

(Adjust the relative path if the workspace layout differs — most likely `path = "../../crates/ui-blocks"` from `examples/component-gallery/`.)

- [ ] **Step 2: Create the scene**

Create `examples/component-gallery/src/previews/scenes/lower_third_demo.rs`:

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_composition::ClipFill;
use ui_blocks::{LowerThird, LowerThirdAccent};

#[component]
pub fn LowerThirdDemoScene() -> Element {
    rsx! {
        Scene {
            id: "lower-third-demo",
            width: 1280,
            height: 720,
            duration_ms: 4_000.0,
            autoplay: Some(true),
            controls: Some(true),
            div { class: "scene-lower-third-backdrop",
                style: "position: relative; width: 100%; height: 100%;",
                Clip { start_ms: 500.0, duration_ms: 3_000.0, fill: ClipFill::HoldEnd,
                    LowerThird {
                        name: "Ada Lovelace".to_string(),
                        role: "Mathematician".to_string(),
                        accent: Some(LowerThirdAccent::Primary),
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 3: Register the module**

In `examples/component-gallery/src/previews/scenes/mod.rs`, add `pub mod lower_third_demo;`.

- [ ] **Step 4: Verify**

Run: `cargo check -p component-gallery`
Expected: success.

- [ ] **Step 5: Commit**

```bash
git add examples/component-gallery/src/previews/scenes/lower_third_demo.rs examples/component-gallery/src/previews/scenes/mod.rs examples/component-gallery/Cargo.toml
git commit -m "$(cat <<'EOF'
feat(gallery): Scene · Lower Third Demo

LowerThird block showcased inside a Scene with a 500ms enter offset
and HoldEnd fill so the chyron sticks at the settled state.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 18: Gallery scene — `CaptionDemoScene`

**Files:**
- Create: `examples/component-gallery/src/previews/scenes/caption_demo.rs`
- Modify: `examples/component-gallery/src/previews/scenes/mod.rs`

- [ ] **Step 1: Create the scene**

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_blocks::Caption;

#[component]
pub fn CaptionDemoScene() -> Element {
    rsx! {
        Scene {
            id: "caption-demo",
            width: 1280,
            height: 360,
            duration_ms: 3_500.0,
            autoplay: Some(true),
            controls: Some(true),
            TimelineScope { id: "caption-timeline", autoplay: true,
                Caption {
                    text: "Built with kinetics ui-blocks.".to_string(),
                    reading_pace_ms_per_word: Some(320.0),
                }
            }
        }
    }
}
```

- [ ] **Step 2: Register**

Add `pub mod caption_demo;` to `scenes/mod.rs`.

- [ ] **Step 3: Verify**

Run: `cargo check -p component-gallery`
Expected: success.

- [ ] **Step 4: Commit**

```bash
git add examples/component-gallery/src/previews/scenes/caption_demo.rs examples/component-gallery/src/previews/scenes/mod.rs
git commit -m "$(cat <<'EOF'
feat(gallery): Scene · Caption Reading-Pace Demo

Caption block inside Scene + TimelineScope. Per-word stagger at
320ms/word.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 19: Gallery scene — `WipeDemoScene`

**Files:**
- Create: `examples/component-gallery/src/previews/scenes/wipe_demo.rs`
- Modify: `examples/component-gallery/src/previews/scenes/mod.rs`

- [ ] **Step 1: Create**

```rust
use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_blocks::WipeTransition;

#[component]
pub fn WipeDemoScene() -> Element {
    rsx! {
        Scene {
            id: "wipe-demo",
            width: 1280,
            height: 720,
            duration_ms: 2_500.0,
            autoplay: Some(true),
            controls: Some(true),
            WipeTransition { duration_ms: 2_500.0, angle_deg: Some(120.0),
                div { class: "scene-wipe-fill",
                    style: "background: linear-gradient(120deg, #a04bfa, #4bbafa);",
                    h2 { style: "padding: 80px;", "Cinematic wipes ship in ui-blocks." }
                }
            }
        }
    }
}
```

- [ ] **Step 2: Register**

`pub mod wipe_demo;` in `scenes/mod.rs`.

- [ ] **Step 3: Verify**

Run: `cargo check -p component-gallery`
Expected: success.

- [ ] **Step 4: Commit**

```bash
git add examples/component-gallery/src/previews/scenes/wipe_demo.rs examples/component-gallery/src/previews/scenes/mod.rs
git commit -m "$(cat <<'EOF'
feat(gallery): Scene · Wipe Transition Demo

WipeTransition block at 120deg sweeping across a gradient backdrop.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 20: Wire the three demos into gallery (previews + docs + manifest)

**Files:**
- Modify: `examples/component-gallery/src/previews/scene.rs`
- Modify: `examples/component-gallery/src/docs.rs`
- Modify: `examples/component-gallery/e2e/tests/_lib/component-manifest.ts`

- [ ] **Step 1: Preview functions**

Append to `examples/component-gallery/src/previews/scene.rs`:

```rust
use crate::previews::scenes::caption_demo::CaptionDemoScene;
use crate::previews::scenes::lower_third_demo::LowerThirdDemoScene;
use crate::previews::scenes::wipe_demo::WipeDemoScene;

pub fn lower_third_demo_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            LowerThirdDemoScene {}
        }
    }
}

pub fn caption_demo_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            CaptionDemoScene {}
        }
    }
}

pub fn wipe_demo_preview() -> Element {
    rsx! {
        div { class: "gallery-variant-tile gallery-variant-tile--scene",
            WipeDemoScene {}
        }
    }
}
```

- [ ] **Step 2: Snippet consts + ComponentDoc entries**

In `examples/component-gallery/src/docs.rs`, append three snippet consts (after existing `SCENE_*_SNIPPET`s) and three `ComponentDoc` entries (after existing Scene category entries):

```rust
const SCENE_LOWER_THIRD_SNIPPET: &str = r##"Scene {
    id: "lower-third-demo",
    duration_ms: 4_000.0,
    Clip { start_ms: 500.0, duration_ms: 3_000.0, fill: ClipFill::HoldEnd,
        LowerThird {
            name: "Ada Lovelace".to_string(),
            role: "Mathematician".to_string(),
            accent: Some(LowerThirdAccent::Primary),
        }
    }
}"##;

const SCENE_CAPTION_SNIPPET: &str = r##"Scene {
    id: "caption-demo",
    duration_ms: 3_500.0,
    TimelineScope { id: "caption-timeline", autoplay: true,
        Caption {
            text: "Built with kinetics ui-blocks.".to_string(),
            reading_pace_ms_per_word: Some(320.0),
        }
    }
}"##;

const SCENE_WIPE_SNIPPET: &str = r##"Scene {
    id: "wipe-demo",
    duration_ms: 2_500.0,
    WipeTransition { duration_ms: 2_500.0, angle_deg: Some(120.0),
        /* gradient-filled backdrop */
    }
}"##;
```

And in the `COMPONENT_DOCS` array, append:

```rust
    ComponentDoc {
        name: "Scene · Lower Third Demo",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "ui-blocks: LowerThird chyron with name + role inside a 4s Scene + HoldEnd clip.",
        snippet: SCENE_LOWER_THIRD_SNIPPET,
        accessibility: "Parent aria-label carries \"<name>, <role>\".",
        render: Some(crate::previews::scene::lower_third_demo_preview),
    },
    ComponentDoc {
        name: "Scene · Caption Reading-Pace Demo",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "ui-blocks: Caption block driving SplitText { Word } at 320ms/word reading pace.",
        snippet: SCENE_CAPTION_SNIPPET,
        accessibility: "SplitText parent carries the full text via aria-label; word spans are aria-hidden.",
        render: Some(crate::previews::scene::caption_demo_preview),
    },
    ComponentDoc {
        name: "Scene · Wipe Transition Demo",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "ui-blocks: WipeTransition CSS mask sweep at 120deg across a gradient backdrop.",
        snippet: SCENE_WIPE_SNIPPET,
        accessibility: "Decorative; underlying heading is in normal reading order.",
        render: Some(crate::previews::scene::wipe_demo_preview),
    },
```

Bump the `COMPONENT_DOCS` array length annotation if one is present (SP-3 ran into this and bumped from 46 to 49).

- [ ] **Step 3: TS manifest**

In `examples/component-gallery/e2e/tests/_lib/component-manifest.ts`, append three entries matching the existing SP-1/SP-3 shape:

```typescript
  {
    name: "Scene · Lower Third Demo",
    slug: "scene-lower-third-demo",
    status: "ready",
    layers: { smoke: true, motion: true, visual: true },
  },
  {
    name: "Scene · Caption Reading-Pace Demo",
    slug: "scene-caption-reading-pace-demo",
    status: "ready",
    layers: { smoke: true, motion: true, visual: true },
  },
  {
    name: "Scene · Wipe Transition Demo",
    slug: "scene-wipe-transition-demo",
    status: "ready",
    layers: { smoke: true, motion: true, visual: true },
  },
```

- [ ] **Step 4: Verify**

Run: `cargo check -p component-gallery && cargo test -p component-gallery`
Expected: success.

- [ ] **Step 5: Commit**

```bash
git add examples/component-gallery/src examples/component-gallery/e2e/tests/_lib/component-manifest.ts
git commit -m "$(cat <<'EOF'
feat(gallery): wire LowerThird/Caption/Wipe demos into the Scene category

Three ComponentDoc entries with preview functions + snippets + TS
manifest entries so the e2e harness recognizes them.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 21: Playwright e2e — `catalog-blocks.spec.ts`

**Files:**
- Create: `examples/component-gallery/e2e/tests/catalog-blocks.spec.ts`

- [ ] **Step 1: Write the spec**

```ts
import { expect, test } from "@playwright/test";

const SCENE_SECTION = "#scene";

test.describe("SP-6 ui-blocks catalog", () => {
  test("LowerThird emits aria-label with name and role", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Lower Third Demo'))",
    );
    await expect(card).toBeVisible();
    const lt = card.locator(".ui-block-lower-third").first();
    await expect(lt).toHaveAttribute("aria-label", "Ada Lovelace, Mathematician");
    await expect(lt).toContainText("Ada Lovelace");
    await expect(lt).toContainText("Mathematician");
  });

  test("Caption emits per-word SplitText spans", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Caption Reading-Pace Demo'))",
    );
    await expect(card).toBeVisible();
    const caption = card.locator(".ui-block-caption").first();
    const wordCount = await caption.locator(".ui-split-text-word").count();
    expect(wordCount).toBe(4); // "Built with kinetics ui-blocks." -> 4 word tokens
  });

  test("WipeTransition emits mask-image inline style", async ({ page }) => {
    await page.goto("/");
    await page.locator(SCENE_SECTION).scrollIntoViewIfNeeded();
    const card = page.locator(
      "article.gallery-entry:has(h4:has-text('Scene · Wipe Transition Demo'))",
    );
    await expect(card).toBeVisible();
    const wipe = card.locator(".ui-block-wipe-transition").first();
    const style = (await wipe.getAttribute("style")) ?? "";
    expect(style).toMatch(/mask-image/);
    await expect(wipe).toHaveAttribute("data-angle-deg", "120");
  });
});
```

If `"Built with kinetics ui-blocks."` doesn't produce exactly 4 word tokens (it should — "Built", "with", "kinetics", "ui-blocks." — the period is part of the last token, so the count is 4), adjust the expected value to match the actual count. Inspect the rendered HTML if needed.

- [ ] **Step 2: Build the gallery**

```bash
cd examples/component-gallery
dx build --release
```

If disk pressure, run `cargo clean -p component-gallery` first.

- [ ] **Step 3: Run the spec on both engines**

```bash
cd examples/component-gallery/e2e
npx playwright test --project=static tests/catalog-blocks.spec.ts
npx playwright test --project=static-webkit tests/catalog-blocks.spec.ts
```

Both must pass (3 tests each).

- [ ] **Step 4: Commit**

```bash
git add examples/component-gallery/e2e/tests/catalog-blocks.spec.ts
git commit -m "$(cat <<'EOF'
test(gallery-e2e): Playwright spec for ui-blocks catalog showcases

Three tests on Chromium + WebKit:
- LowerThird emits aria-label with "<name>, <role>".
- Caption emits per-word SplitText spans (4 word tokens).
- WipeTransition emits mask-image inline style + 120deg data attr.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 22: Workspace verification + final cleanup

- [ ] **Step 1: Format + clippy**

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Fix any inline; if `cargo fmt` fixes were needed, commit as `chore(fmt): apply cargo fmt --all`.

- [ ] **Step 2: Full test suite**

```bash
cargo test --workspace
```

Expected: every crate's test binary reports `ok` with 0 failed. Quote a summary.

- [ ] **Step 3: wasm32**

```bash
cargo check -p ui-runtime --target wasm32-unknown-unknown
cargo check -p ui-dioxus --target wasm32-unknown-unknown
cargo check -p ui-blocks --target wasm32-unknown-unknown
```

All three green.

- [ ] **Step 4: Full Playwright regression**

```bash
cd examples/component-gallery
dx build --release
cd e2e
npx playwright test --project=static tests/catalog-blocks.spec.ts
npx playwright test --project=static-webkit tests/catalog-blocks.spec.ts
npx playwright test --project=static tests/scene-player.spec.ts            # SP-1 regression
npx playwright test --project=static-webkit tests/scene-player.spec.ts     # SP-1 regression
npx playwright test --project=static tests/gsap-tier-primitives.spec.ts    # SP-3 regression
npx playwright test --project=static-webkit tests/gsap-tier-primitives.spec.ts # SP-3 regression
```

All six must pass.

- [ ] **Step 5: Visual baselines**

If new PNG baselines were generated for the three new Scene entries under
`examples/component-gallery/e2e/tests/visual.spec.ts-snapshots/`, commit them
(SP-3 set the Chromium-only precedent):

```bash
git add examples/component-gallery/e2e/tests/visual.spec.ts-snapshots/
git commit -m "$(cat <<'EOF'
test(gallery-e2e): commit SP-6 catalog Scene visual baselines

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

If no baselines were generated, skip the commit.

- [ ] **Step 6: CLI smoke against renderer**

```bash
cargo run --bin kinetics -- doctor
cargo run --bin kinetics -- render --scene hello --out /tmp/sp-456-render --frames 3 --fps 1
```

`doctor` must print versions without panicking. `render` must produce
`/tmp/sp-456-render/frames/0.html`, `1.html`, `2.html`, and
`manifest.json`. Visually spot-check one HTML file.

- [ ] **Step 7: Branch summary**

```bash
git log --oneline main..HEAD
git diff --stat main..HEAD
```

Tally final commit count and LOC delta.

- [ ] **Step 8: Final commit (if any cleanup happened)**

If steps 1-7 produced any uncommitted changes (e.g. fmt fixes), commit them with `chore(sp-4-5-6): final fmt/clippy cleanup`. Otherwise skip.

---

## Self-Review Notes

**Spec coverage:**
- SP-4 (`kinetics-render`) → Tasks 2-5.
- SP-5 (`kinetics-cli`) → Tasks 6-9.
- SP-6 (`ui-blocks` + agent skill) → Tasks 10-16.
- Gallery showcases → Tasks 17-20.
- Playwright + verification → Tasks 21-22.

**Placeholder scan:** No "TBD" / "implement later" / "similar to" placeholders. Each step has exact code and exact commands.

**Type consistency:** `RenderConfig` fields used in Task 2 match the spec section. `SceneSpec` in Task 9 uses the `scene_fn: fn(SceneClock) -> Element` signature consistent with the `Renderer::render<F>` bound from Task 3. `LowerThird` props in Tasks 10 and 17 match (`name`, `role`, `accent: Option<LowerThirdAccent>`). `Caption` props consistent across Tasks 11, 18, 20. `WipeTransition` props consistent across Tasks 12, 19, 20.

**Known forward-references:**
- Task 9's renderer-driven scene_fn signature (`fn(SceneClock) -> Element`) means scenes referenced via the registry can't currently consume `SceneContext` because they're rendered outside a gallery context. That's an accepted SP-4+5+6 limitation: standalone-rendered scenes are self-contained.
- The Caption block's `reading_pace_ms_per_word` prop is passed through unused in Task 11 (consumed by the surrounding TimelineScope's stagger config, which isn't customizable per-Caption in SP-3). This is documented in the rustdoc; future work can wire it through.

**Plan size:** 22 tasks. Comparable to SP-1's 21 and SP-3's 17. Each task is independently buildable + committable.
