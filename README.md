# Unified UI

Unified UI is a Dioxus-first UI library for downstream SaaS products.

The library exposes one public crate, `unified_ui`, while keeping tokens,
glass materials, motion, layout, and renderer adapters in focused internal
crates.

Default goals:

- semantic component names
- Apple-like glass materials with solid fallbacks
- Web, Desktop, Mobile, and Native adapter contracts
- accessibility and reduced-preference policies
- WCAG 2.2 AA target for default themes
- optional GSAP and HyperFrames integrations outside default features
