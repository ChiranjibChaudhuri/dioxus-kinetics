# Render Pipeline + CLI + Catalog + Agent Skill — Design (SP-4+5+6)

## Goal

Land the final three "HyperFrames for Dioxus" sub-projects in one
phase. SP-1 shipped the Scene Player; SP-3 shipped the GSAP-tier
primitives. This phase adds:

- **SP-4**: a frame-by-frame Rust renderer that walks any `Scene` via
  `SceneDriver::Manual` + `clock.seek_ms(t)` and emits per-frame
  HTML + an `ExportManifest`. PNG capture (via Playwright child) and
  MP4 encoding (via FFmpeg child) are opt-in, graceful-degradation
  features.
- **SP-5**: a `kinetics` CLI (`init`, `preview`, `render`, `lint`,
  `doctor`) wrapping the existing dev-loop and the new renderer.
- **SP-6**: a `ui-blocks` catalog crate with five reusable cinematic
  blocks (LowerThird, Caption, WipeTransition, MetricCounter,
  SocialOverlay), plus a `.claude/skills/kinetics-scene/` skill that
  teaches AI agents the Scene API.

The combined spec ships in one phase per user direction. Sub-project
boundaries are preserved internally so each piece is independently
testable.

## Scope

In scope:

1. **`kinetics-render`** crate (`crates/kinetics-render/`):
   - `Renderer` struct with builder API: `frames(n)`, `fps(60)`,
     `output_dir(p)`, `composition_id(id)`.
   - `Renderer::render<F>(scene_fn: F)` where
     `F: Fn(SceneClock) -> Element` constructs a scene rooted at the
     supplied clock. The renderer iterates frame 0..N, calls
     `clock.seek_ms((frame / fps) * 1000)`, and serializes the
     resulting `Element` via `dioxus_ssr::render_element`.
   - Output: `<output_dir>/frames/<frame_index>.html` per frame plus
     `<output_dir>/manifest.json` (a `ui_capture::ExportManifest`).
   - Errors via `RenderError` enum (IO, invalid config). No panics
     in the public API.
   - Cross-platform native: tokio + std::fs. No wasm-bindgen.
2. **Optional PNG capture** (gated by an `--capture-png` flag):
   - The renderer writes a small Node script
     (`output_dir/capture.cjs`) and spawns it via
     `tokio::process::Command::new("npx").args(["playwright", ...])`.
   - The Node script loads each HTML frame in a Playwright Chromium
     context, screenshots to `<output_dir>/png/<frame_index>.png`.
   - If `npx` or Playwright are unavailable, the renderer returns
     `Ok(())` for the HTML phase and emits a warning indicating that
     PNG capture was skipped.
3. **Optional MP4 encoding** (gated by `--encode-mp4`):
   - The renderer spawns FFmpeg as a child:
     `ffmpeg -framerate {fps} -i <output_dir>/png/%d.png -c:v libx264 -pix_fmt yuv420p <output_dir>/render.mp4`.
   - Requires `--capture-png` (no PNGs → nothing to encode).
   - Missing FFmpeg → warning, no error.
4. **`kinetics-cli`** crate (`crates/kinetics-cli/`):
   - Binary name `kinetics` (Cargo workspace member, `[[bin]]`
     entrypoint).
   - `clap` derive-based arg parsing.
   - Five subcommands:
     - `kinetics init <name>` — scaffold a Dioxus + kinetics example
       crate in `./<name>/` using an embedded template (single
       `main.rs` rendering a 5-second Product Intro Scene).
     - `kinetics preview [--target gallery]` — `dx serve --hot-reload`
       on the named target (defaults to `gallery`).
     - `kinetics render --scene <name> --out <dir> [--frames N]
       [--fps N] [--capture-png] [--encode-mp4]` — runs the
       `kinetics-render` Renderer against a named scene from the
       gallery's manifest of registered scenes.
     - `kinetics lint` — runs `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings`.
     - `kinetics doctor` — prints versions: rustc, cargo, dx, dioxus,
       Playwright (if installed), FFmpeg (if installed). Exits 0
       even if optional tools are missing — only reports.
   - All subcommands return a typed exit code (0 success, non-zero
     for command failures); `clap` handles `--help`/`--version`.
