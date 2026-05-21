# Unified UI

Unified UI is a Dioxus-first UI library for downstream SaaS products.

The library exposes one public crate:

```rust
use unified_ui::prelude::*;
```

Design principles:

- semantic component names
- Apple-like glass materials with solid fallbacks
- Web, Desktop, Mobile, and Native adapter contracts
- accessibility and reduced-preference policies
- WCAG 2.2 AA target for default themes
- optional GSAP and HyperFrames integrations outside default features

## Features

Default features:

- `web`
- `desktop`
- `mobile`
- `tokens`
- `glass`
- `motion`
- `layout-motion`
- `a11y`

Optional features:

- `native`
- `gsap`
- `hyperframes-export`
- `a11y-tests`

## First Components

- `Button`
- `Surface`
- `GlassSurface`
- `Stack`

## Component Gallery

The runnable documentation app lives in `examples/component-gallery`.

Check it with:

```powershell
cargo check -p component-gallery
```

Run it with the Dioxus CLI when available:

```powershell
dx serve --package component-gallery
```

The gallery is registry-driven. Ready components render live examples, and planned components appear as disabled coming-soon entries until their implementation lands.

## Documentation

- `docs/component-naming.md`
- `docs/glass-materials.md`
- `docs/platform-support.md`
- `docs/superpowers/specs/2026-05-20-component-gallery-design.md`
