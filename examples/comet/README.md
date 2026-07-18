# comet

A Perplexity-Comet-style "agentic browser" landing page — dark, warm, Perplexity-teal
(#20B8CE), composed entirely from kinetics primitives:

- `LiquidGlass` — the Apple-style glass that frames the browser window and the
  feature cards
- `KineticText` — rise-in / fade-in hero copy
- `AgentTimeline` + `AiStatus` — the live agent rail inside the browser mockup
  (Done / Active / Pending steps, "Searching" status)
- `Button`-style CTAs and a sketched content pane

## Run it

```powershell
dx serve --package comet --port 9176
```

Open http://localhost:9176 in a WebGPU-capable browser so the `LiquidGlass`
surfaces engage the WebGPU/WebGL2 engine (they fall back through SVG to solid
otherwise).

## Note on fidelity

The live Perplexity Comet site blocks programmatic fetches (403), so this page
matches the **known Comet identity** — warm-dark background, Perplexity teal,
"the agentic browser" hero with a browser-window mockup whose agent rail is
mid-task — rather than being a pixel-perfect copy. It exists to showcase how
the kinetics primitives compose into a polished, on-brand marketing surface.
