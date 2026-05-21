# Advanced UI Wave Design

## Goal

Extend `dioxus-kinetics` from a basic component MVP into a useful advanced SaaS UI wave. This phase delivers a balanced slice: reusable styling, a focused set of high-value components, basic overlay structures, and a richer component gallery. It must remain stable enough for downstream SaaS apps and avoid committing to a complex state-management layer too early.

## Scope

This phase implements:

- reusable library styling in a new `ui-styles` crate
- 12 advanced Dioxus components in `ui-dioxus`
- public prelude exports in `unified_ui`
- gallery registry updates so the new components become `Ready`
- richer gallery examples for variants, states, density, theme, and SaaS use cases

The mandatory component set is:

- `TextField`
- `Checkbox`
- `Switch`
- `Tabs`
- `Dialog`
- `Toast`
- `CommandMenu`
- `Tooltip`
- `Toolbar`
- `Sidebar`
- `MetricCard`
- `EmptyState`

The components are controlled-core and hybrid-ready. They receive state through props and render semantic structure. Stateful convenience wrappers, hooks, global overlay orchestration, and complex keyboard selection engines are intentionally outside this phase.

## Architecture

### `ui-styles`

Add a focused style crate that exports reusable CSS strings:

- `BASE_CSS`: reset, typography, CSS variables, theme variables, density variables
- `COMPONENT_CSS`: `.ui-*` class styles for library components
- `library_css()`: concatenates base and component CSS for downstream apps

The gallery can keep local layout/workbench CSS, but it depends on the shared variables and includes `library_css()` so examples use reusable library styles instead of gallery-only component styling.

### `ui-dioxus`

Expand this crate from basic primitives to advanced semantic components. Each component must:

- render semantic HTML where possible
- expose stable `.ui-*` classes
- expose ARIA attributes for role, invalid state, selected state, checked state, disabled state, and overlay semantics where applicable
- accept controlled props for state
- avoid internal app state except for purely presentational derived values

### `unified_ui`

Export the advanced components and style helpers through `unified_ui::prelude::*`. Public names must remain semantic and must not borrow external library names.

### `component-gallery`

Update the gallery registry so all 12 advanced components are `Ready`. The gallery shows real rendered examples, snippets, component summaries, and state/variant demonstrations.

## Component Behavior

### TextField

Renders a labeled text input with optional placeholder, value, disabled state, invalid state, help text, error text, and leading/trailing adornment text. It exposes an input `id`, label `for`, `aria-invalid`, and descriptive text relationships when help or error text is present.

### Checkbox

Renders a labeled checkbox with checked, indeterminate, disabled, and description support. Indeterminate is represented semantically with `aria-checked="mixed"` and a visual mixed state class.

### Switch

Renders a labeled binary switch with checked, disabled, and description support. It uses `role="switch"` and `aria-checked`.

### Tabs

Renders a controlled tab interface from tab items and panels. It uses `tablist`, `tab`, and `tabpanel` roles, sets selected state, and only shows the selected panel.

### Dialog

Renders controlled modal markup when `open` is true. It includes backdrop, panel, title, description, body, and actions. It uses `role="dialog"` and `aria-modal="true"`. Global focus trapping and overlay stack management are outside this phase.

### Toast

Renders a status notification with tone, title, description, optional action label, and optional dismiss label. It uses a live region role appropriate for the tone.

### CommandMenu

Renders controlled command-search markup with query value, grouped items, selected item id, and empty state. It uses dialog/listbox-oriented semantics without implementing full keyboard selection helpers in this phase.

### Tooltip

Renders a controlled tooltip around trigger content. It connects trigger and tooltip content with `aria-describedby` when visible.

### Toolbar

Renders grouped commands and a secondary region. It uses `role="toolbar"` and stable classes for command groups.

### Sidebar

Renders navigation sections, items, selected item, and collapsed state. It supports compact app-shell usage and sets `aria-current` on the selected item.

### MetricCard

Renders a dashboard metric with label, value, optional delta, tone, and a reserved sparkline region for downstream chart content. It is a display component for SaaS dashboards.

### EmptyState

Renders a polished empty state with title, description, optional action area, and optional visual region.

## Styling Requirements

The advanced style system must provide:

- CSS variables for foreground, muted foreground, background, surfaces, glass surfaces, border, focus, danger, warning, success, info, radius, spacing, and motion
- light and dark theme selectors via `[data-ui-theme="light"]` and `[data-ui-theme="dark"]`
- density selectors via `[data-ui-density="compact"]`, `[data-ui-density="comfortable"]`, and `[data-ui-density="spacious"]`
- hover, active, selected, disabled, invalid, focus-visible, and destructive states
- glass material styling with blur where available and solid fallback variables
- reduced motion support through `@media (prefers-reduced-motion: reduce)`
- reduced transparency support through a `[data-ui-transparency="reduced"]` selector

The visual tone is Apple-like and SaaS-focused: bright layered surfaces, precise focus rings, restrained shadows, clear status colors, compact workbench density, and no decorative landing-page treatment.

## Gallery Requirements

The gallery becomes a workbench:

- imports reusable library CSS from `ui-styles`
- keeps local layout CSS only for documentation/workbench structure
- has top-level preview controls for theme and density as static rendered controls
- groups components by category
- shows each component with summary, rendered example, variant or state examples, Rust snippet, and accessibility notes
- uses realistic SaaS contexts such as billing settings, team management, command search, dashboard metrics, empty reports, sidebar navigation, and workflow actions

The first wave does not need live client-side interactivity for toggling theme or density. It must render the controls and examples statically so the app compiles and SSR tests can verify the structure.

## Data Flow

Component examples remain registry-driven:

1. `component_docs()` returns static documentation records.
2. Each record has category, status, summary, snippet, accessibility notes, and render function.
3. Gallery sections derive from `categories()`.
4. Ready records call render functions.
5. Coming-soon entries remain possible in the data model, but the 12 advanced components in this phase are marked `Ready`.

Component state is passed through props. No component in this phase owns application workflow state.

## Error Handling

The main error surface is compile-time drift:

- snippets can drift from real APIs
- public prelude exports can miss a component
- styles can miss a component class
- gallery registry can claim a component is ready without a renderer

Tests must catch these cases through SSR structure checks, registry invariant checks, and public facade checks.

## Testing

Verification must include:

```powershell
cargo fmt --all -- --check
cargo check -p component-gallery
cargo test -p ui-styles
cargo test -p ui-dioxus
cargo test -p unified_ui
cargo test -p component-gallery
cargo test --workspace
```

Expected test coverage:

- `ui-styles` tests confirm exported CSS contains theme, density, glass, and advanced component selectors.
- `ui-dioxus` SSR tests confirm semantic structure and classes for every new component.
- `unified_ui` tests confirm the prelude exports new components and style helper.
- `component-gallery` tests confirm all 12 advanced components are ready, render examples, and expose snippets/accessibility notes.

## Out Of Scope

This phase does not implement:

- global overlay stack orchestration
- focus trapping
- full keyboard roving index or command execution engine
- runtime theme switching
- runtime density switching
- visual regression screenshots
- native renderer visual parity
- GSAP timelines
- HyperFrames export demos
- full `DataTable` or filter system

Those are follow-up plans after this balanced wave lands.