5. **`ui-blocks`** crate (`crates/ui-blocks/`):
   - Five `#[component]` blocks listed in the goal section. Each
     accepts plain props (Strings, enums, durations) and composes
     SP-1 / SP-3 primitives. No private API access.
   - Each block has SSR tests asserting key DOM structure (e.g.
     `LowerThird` emits an `aria-label` carrying name + role;
     `Caption` emits per-word spans with stagger indices).
   - Re-exported via `kinetics::prelude` so downstream code reads
     `use kinetics::prelude::*; rsx! { LowerThird { ... } }`.
6. **Gallery showcase entries** for three of the five blocks
   (the most distinctive):
   - `Scene · Lower Third Demo`
   - `Scene · Caption Reading-Pace Demo`
   - `Scene · Wipe Transition Demo`
   The other two (MetricCounter, SocialOverlay) are exposed in the
   prelude and visible via downstream consumers; the gallery is the
   integration surface, not a comprehensive catalog viewer.
7. **`.claude/skills/kinetics-scene/SKILL.md`** — a single Markdown
   document with the agent skill name `kinetics-scene` and
   description that triggers it when users ask about authoring Scene
   compositions. Body covers:
   - Scene / Clip / SceneDriver / Sequence API surface.
   - SplitText / MotionPath usage.
   - ui-blocks catalog quick-reference.
   - Reduced-motion + accessibility patterns.
   - Workspace conventions (TDD, `let mut s = …; s.set(…);`,
     Conventional Commits).
8. **`kinetics::prelude`** extensions for the new public surface.
9. **Playwright e2e** spec for the three new gallery showcases.

Out of scope:

- Cloud / distributed rendering.
- Audio mixing (silent video output).
- Live preview server with hot-reload of scene definitions (the
  `preview` subcommand defers to `dx serve`).
- Real-time HTTP API for the renderer (the renderer is a library +
  CLI front-end, not a server).
- SVG path string parsing for MotionPath.
- WebGL shader transitions.
- Live agent skill auto-update (the SKILL.md is committed and
  refreshed in subsequent SPs as the API evolves).
- A standalone `kinetics-render` binary — render flows through the
  CLI's `kinetics render` subcommand. The crate exports a library
  API only.
- Native (non-Chromium) PNG capture. The Playwright shell is the
  only PNG path.

## Architecture

### Crate layout

```
crates/
  kinetics-render/
    src/
      lib.rs                # Renderer + RenderConfig + RenderError + Frame
      capture.rs            # PNG capture orchestrator (Playwright shell)
      encode.rs             # MP4 encoder orchestrator (FFmpeg shell)
      template.rs           # capture.cjs script source
    tests/
      renderer.rs           # HTML frame export tests
      capture.rs            # PNG capture tests (skipped if Playwright unavailable)
      encode.rs             # MP4 encode tests (skipped if FFmpeg unavailable)

  kinetics-cli/
    src/
      main.rs               # clap entrypoint + subcommand dispatch
      cmd_init.rs           # init subcommand
      cmd_preview.rs        # preview subcommand
      cmd_render.rs         # render subcommand
      cmd_lint.rs           # lint subcommand
      cmd_doctor.rs         # doctor subcommand
      scene_registry.rs     # named scene -> Renderer config mapping
      template/
        init_main.rs.tmpl   # embedded scaffold for `kinetics init`
    tests/
      cli.rs                # smoke tests via `assert_cmd`

  ui-blocks/
    src/
      lib.rs                # re-exports the 5 blocks
      lower_third.rs
      caption.rs
      wipe_transition.rs
      metric_counter.rs
      social_overlay.rs
    tests/
      blocks_ssr.rs         # SSR tests, one per block

  kinetics/src/lib.rs       # +prelude exports + +public_api_names

.claude/skills/
  kinetics-scene/
    SKILL.md

examples/component-gallery/src/previews/scenes/
  lower_third_demo.rs
  caption_demo.rs
  wipe_demo.rs

examples/component-gallery/e2e/tests/
  catalog-blocks.spec.ts    # 3 tests (one per showcase)
```

