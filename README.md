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

## Documentation

- `docs/component-naming.md`
- `docs/glass-materials.md`
- `docs/platform-support.md`
