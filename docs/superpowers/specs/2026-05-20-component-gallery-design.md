# Component Gallery Documentation App Design

## Goal

Create a Dioxus example app that documents the unified UI library category by category. The page must show what each component is for, how to call it, and what it renders like. It should be useful to downstream SaaS teams as a living component reference, not a marketing page.

## Scope

The implementation will add `examples/component-gallery` as a runnable Dioxus example crate. The gallery will document currently implemented components with live rendered examples:

- `Button`
- `Surface`
- `GlassSurface`
- `Stack`

The gallery will also include planned semantic components as disabled coming-soon entries so the documentation structure can grow without changing page architecture:

- `IconButton`
- `TextField`
- `Checkbox`
- `Tabs`
- `Dialog`
- `Toast`
- `Presence`
- `Sequence`
- `SharedLayout`
- `SharedElement`

No new library components will be implemented in this phase.

## Architecture

The example app will depend on `dioxus` and `kinetics`. It will use the public prelude so examples match downstream usage:

```rust
use kinetics::prelude::*;
```

The app will be registry-driven. A single Rust module will hold the documentation records:

```rust
struct ComponentDoc {
    name: &'static str,
    category: ComponentCategory,
    status: ComponentStatus,
    summary: &'static str,
    snippet: &'static str,
    render: Option<fn() -> Element>,
}
```

Ready components will provide a live render function. Coming-soon components will set `render` to `None` and display a disabled preview panel. This keeps planned entries visible without pretending unavailable components exist.

## Categories

Categories should describe product-facing function:

- `Actions`: command controls such as `Button` and future `IconButton`
- `Inputs`: data entry controls such as future `TextField` and `Checkbox`
- `Layout`: structure primitives such as `Stack`
- `Surfaces`: containers such as `Surface` and `GlassSurface`
- `Feedback`: user feedback and overlays such as future `Toast` and `Dialog`
- `Motion`: lifecycle and layout motion primitives such as future `Presence`, `Sequence`, `SharedLayout`, and `SharedElement`

The UI will derive category sections from the registry instead of hardcoding each section.

## Page Experience

The gallery should be a documentation surface for builders. It should prioritize scanning and comparison:

- category navigation on the left at desktop widths
- compact category tabs or stacked sections on narrow screens
- a main content column grouped by category
- one component entry per documented item
- each entry shows name, status, summary, snippet, and rendered example

The rendered example must be actual Dioxus output for ready components. Snippets can be plain preformatted Rust blocks in this phase.

## Styling

The example app will include a local stylesheet so it works immediately. It will style both gallery chrome and the existing component classes emitted by `ui-dioxus`:

- `.ui-button`
- `.ui-button--primary`
- `.ui-button--secondary`
- `.ui-button--ghost`
- `.ui-button--danger`
- `.ui-surface`
- `.ui-glass-surface`
- `.ui-stack`

The visual style should align with the library direction: Apple-like glass, soft borders, bright layered surfaces, accessible contrast, restrained motion, and SaaS-oriented density. Rendered examples should sit in preview panels, but the page should not use decorative nested card patterns.

## Data Flow

The app has static documentation data. At render time it will:

1. read the component registry
2. group entries by `ComponentCategory`
3. render category navigation
4. render each component entry
5. call the live render function only when `status` is `Ready`

Coming-soon entries never call unavailable components.

## Error Handling

The gallery has no runtime data loading. The main failure modes are compile-time issues:

- snippets drifting from actual public API
- missing render functions for ready entries
- planned components accidentally rendered before implementation

The registry types should make these cases visible in review. Verification must compile the example app so invalid imports or render functions fail early.

## Testing

Verification will run:

```powershell
cargo fmt --all -- --check
cargo check -p component-gallery
cargo test --workspace
```

If the Dioxus CLI is available locally, the implementation can also start the app with `dx serve` and report the local URL. If it is not installed, `cargo check` remains the required verification gate.

## Out Of Scope

This phase will not add new UI primitives, a full docs site router, generated Markdown, screenshot tests, syntax highlighting, search, theme switching, or package publishing. Those can be added after the gallery proves the documentation structure.