### `kinetics-render` API

```rust
pub struct Renderer { /* private */ }

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

pub struct RenderReport {
    pub frames_written: u32,
    pub html_dir: PathBuf,
    pub png_dir: Option<PathBuf>,
    pub mp4_path: Option<PathBuf>,
    pub warnings: Vec<String>,
}

#[derive(Debug)]
pub enum RenderError {
    InvalidConfig(String),
    Io(io::Error),
}

impl Renderer {
    pub fn new(config: RenderConfig) -> Self;

    /// Renders `scene_fn(clock)` over the configured frame range.
    /// `scene_fn` receives a `SceneClock` already configured for
    /// Manual driving; the renderer seeks it per-frame.
    pub fn render<F>(&self, scene_fn: F) -> Result<RenderReport, RenderError>
    where
        F: Fn(SceneClock) -> Element + 'static;
}
```

The renderer is sync-on-the-outside, with the per-frame work
internally awaiting tokio-spawned tasks if PNG/MP4 stages run. The
public surface stays simple to call from the CLI.

The `scene_fn` closure constructs the Dioxus tree fresh for each
frame. Re-rendering is cheap because `dioxus-ssr` instantiates a
fresh VirtualDom per call; we're not building a long-lived render
graph.

### PNG capture (`capture_png: true`)

After HTML frames are written:

1. Renderer writes `capture.cjs` (embedded as a constant string) to
   the output directory. The script:
   - Accepts the output dir as argv[2].
   - Launches Playwright Chromium with `headless: true,
     viewport: { width, height }`.
   - Iterates `frames/*.html`, navigates the page to a `file://` URL,
     waits for `networkidle`, calls `page.screenshot({ path: ... })`.
2. Renderer spawns `npx playwright install chromium` (idempotent)
   followed by `node capture.cjs <output_dir>`.
3. Success → `RenderReport.png_dir = Some(<output_dir>/png)`.
4. Any of: `npx` not on PATH, `playwright` not installed, the script
   exits non-zero → returns `Ok(report)` with `png_dir = None` and a
   warning string explaining the skip.

### MP4 encoding (`encode_mp4: true`)

Requires `capture_png` to also be true. After PNG capture:

1. Renderer spawns FFmpeg:
   ```
   ffmpeg -y -framerate {fps} -i {output_dir}/png/%d.png \
          -c:v libx264 -pix_fmt yuv420p {output_dir}/render.mp4
   ```
2. Success → `RenderReport.mp4_path = Some(...)`.
3. FFmpeg not on PATH or exits non-zero → `Ok(report)` with
   `mp4_path = None` and a warning.

### `kinetics-cli` design

```
$ kinetics --help
A CLI for authoring and rendering kinetics Scene compositions.

USAGE:
    kinetics <COMMAND>

COMMANDS:
    init       Scaffold a new kinetics example crate
    preview    Run the dev server (dx serve --hot-reload)
    render     Render a scene to HTML/PNG/MP4
    lint       Run fmt --check + clippy across the workspace
    doctor     Print toolchain versions
```

`kinetics render --scene <name>` reads from a static `SceneRegistry`
in `scene_registry.rs` that maps named scenes (e.g.
`"product-intro"`, `"scroll-story"`, `"split-headline"`) to the
corresponding `RenderConfig` + `scene_fn`. SP-4+5+6 ships five
named scenes:

- `product-intro` (SP-1)
- `scroll-story` (SP-3)
- `split-headline` (SP-3)
- `curved-trajectory` (SP-3)
- `lower-third` (SP-6)

