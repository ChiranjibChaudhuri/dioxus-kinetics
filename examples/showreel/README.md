# showreel

A LinkedIn-ready video demo of the workspace's most advanced **renderable**
feature: the cinematic Scene system — kinetic text, animated charts, and a
liquid-glass metric card — composed into a 5-second hero and exported to an
MP4 with **no browser, no screen recorder, and no JS runtime**.

The committed [`showreel.mp4`](./showreel.mp4) was produced by the command
below; it is a 1920×1080, H.264, 30 fps, 5.0 s clip.

## Run it live

```powershell
dx serve --package showreel --port 9175
```

Open http://localhost:9175 in a WebGPU-capable browser to see the glass
metric card engage the WebGPU/WebGL2 engine (it falls back through SVG to
solid otherwise).

## Render the MP4

```powershell
cargo run -p kinetics-cli -- render --scene showreel `
  --out ./out --frames 150 --fps 30 --capture-png --encode-mp4
```

This walks the `showreel` Scene via `SceneClock { driver: Manual }`, writes
150 self-contained HTML frames (each with the shared library CSS inlined),
captures a PNG per frame via Playwright Chromium, and encodes the sequence
to `out/render.mp4` via FFmpeg. PNG capture and MP4 encode both degrade
gracefully (HTML frames are always written) when Node/Playwright/FFmpeg are
not on `PATH`.

### One-time capture prerequisites

```powershell
npm install playwright
npx playwright install chromium
```

`ffmpeg` must also be on `PATH` for the MP4 stage.

## What it demonstrates

- **Kinetic text** — `KineticText` with `rise-in` / `fade-in` cues, seeked
  frame-accurately by the manual clock.
- **Animated charts** — `AreaChart` and `FunnelChart` with `progress` pinned
  to the clock so the draw-in is deterministic across frames.
- **Liquid glass** — `GlassSurface` wraps the metric strip (live app only;
  SSR capture renders the solid/SVG tier).
- **Render-to-video pipeline** — the differentiator no other UI library
  ships: the same Scene composes a live app **and** a server-rendered video.