The registry is hard-coded for SP-4+5+6; user-extensible registries
are a follow-up.

### `ui-blocks` catalog

Each block is a `#[component]` taking plain props. Internal
composition uses `Scene`/`Clip`/`Sequence`/`SplitText`/`MotionPath`
exclusively — no private API.

#### `LowerThird`

```rust
#[component]
pub fn LowerThird(
    name: String,
    role: String,
    accent: Option<LowerThirdAccent>,  // default LowerThirdAccent::Primary
) -> Element;
```

Renders a fixed-position bar at the bottom-left of the parent container.
The bar slides in from the left over 600ms, then the name + role fade
in 200ms apart. `aria-label` on the parent carries
`"<name>, <role>"`.

#### `Caption`

```rust
#[component]
pub fn Caption(
    text: String,
    reading_pace_ms_per_word: Option<f32>,  // default 320.0
) -> Element;
```

Renders a centered caption bar. Uses `SplitText { split_by: Word,
stagger_step_ms: Some(reading_pace_ms_per_word) }` so the words
appear at a natural reading pace.

#### `WipeTransition`

```rust
#[component]
pub fn WipeTransition(
    duration_ms: f32,
    angle_deg: Option<f32>,  // default 90.0 (left-to-right)
    children: Element,
) -> Element;
```

Renders a CSS conic-gradient mask that sweeps across the children over
`duration_ms`. Implementation: a `KineticBox` whose `mask-image` CSS
animates from `conic-gradient(0deg, transparent 0%, black 0%)` to
`conic-gradient(0deg, transparent 100%, black 100%)`.

#### `MetricCounter`

```rust
#[component]
pub fn MetricCounter(
    label: String,
    value: String,
    delta_text: Option<String>,
) -> Element;
```

Three `KineticText` elements: label, value, optional delta. Sequential
fade-up via a `TimelineScope` stagger.

#### `SocialOverlay`

```rust
#[component]
pub fn SocialOverlay(
    platform: SocialPlatform,    // enum: Instagram | Twitter | YouTube | TikTok
    handle: String,              // e.g. "@kineticsui"
    message: String,             // e.g. "Just followed you!"
) -> Element;
```

Notification-card style overlay that slides in from the top-right,
holds for 2 seconds, then slides out. Platform enum controls the
brand-color accent.

### Agent skill — `.claude/skills/kinetics-scene/SKILL.md`

Single Markdown file with YAML frontmatter:

```yaml
---
name: kinetics-scene
description: Use when authoring or modifying Dioxus Kinetics Scene
  compositions — covers Scene/Clip/SceneDriver, SplitText/MotionPath,
  ui-blocks catalog, reduced-motion patterns, and workspace TDD
  conventions.
---
```

Body sections:

1. **Quick start** — minimal `Scene { ... }` example with autoplay.
2. **Drivers** — when to pick Autoplay / Manual / Scroll, with
   example snippets.
3. **Clips** — start_ms / duration_ms / fill semantics.
4. **Per-glyph text** — SplitText Character vs Word.
5. **Curved motion** — MotionPath + PathPoint::Line / Bezier.
6. **Catalog blocks** — quick-reference list with 1-line summaries.
7. **Reduced motion** — `ReducedMotionProvider`, what each component
   does when reduced.
8. **Accessibility** — aria-label patterns, aria-hidden on glyphs.
9. **Workspace conventions** — TDD ordering, the
   `let mut s = …; s.set(…);` Signal idiom, Conventional Commits.

### Data flow — `kinetics render` happy path

```
$ kinetics render --scene product-intro --out /tmp/out --frames 60 --capture-png --encode-mp4
   ↓
[CLI parses args, builds RenderConfig]
   ↓
[CLI looks up "product-intro" in SceneRegistry → scene_fn + base config]
   ↓
[Renderer::render(scene_fn)]
   ↓
For each frame in 0..N:
  - clock.seek_ms(frame / fps * 1000)
  - element = scene_fn(clock)
  - html = dioxus_ssr::render_element(rsx! { {element} })
  - fs::write("frames/{frame}.html", html)
   ↓
[Renderer writes manifest.json (ExportManifest)]
   ↓
[capture_png: true → write capture.cjs, spawn "node capture.cjs <out>"]
   ↓
For each html file: Playwright screenshot → "png/{frame}.png"
   ↓
[encode_mp4: true → spawn ffmpeg -framerate 60 -i png/%d.png ... render.mp4]
   ↓
RenderReport { frames_written: 60, html_dir, png_dir, mp4_path, warnings }
   ↓
[CLI prints report summary; exits 0]
```

Failure modes route through `Ok(report)` with warnings (not
`Err`) for optional-tool absences. Hard `Err` only for: invalid
config (e.g. fps=0), filesystem errors writing HTML.

## Testing

### Unit (`kinetics-render`)

- `renderer::tests::config_validation_rejects_zero_fps`.
- `renderer::tests::config_validation_rejects_zero_frames`.
- `renderer::tests::renders_n_html_files_with_seeked_elapsed_ms`
  (asserts `frames/0.html` and `frames/<N-1>.html` contain different
  `data-elapsed-ms` attributes).
- `renderer::tests::writes_export_manifest`.

### Integration — PNG and MP4 (`kinetics-render`)

Both stages are gated by environment availability:

- `capture::tests::skips_when_npx_unavailable` — uses a fake PATH.
- `encode::tests::skips_when_ffmpeg_unavailable`.
- If `KINETICS_RENDER_FULL=1` is set in the environment, an
  additional pair of tests runs the real Playwright + FFmpeg paths
  end-to-end on a 2-frame composition. CI doesn't set this flag;
  local development can.

### Unit (`kinetics-cli`)

Using `assert_cmd`:

- `cli::tests::help_prints_subcommands`.
- `cli::tests::init_creates_scaffolded_directory`.
- `cli::tests::lint_returns_zero_on_clean_workspace` — runs `kinetics lint` against a temp workspace fixture.
- `cli::tests::doctor_succeeds_even_when_optional_tools_missing`.
- `cli::tests::render_with_unknown_scene_returns_nonzero`.

### Integration — SSR (`ui-blocks`)

One test per block in `tests/blocks_ssr.rs`:

- `lower_third::tests::emits_aria_label_with_name_and_role`.
- `caption::tests::emits_per_word_split_text_spans`.
- `wipe_transition::tests::emits_mask_image_kinetic_box`.
- `metric_counter::tests::renders_three_kinetic_text_lines`.
- `social_overlay::tests::renders_platform_accent_class`.

### E2E (`gallery`)

`examples/component-gallery/e2e/tests/catalog-blocks.spec.ts`:

- `Scene · Lower Third Demo` — aria-label assertion.
- `Scene · Caption Reading-Pace Demo` — per-word spans present.
- `Scene · Wipe Transition Demo` — mask-image style present.

Both Chromium and WebKit projects.

## Files (final)

New:

- `crates/kinetics-render/Cargo.toml`
- `crates/kinetics-render/src/lib.rs`
- `crates/kinetics-render/src/capture.rs`
- `crates/kinetics-render/src/encode.rs`
- `crates/kinetics-render/src/template.rs`
- `crates/kinetics-render/tests/renderer.rs`
- `crates/kinetics-render/tests/capture.rs`
- `crates/kinetics-render/tests/encode.rs`
- `crates/kinetics-cli/Cargo.toml`
- `crates/kinetics-cli/src/main.rs`
- `crates/kinetics-cli/src/cmd_init.rs`
- `crates/kinetics-cli/src/cmd_preview.rs`
- `crates/kinetics-cli/src/cmd_render.rs`
- `crates/kinetics-cli/src/cmd_lint.rs`
- `crates/kinetics-cli/src/cmd_doctor.rs`
- `crates/kinetics-cli/src/scene_registry.rs`
- `crates/kinetics-cli/src/template/init_main.rs.tmpl`
- `crates/kinetics-cli/tests/cli.rs`
- `crates/ui-blocks/Cargo.toml`
- `crates/ui-blocks/src/lib.rs`
- `crates/ui-blocks/src/lower_third.rs`
- `crates/ui-blocks/src/caption.rs`
- `crates/ui-blocks/src/wipe_transition.rs`
- `crates/ui-blocks/src/metric_counter.rs`
- `crates/ui-blocks/src/social_overlay.rs`
- `crates/ui-blocks/tests/blocks_ssr.rs`
- `.claude/skills/kinetics-scene/SKILL.md`
- `examples/component-gallery/src/previews/scenes/lower_third_demo.rs`
- `examples/component-gallery/src/previews/scenes/caption_demo.rs`
- `examples/component-gallery/src/previews/scenes/wipe_demo.rs`
- `examples/component-gallery/e2e/tests/catalog-blocks.spec.ts`

Edited:

- `Cargo.toml` (workspace) — add new crate members.
- `crates/kinetics/src/lib.rs` — prelude + `public_api_names`.
- `crates/kinetics/Cargo.toml` — add `ui-blocks` dep behind a
  `blocks` feature (default enabled).
- `examples/component-gallery/src/previews/scenes/mod.rs` — pub mod
  the three new demos.
- `examples/component-gallery/src/previews/scene.rs` — three new
  preview functions.
- `examples/component-gallery/src/docs.rs` — three new `ComponentDoc`
  entries + snippet consts.
- `examples/component-gallery/e2e/tests/_lib/component-manifest.ts`
  — three new manifest entries.
- `examples/component-gallery/Cargo.toml` — `ui-blocks` workspace dep.

## Risks and mitigations

| Risk | Mitigation |
|------|------------|
| `clap` derive macros may not be in the workspace yet. | Add `clap = { version = "4", features = ["derive"] }` to `kinetics-cli`'s `Cargo.toml`. Workspace dep section. |
| `assert_cmd` adds a heavy dev-dep. | Use `escargot` + `std::process::Command` if `assert_cmd` proves too heavy. SP-4+5+6's CLI tests are smoke-level; either works. |
| Playwright Chromium download is multi-hundred-MB on first run. | The CLI's `doctor` subcommand surfaces whether it's installed. `kinetics render --capture-png` returns a warning if missing rather than hanging. |
| FFmpeg encoding parameters tuned for one platform may produce non-portable MP4s. | The default args (`libx264`, `yuv420p`) are the conservative "plays everywhere" preset. Document the choice in the rustdoc. |
| `kinetics init` template scaffold may go stale as Dioxus API evolves. | Template is one file (a `main.rs`), short and copy-paste-mechanical. Update happens in the same PR as any breaking Dioxus bump. |
| Multiple new crates inflate cold-build times. | Each new crate is small (< 500 lines) and only `ui-blocks` is reached during normal gallery builds. CLI + renderer build only when explicitly targeted. |
| `ui-blocks` could grow into a kitchen-sink dump. | SP-4+5+6 ships exactly five blocks. Future blocks gate through a new SP (or an explicit follow-up). |
| Agent skill text rot. | The SKILL.md is a single file we commit alongside the spec; future SPs that change the API also touch the skill. CI doesn't validate it (out of scope for SP-4+5+6). |
| `kinetics-render` SSR loop builds a new `VirtualDom` per frame — O(N) cost. | For SP-4+5+6's typical 60-frame at 60fps = 1-second compositions, this is sub-second total. Caching across frames is a future optimization once the pipeline is in production use. |
| Playwright + FFmpeg shell-outs are platform-dependent (Windows path quoting, `npx` vs `npx.cmd`, etc.). | Use `tokio::process::Command::new("npx")` directly — Cargo's std + Windows toolchain handle `.cmd` resolution. Document the requirement in `kinetics doctor`. |
| The CLI's `lint` and `preview` are thin wrappers — risk of being marketing-only. | Acknowledge: they are wrappers. They exist for discoverability ("type `kinetics lint`, get the right command"). Not a defect. |

## Decisions and rationale

1. **Single phase for SP-4+5+6.** User directed. The three are
   complementary: the renderer is consumed by the CLI; the catalog
   is authored using the API both layers expose. Shipping them
   together makes a complete "develop → render → ship" story.
2. **Renderer is library + CLI consumer, not a standalone binary.**
   Keeps the surface narrow. The CLI is the single front door for
   user-facing operations.
3. **PNG capture via Playwright, not chromiumoxide.** Playwright is
   already a dev-tool in the workspace (e2e). Spawning it as a child
   process avoids pulling in a heavy Rust browser-automation
   dependency.
4. **MP4 via FFmpeg.** Universal availability; the canonical "this
   plays everywhere" output format. No in-process encoder (would
   require linking `x264` etc.).
5. **`ui-blocks` is a separate crate, not part of `ui-dioxus`.**
   Keeps the kinetics core lean. Downstream apps that don't want
   the catalog can skip the `blocks` feature.
6. **Five blocks, three showcased.** Five gives a satisfying
   minimum-viable catalog. Showcasing three keeps the gallery
   focused; MetricCounter is already implicitly demoed in SP-1's
   Product Intro, and SocialOverlay's full effect needs a video
   demo (which the gallery static preview can't reproduce).
7. **Agent skill ships as a file, not as a CI-validated artifact.**
   Validation tooling for skill content is itself a future
   sub-project.
8. **No user-extensible scene registry.** The five named scenes are
   enough to demonstrate the CLI; arbitrary scene registration
   requires a plugin/dynlib model that's a future sub-project.
9. **`init` template is a single file.** A multi-file scaffold has
   to encode Cargo workspace decisions that vary per consumer.
   One-file template is honest about what `init` actually buys you:
   a working `main.rs` you'd otherwise copy from the docs.

## Acceptance criteria

- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and
  `cargo check -p ui-runtime --target wasm32-unknown-unknown` all
  pass.
- `cargo test -p kinetics-render` passes (HTML + optional-skip
  capture/encode tests).
- `cargo test -p kinetics-cli` passes (smoke tests via `assert_cmd`).
- `cargo test -p ui-blocks` passes (5 SSR tests).
- `kinetics render --scene product-intro --out /tmp/test --frames 30
  --fps 30` writes 30 HTML files + `manifest.json` and exits 0 on
  Windows / macOS / Linux.
- `kinetics doctor` runs and reports versions without panicking.
- The three new gallery entries render in the Scene category.
- `npx playwright test --project=static tests/catalog-blocks.spec.ts`
  passes on Chromium; WebKit ditto.
- Existing SP-1 + SP-3 Playwright specs continue to pass (zero
  regression).
- `.claude/skills/kinetics-scene/SKILL.md` exists, is valid YAML
  frontmatter + Markdown, and is referenced from the README (one
  line in "Available agent skills" or similar).

## Follow-ups

- **Cloud rendering** — extend `Renderer` to write to S3 / GCS, and
  add a `kinetics render --remote` flag.
- **Audio mixing** — bring `<audio>` clip semantics from HeyGen's
  HyperFrames into `ui-composition`, then mix during the MP4 encode
  stage via FFmpeg's audio inputs.
- **Hot-reload preview** — `kinetics preview` could open a browser
  to a scene-specific URL with `dx serve` hot-reload, not just the
  generic gallery.
- **User-extensible scene registry** — a plugin model so downstream
  apps register their own scenes for `kinetics render`.
- **Agent skill validation** — a CI lint that fails if SKILL.md
  references symbols that don't exist in the prelude.
- **Catalog growth** — additional blocks (Chart, Quote, Avatar,
  Section divider, etc.) under future SP-7+.
- **WebGL shader transitions** — extend `WipeTransition` to use
  WebGL fragment shaders for richer effects.
