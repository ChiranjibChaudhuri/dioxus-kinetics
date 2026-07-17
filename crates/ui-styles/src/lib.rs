#![forbid(unsafe_code)]

use ui_tokens::elevation::{DARK_ELEVATION, LIGHT_ELEVATION};

pub fn base_css() -> String {
    format!(
        r#"
:root,
[data-ui-theme="light"] {{
    color-scheme: light;
    --ui-font-sans: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    --ui-bg: #f6f8fb;
    --ui-surface: #ffffff;
    --ui-surface-muted: #f2f5f9;
    --ui-surface-strong: #e8eef6;
    --ui-glass: rgba(255, 255, 255, 0.68);
    --ui-glass-solid: #ffffff;
    --ui-glass-blur: 18px;
    --ui-glass-saturate: 160%;
    --ui-glass-highlight: rgba(255, 255, 255, 0.5);
    --ui-glass-highlight-bottom: rgba(255, 255, 255, 0.12);
    --ui-fg: #111827;
    --ui-muted-fg: #5c6778;
    --ui-border: rgba(118, 132, 150, 0.26);
    --ui-focus: #007aff;
    --ui-primary: #0058b3;
    --ui-success: #1a6b2e;
    --ui-warning: #9a5800;
    --ui-danger: #c42b2b;
    --ui-info: #0f63a3;
    --ui-accent: var(--ui-primary);
    --ui-on-accent: #ffffff;
    --ui-on-danger: #ffffff;
    --ui-on-warning: #1a1a1a;
    --ui-shadow-soft: 0 18px 46px rgba(27, 39, 61, 0.10);
    --ui-shadow-lifted: 0 24px 80px rgba(13, 20, 32, 0.24);
    --ui-elevation-0: {l0};
    --ui-elevation-1: {l1};
    --ui-elevation-2: {l2};
    --ui-elevation-3: {l3};
    --ui-radius-sm: 6px;
    --ui-radius-md: 10px;
    --ui-radius-lg: 14px;
    --ui-radius-floating: 18px;
    --ui-radius-full: 999px;
    --ui-space-0: 2px;
    --ui-space-1: 4px;
    --ui-space-2: 8px;
    --ui-space-3: 12px;
    --ui-space-4: 16px;
    --ui-space-5: 24px;
    --ui-space-6: 32px;
    --ui-space-7: 48px;
    --ui-space-8: 64px;
    --ui-control-height: 36px;
    --ui-motion-fast: 120ms;
    --ui-motion-normal: 180ms;
    --ui-motion-press: 90ms;
    --ui-press-scale: 0.97;
    --ui-ease-standard: cubic-bezier(0.2, 0, 0, 1);
    --ui-ease-emphasized: cubic-bezier(0.2, 0, 0, 1);
    --ui-ease-decelerate: cubic-bezier(0, 0, 0, 1);
    --ui-ease-accelerate: cubic-bezier(0.4, 0, 1, 1);
    --ui-ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1);
    --ui-text-caption2: 11px;
    --ui-leading-caption2: 1.45;
    --ui-tracking-caption2: 0.005em;
    --ui-text-caption: 12px;
    --ui-leading-caption: 1.40;
    --ui-tracking-caption: 0.004em;
    --ui-text-footnote: 13px;
    --ui-leading-footnote: 1.45;
    --ui-tracking-footnote: 0em;
    --ui-text-subhead: 15px;
    --ui-leading-subhead: 1.40;
    --ui-tracking-subhead: -0.002em;
    --ui-text-callout: 16px;
    --ui-leading-callout: 1.45;
    --ui-tracking-callout: -0.004em;
    --ui-text-body: 17px;
    --ui-leading-body: 1.47;
    --ui-tracking-body: -0.006em;
    --ui-text-headline: 17px;
    --ui-leading-headline: 1.40;
    --ui-tracking-headline: -0.006em;
    --ui-text-title3: 20px;
    --ui-leading-title3: 1.25;
    --ui-tracking-title3: -0.010em;
    --ui-text-title2: 22px;
    --ui-leading-title2: 1.20;
    --ui-tracking-title2: -0.012em;
    --ui-text-title1: 28px;
    --ui-leading-title1: 1.15;
    --ui-tracking-title1: -0.016em;
    --ui-text-largetitle: 34px;
    --ui-leading-largetitle: 1.10;
    --ui-tracking-largetitle: -0.020em;
    --ui-text-display: clamp(40px, 5vw, 64px);
    --ui-leading-display: 1.04;
    --ui-tracking-display: -0.022em;
    --ui-weight-regular: 400;
    --ui-weight-medium: 500;
    --ui-weight-semibold: 600;
    --ui-weight-bold: 700;
}}

[data-ui-theme="dark"] {{
    color-scheme: dark;
    --ui-bg: #0d1117;
    --ui-surface: #151b23;
    --ui-surface-muted: #1c2430;
    --ui-surface-strong: #263142;
    --ui-glass: rgba(25, 32, 43, 0.72);
    --ui-glass-solid: #151b23;
    --ui-glass-highlight: rgba(255, 255, 255, 0.22);
    --ui-glass-highlight-bottom: rgba(255, 255, 255, 0.12);
    --ui-fg: #eef3f8;
    --ui-muted-fg: #aab4c2;
    --ui-border: rgba(205, 215, 228, 0.18);
    --ui-focus: #64b5ff;
    --ui-primary: #4c9bff;
    --ui-success: #3ecf6a;
    --ui-warning: #f0a82e;
    --ui-danger: #ff6b6b;
    --ui-info: #5cb6ff;
    --ui-accent: var(--ui-primary);
    --ui-on-accent: #06121f;
    --ui-on-danger: #3a0d0d;
    --ui-on-warning: #1a1a1a;
    --ui-shadow-soft: 0 18px 46px rgba(0, 0, 0, 0.24);
    --ui-shadow-lifted: 0 26px 90px rgba(0, 0, 0, 0.42);
    --ui-elevation-0: {d0};
    --ui-elevation-1: {d1};
    --ui-elevation-2: {d2};
    --ui-elevation-3: {d3};
}}

[data-ui-density="compact"] {{
    --ui-control-height: 32px;
    --ui-space-3: 10px;
    --ui-space-4: 12px;
}}

[data-ui-density="comfortable"] {{
    --ui-control-height: 36px;
}}

[data-ui-density="spacious"] {{
    --ui-control-height: 42px;
    --ui-space-3: 14px;
    --ui-space-4: 20px;
}}

[data-ui-transparency="reduced"] {{
    --ui-glass: var(--ui-glass-solid);
}}

* {{
    box-sizing: border-box;
}}

body {{
    margin: 0;
    font-family: var(--ui-font-sans);
    background: var(--ui-bg);
    color: var(--ui-fg);
    font-size: var(--ui-text-body);
    line-height: var(--ui-leading-body);
    letter-spacing: var(--ui-tracking-body);
    font-optical-sizing: auto;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
}}

button,
input,
textarea,
select {{
    font: inherit;
}}

@media (prefers-reduced-motion: reduce) {{
    *,
    *::before,
    *::after {{
        transition-duration: 0.01ms !important;
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        scroll-behavior: auto !important;
    }}
}}

/* Windows High Contrast / forced-colors. The OS replaces our palette with a
   user-chosen system color set, so we map the load-bearing surfaces, borders
   and focus rings to CSS system colors and let decorative glass fall back to
   the system Canvas. forced-color-adjust:auto (the default) keeps controls in
   the system theme; we only adjust where our own tokens would otherwise leak a
   non-system color. Inset focus rings are pushed to a non-negative offset so
   the OS-drawn Highlight ring is not clipped by overflow:hidden ancestors. */
@media (forced-colors: active) {{
    :root,
    [data-ui-theme="light"],
    [data-ui-theme="dark"] {{
        --ui-border: CanvasText;
        --ui-focus: Highlight;
    }}
    .ui-glass-surface,
    .ui-glass-layer,
    .ui-metric-card,
    .ui-empty-state,
    .ui-dialog-panel,
    .ui-command-menu-panel,
    .ui-popover,
    .ui-tooltip-content,
    .ui-sidebar,
    .ui-toast,
    .ui-alert,
    .ui-assistant-panel,
    .ui-prompt-input,
    .ui-surface {{
        background: Canvas;
        color: CanvasText;
        border-color: CanvasText;
        /* Drop translucent/blurred decoration so the system colors stay
           legible; the OS owns the contrast contract here. */
        backdrop-filter: none;
        -webkit-backdrop-filter: none;
        box-shadow: none;
    }}
    .ui-button:focus-visible,
    .ui-field-control:focus-visible,
    .ui-checkbox-input:focus-visible,
    .ui-switch-control:focus-visible,
    .ui-tab:focus-visible,
    .ui-command-menu-input:focus-visible,
    .ui-sidebar-link:focus-visible,
    .ui-icon-button:focus-visible,
    .ui-combobox-input:focus-visible,
    .ui-radio-input:focus-visible,
    .ui-datepicker-cell:focus-visible,
    .ui-dropdown-menu-button:focus-visible {{
        outline: 2px solid Highlight;
        outline-offset: 2px;
    }}
}}

/* Higher-contrast preference: strengthen our hairline borders and drop the
   backdrop blur so material surfaces read as crisp panels. */
@media (prefers-contrast: more) {{
    :root,
    [data-ui-theme="light"] {{
        --ui-border: rgba(60, 72, 90, 0.62);
    }}
    [data-ui-theme="dark"] {{
        --ui-border: rgba(220, 230, 244, 0.52);
    }}
    .ui-glass-surface,
    .ui-glass-layer,
    .ui-dialog-panel,
    .ui-command-menu-panel,
    .ui-popover,
    .ui-tooltip-content,
    .ui-assistant-panel,
    .ui-prompt-input {{
        backdrop-filter: blur(0) saturate(160%);
        -webkit-backdrop-filter: blur(0) saturate(160%);
    }}
}}
"#,
        l0 = LIGHT_ELEVATION.e0,
        l1 = LIGHT_ELEVATION.e1,
        l2 = LIGHT_ELEVATION.e2,
        l3 = LIGHT_ELEVATION.e3,
        d0 = DARK_ELEVATION.e0,
        d1 = DARK_ELEVATION.e1,
        d2 = DARK_ELEVATION.e2,
        d3 = DARK_ELEVATION.e3,
    )
}

pub const COMPONENT_CSS: &str = r#"
.ui-button,
.ui-field-control,
.ui-command-menu-input {
    min-height: var(--ui-control-height);
    border-radius: var(--ui-radius-md);
    transition: border-color var(--ui-motion-fast), box-shadow var(--ui-motion-fast), background var(--ui-motion-fast), transform var(--ui-motion-fast);
}

.ui-button {
    position: relative;
    overflow: hidden;
    border: 1px solid transparent;
    padding: 0 14px;
    font-weight: 700;
    cursor: pointer;
    will-change: transform;
}

.ui-button:hover:not(:disabled) {
    transform: translateY(-1px);
}

.ui-button:active:not(:disabled),
.ui-select-trigger:active:not([disabled]),
.ui-datepicker-trigger:active:not([disabled]),
.ui-segmented-option:active:not(:disabled),
.ui-pagination-button:active:not([disabled]) {
    transform: translateY(0) scale(var(--ui-press-scale));
    transition-duration: var(--ui-motion-press);
    transition-timing-function: var(--ui-ease-accelerate);
}

/* Specular sheen for filled buttons — a faint top-down highlight that reads
   as glossy Liquid-Glass material. `.ui-button` is a stacking context (it sets
   will-change: transform), so `z-index: -1` paints the sheen above the button
   background but BELOW the label text (which is normal in-flow content at the
   default z-index 0). pointer-events:none keeps it click-through. */
.ui-button--primary::before,
.ui-button--danger::before {
    content: "";
    position: absolute;
    inset: 0;
    z-index: -1;
    pointer-events: none;
    background: linear-gradient(180deg, rgba(255, 255, 255, 0.22), transparent 45%);
}

/* Visually-hidden utility — keeps content in the accessibility tree (screen
   readers, aria-live announcers) while removing it from the visual layout.
   Consumed by live-region count announcers (CommandMenu, Combobox), the
   AgentTimeline step state, Stepper, and navigation skip targets. */
.visually-hidden {
    position: absolute !important;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
}

.ui-button:disabled,
.ui-field-control:disabled,
.ui-checkbox-input:disabled,
.ui-switch-control[aria-disabled="true"] {
    cursor: not-allowed;
    opacity: 0.52;
}

.ui-button--primary {
    background: var(--ui-primary);
    color: #ffffff;
    box-shadow: 0 10px 22px color-mix(in srgb, var(--ui-primary), transparent 78%);
}

.ui-button--secondary {
    background: var(--ui-surface);
    color: var(--ui-fg);
    border-color: var(--ui-border);
}

.ui-button--ghost {
    background: transparent;
    color: var(--ui-fg);
}

.ui-button--danger {
    background: var(--ui-danger);
    color: var(--ui-on-danger);
}

.ui-surface,
.ui-glass-surface,
.ui-metric-card,
.ui-empty-state,
.ui-dialog-panel,
.ui-command-menu-panel,
.ui-sidebar {
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-surface);
    color: var(--ui-fg);
    box-shadow: var(--ui-elevation-0);
}

.ui-surface,
.ui-glass-surface {
    display: grid;
    gap: var(--ui-space-2);
    padding: var(--ui-space-4);
}

.ui-dialog-panel,
.ui-command-menu-panel {
    background: var(--ui-glass);
    backdrop-filter: blur(18px) saturate(160%);
    -webkit-backdrop-filter: blur(18px) saturate(160%);
}

.ui-glass-surface {
    /* Default tone (neutral) maps to the surface white; tone selectors below
       override `--ui-glass-tint` for the per-tone color. The SVG filter fallback
       path may reference --ui-glass-tint via computed styles so the variable
       must stay present even though the engine renders the actual blur/tint. */
    --ui-glass-tint: #ffffff;
    /* Translucent material: the glass tint floats over whatever is behind it,
       blurred + saturated, instead of an opaque tone wash. The opaque
       `var(--ui-glass-solid)` base only wins under the
       [data-ui-glass-policy="solid"] / [data-ui-transparency="reduced"]
       overrides below. */
    background: var(--ui-glass);
    backdrop-filter: blur(var(--ui-glass-blur, 18px)) saturate(var(--ui-glass-saturate, 160%));
    -webkit-backdrop-filter: blur(var(--ui-glass-blur, 18px)) saturate(var(--ui-glass-saturate, 160%));
    box-shadow: var(--ui-elevation-2), inset 0 1px 0.5px var(--ui-glass-highlight), inset 0 -1px 0.5px var(--ui-glass-highlight-bottom);
    border-color: var(--ui-border);
}

/* Tone: sets the dominant color of the glass material. The SVG filter path
   reads --ui-glass-tint from computed style, so keep these rules even though
   the engine path uses its own per-tone logic. The CSS-only render path
   (when GlassSurface skips the WebGL engine, e.g. `force_css: true`) also
   uses the tint as a soft background wash so tones stay visually
   distinguishable without backdrop-filter or a canvas. */
.ui-glass-surface[data-glass-tone="neutral"] { --ui-glass-tint: #ffffff; }
.ui-glass-surface[data-glass-tone="primary"] { --ui-glass-tint: var(--ui-primary); }
.ui-glass-surface[data-glass-tone="info"]    { --ui-glass-tint: var(--ui-info); }
.ui-glass-surface[data-glass-tone="success"] { --ui-glass-tint: var(--ui-success); }
.ui-glass-surface[data-glass-tone="warning"] { --ui-glass-tint: var(--ui-warning); }
.ui-glass-surface[data-glass-tone="danger"]  { --ui-glass-tint: var(--ui-danger); }

[data-ui-theme="dark"] .ui-glass-surface[data-glass-tone="neutral"] {
    --ui-glass-tint: #161c26;
}

/* Translucent tone wash. `color-mix` blends ~18% of the tone tint into the
   live --ui-glass material so each tone reads distinctly while STILL letting
   the backdrop show through (preserving translucency). The opaque-gradient
   fallback is reserved for [data-ui-glass-policy="solid"] /
   [data-ui-transparency="reduced"] below, which keep the surface fully
   readable when transparency is off. */
.ui-glass-surface[data-glass-tone="primary"] {
    background: color-mix(in srgb, var(--ui-glass-tint) 18%, var(--ui-glass));
}
.ui-glass-surface[data-glass-tone="info"] {
    background: color-mix(in srgb, var(--ui-glass-tint) 18%, var(--ui-glass));
}
.ui-glass-surface[data-glass-tone="success"] {
    background: color-mix(in srgb, var(--ui-glass-tint) 18%, var(--ui-glass));
}
.ui-glass-surface[data-glass-tone="warning"] {
    background: color-mix(in srgb, var(--ui-glass-tint) 18%, var(--ui-glass));
}
.ui-glass-surface[data-glass-tone="danger"] {
    background: color-mix(in srgb, var(--ui-glass-tint) 18%, var(--ui-glass));
}

/* Opaque tone fallback: only when the user prefers solid glass or reduced
   transparency. Keeps each tone distinguishable without a live backdrop. */
[data-ui-glass-policy="solid"] .ui-glass-surface[data-glass-tone="primary"],
[data-ui-transparency="reduced"] .ui-glass-surface[data-glass-tone="primary"],
[data-ui-glass-policy="solid"] .ui-glass-surface[data-glass-tone="info"],
[data-ui-transparency="reduced"] .ui-glass-surface[data-glass-tone="info"],
[data-ui-glass-policy="solid"] .ui-glass-surface[data-glass-tone="success"],
[data-ui-transparency="reduced"] .ui-glass-surface[data-glass-tone="success"],
[data-ui-glass-policy="solid"] .ui-glass-surface[data-glass-tone="warning"],
[data-ui-transparency="reduced"] .ui-glass-surface[data-glass-tone="warning"],
[data-ui-glass-policy="solid"] .ui-glass-surface[data-glass-tone="danger"],
[data-ui-transparency="reduced"] .ui-glass-surface[data-glass-tone="danger"] {
    background: linear-gradient(180deg,
        color-mix(in srgb, var(--ui-glass-tint) 22%, var(--ui-glass-solid)),
        var(--ui-glass-solid)) !important;
}

/* Level: maps to elevation depth. Subtle sits close to the page;
   Floating + Overlay + Chrome progressively lift off it. The wgpu engine
   computes its own shadow; these rules give the CSS render path a parallel
   visual contract so the 3-level showcase row is distinguishable. */
.ui-glass-surface[data-glass-level="subtle"]   { box-shadow: var(--ui-elevation-1); --ui-glass-blur: 12px; --ui-glass-saturate: 140%; }
.ui-glass-surface[data-glass-level="floating"] { box-shadow: var(--ui-elevation-2); --ui-glass-blur: 18px; --ui-glass-saturate: 160%; }
.ui-glass-surface[data-glass-level="overlay"]  { box-shadow: var(--ui-elevation-3); --ui-glass-blur: 28px; --ui-glass-saturate: 175%; }
.ui-glass-surface[data-glass-level="chrome"]   { box-shadow: var(--ui-elevation-3); --ui-glass-blur: 36px; --ui-glass-saturate: 190%; }

/* Density: padding rhythm inside the glass surface. */
.ui-glass-surface[data-glass-density="compact"] {
    padding: var(--ui-space-3);
}

.ui-glass-surface[data-glass-density="comfortable"] {
    padding: var(--ui-space-4);
}

.ui-glass-surface[data-glass-density="spacious"] {
    padding: var(--ui-space-5);
}

.ui-dialog-panel {
    box-shadow: var(--ui-elevation-3), inset 0 1px 0.5px var(--ui-glass-highlight), inset 0 -1px 0.5px var(--ui-glass-highlight-bottom);
}

.ui-command-menu-panel {
    box-shadow: var(--ui-elevation-2), inset 0 1px 0.5px var(--ui-glass-highlight), inset 0 -1px 0.5px var(--ui-glass-highlight-bottom);
}

.ui-stack {
    display: flex;
    flex-direction: column;
}

.ui-stack--gap-sm { gap: var(--ui-space-2); }
.ui-stack--gap-md { gap: var(--ui-space-3); }

.ui-form {
    display: flex;
    flex-direction: column;
    gap: var(--ui-space-4);
}

.ui-form--disabled {
    opacity: 0.6;
    pointer-events: none;
}

.ui-form-summary {
    margin: 0;
    padding: var(--ui-space-2) var(--ui-space-4);
    list-style: none;
    border: 1px solid var(--ui-danger);
    border-radius: var(--ui-radius-md);
    background: color-mix(in srgb, var(--ui-danger) 8%, transparent);
}

.ui-form-summary-item {
    color: var(--ui-danger);
    font-size: var(--ui-text-footnote);
}

.ui-file-input {
    display: grid;
    gap: var(--ui-space-2);
}

.ui-file-input-label,
.ui-dropzone-label {
    font-weight: 700;
}

.ui-file-input-control {
    width: 100%;
    border: 1px solid var(--ui-border);
    background: var(--ui-surface);
    color: var(--ui-fg);
    padding: var(--ui-space-2);
    border-radius: var(--ui-radius-md);
}

.ui-file-input-help {
    color: var(--ui-muted-fg);
}

.ui-dropzone {
    border: 1.5px dashed var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-surface-muted);
    padding: var(--ui-space-6);
    text-align: center;
}

.ui-dropzone[data-disabled="true"] {
    opacity: 0.6;
    pointer-events: none;
}

.ui-dropzone-region {
    display: flex;
    flex-direction: column;
    gap: var(--ui-space-1);
    cursor: pointer;
}

.ui-dropzone-region--dragover {
    outline: 2px solid var(--ui-focus);
    outline-offset: 4px;
    border-radius: var(--ui-radius-md);
}

.ui-dropzone-hint {
    color: var(--ui-muted-fg);
    font-size: var(--ui-text-footnote);
}

.ui-dropzone-input {
    position: absolute;
    width: 1px;
    height: 1px;
    opacity: 0;
    overflow: hidden;
    clip: rect(0 0 0 0);
}

.ui-attachment {
    display: inline-flex;
    align-items: center;
    gap: var(--ui-space-2);
    padding: var(--ui-space-1) var(--ui-space-3);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-full);
    background: var(--ui-surface-muted);
    font-size: var(--ui-text-footnote);
}

.ui-attachment-size {
    color: var(--ui-muted-fg);
}

.ui-attachment-remove {
    border: 0;
    background: transparent;
    color: var(--ui-muted-fg);
    cursor: pointer;
    line-height: 1;
    padding: 0 var(--ui-space-1);
}

.ui-attachment-remove:hover {
    color: var(--ui-danger);
}

.ui-tag-input {
    display: grid;
    gap: var(--ui-space-2);
}

.ui-tag-input-label {
    font-weight: 700;
}

.ui-tag-input-field {
    display: flex;
    flex-wrap: wrap;
    gap: var(--ui-space-2);
    align-items: center;
    padding: var(--ui-space-2);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
}

.ui-tag-input-chip {
    display: inline-flex;
    align-items: center;
    gap: var(--ui-space-1);
    padding: 0 var(--ui-space-2);
    border-radius: var(--ui-radius-full);
    background: var(--ui-surface-muted);
    font-size: var(--ui-text-footnote);
}

.ui-tag-input-chip-remove {
    border: 0;
    background: transparent;
    color: var(--ui-muted-fg);
    cursor: pointer;
    line-height: 1;
    padding: 0;
}

.ui-tag-input-chip-remove:hover {
    color: var(--ui-danger);
}

.ui-tag-input-control {
    flex: 1 1 8rem;
    min-width: 6rem;
    border: 0;
    background: transparent;
    color: var(--ui-fg);
    outline: none;
}

.ui-tag-input-help {
    color: var(--ui-muted-fg);
}

.ui-virtual-table-wrap {
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    overflow: auto;
}

.ui-virtual-table {
    width: 100%;
    border-collapse: collapse;
}

.ui-virtual-table-spacer td {
    padding: 0;
    border: 0;
}

.ui-virtual-table .ui-data-table-head th {
    position: sticky;
    top: 0;
    background: var(--ui-surface-muted);
    z-index: 1;
}

/* Auth surfaces --------------------------------------------------------- */

.ui-auth-card {
    display: grid;
    gap: var(--ui-space-4);
    padding: var(--ui-space-6);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-floating);
    background: var(--ui-surface);
}

.ui-auth-card-title {
    margin: 0;
    font-size: var(--ui-text-title2);
    font-weight: var(--ui-weight-bold);
}

.ui-auth-card-description {
    margin: 0;
    color: var(--ui-muted-fg);
}

.ui-auth-card-body {
    display: grid;
    gap: var(--ui-space-3);
}

.ui-oauth-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--ui-space-2);
    padding: 0 var(--ui-space-4);
    height: var(--ui-control-height);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    color: var(--ui-fg);
    cursor: pointer;
}

.ui-oauth-button:hover {
    background: var(--ui-surface-muted);
}

.ui-password-strength {
    display: grid;
    gap: var(--ui-space-1);
}

.ui-password-strength-bars {
    display: flex;
    gap: var(--ui-space-1);
}

.ui-password-strength-bar {
    flex: 1;
    height: 4px;
    border-radius: var(--ui-radius-full);
    background: var(--ui-border);
}

.ui-password-strength-bar--on { background: currentColor; }
.ui-password-strength--weak { color: var(--ui-danger); }
.ui-password-strength--fair { color: var(--ui-warning); }
.ui-password-strength--good { color: var(--ui-info); }
.ui-password-strength--strong { color: var(--ui-success); }

.ui-password-strength-label {
    font-size: var(--ui-text-caption2);
    color: var(--ui-muted-fg);
    text-transform: capitalize;
}

.ui-mfa-code-cells {
    display: flex;
    gap: var(--ui-space-2);
}

.ui-mfa-code-cell {
    width: 2.5rem;
    height: 3rem;
    text-align: center;
    font-size: var(--ui-text-title3);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    color: var(--ui-fg);
}

/* Billing surfaces ------------------------------------------------------ */

.ui-pricing-table {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: var(--ui-space-4);
}

.ui-plan-card {
    display: grid;
    gap: var(--ui-space-4);
    padding: var(--ui-space-5);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-surface);
}

.ui-plan-card--featured {
    border-color: var(--ui-primary);
    box-shadow: var(--ui-shadow-soft);
}

.ui-plan-card-name {
    margin: 0;
    font-size: var(--ui-text-title3);
    font-weight: var(--ui-weight-semibold);
}

.ui-plan-card-price {
    font-size: var(--ui-text-title1);
    font-weight: var(--ui-weight-bold);
}

.ui-plan-card-period {
    color: var(--ui-muted-fg);
}

.ui-plan-card-features {
    margin: 0;
    padding-left: var(--ui-space-4);
    display: grid;
    gap: var(--ui-space-1);
    color: var(--ui-fg);
}

.ui-plan-card-cta {
    height: var(--ui-control-height);
    border-radius: var(--ui-radius-md);
    cursor: pointer;
}

.ui-plan-card-cta--primary {
    border: 0;
    background: var(--ui-primary);
    color: var(--ui-on-accent);
}

.ui-plan-card-cta--ghost {
    border: 1px solid var(--ui-border);
    background: transparent;
    color: var(--ui-fg);
}

.ui-usage-meter {
    display: grid;
    gap: var(--ui-space-1);
}

.ui-usage-meter-head {
    display: flex;
    justify-content: space-between;
    font-size: var(--ui-text-footnote);
}

.ui-usage-meter-readout {
    color: var(--ui-muted-fg);
    font-variant-numeric: tabular-nums;
}

.ui-usage-meter-track {
    height: 8px;
    border-radius: var(--ui-radius-full);
    background: var(--ui-surface-muted);
    overflow: hidden;
}

.ui-usage-meter-bar {
    height: 100%;
    border-radius: inherit;
    background: var(--ui-primary);
}

.ui-usage-meter--warning .ui-usage-meter-bar { background: var(--ui-warning); }
.ui-usage-meter--critical .ui-usage-meter-bar { background: var(--ui-danger); }

.ui-invoice-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: grid;
    gap: var(--ui-space-1);
}

.ui-invoice-row {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr auto;
    gap: var(--ui-space-3);
    align-items: center;
    padding: var(--ui-space-2) var(--ui-space-3);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    font-size: var(--ui-text-footnote);
}

.ui-invoice-status {
    padding: 0 var(--ui-space-2);
    border-radius: var(--ui-radius-full);
    font-size: var(--ui-text-caption2);
    font-weight: var(--ui-weight-semibold);
}

.ui-invoice-status--paid { background: color-mix(in srgb, var(--ui-success) 16%, transparent); color: var(--ui-success); }
.ui-invoice-status--due { background: color-mix(in srgb, var(--ui-info) 16%, transparent); color: var(--ui-info); }
.ui-invoice-status--overdue { background: color-mix(in srgb, var(--ui-danger) 16%, transparent); color: var(--ui-danger); }
.ui-invoice-status--draft { background: var(--ui-surface-muted); color: var(--ui-muted-fg); }

.ui-text-field,
.ui-checkbox,
.ui-switch,
.ui-tabs,
.ui-toolbar,
.ui-sidebar,
.ui-metric-card,
.ui-empty-state,
.ui-toast,
.ui-command-menu,
.ui-tooltip {
    color: var(--ui-fg);
}

.ui-field {
    display: grid;
    gap: var(--ui-space-2);
}

.ui-field-row {
    display: flex;
    align-items: stretch;
}

.ui-field-label,
.ui-checkbox-label,
.ui-switch-label {
    font-weight: 700;
}

.ui-field-control,
.ui-command-menu-input {
    width: 100%;
    border: 1px solid var(--ui-border);
    background: var(--ui-surface);
    color: var(--ui-fg);
    padding: 0 12px;
}

.ui-field-adornment {
    display: grid;
    align-items: center;
    border: 1px solid var(--ui-border);
    background: var(--ui-surface-muted);
    color: var(--ui-muted-fg);
    padding: 0 10px;
}

.ui-field-adornment--leading {
    border-right: 0;
    border-radius: var(--ui-radius-md) 0 0 var(--ui-radius-md);
}

.ui-field-adornment--trailing {
    border-left: 0;
    border-radius: 0 var(--ui-radius-md) var(--ui-radius-md) 0;
}

.ui-field-row .ui-field-adornment--leading + .ui-field-control {
    border-radius: 0 var(--ui-radius-md) var(--ui-radius-md) 0;
}

.ui-field-control:focus-visible,
.ui-checkbox-input:focus-visible,
.ui-switch-control:focus-visible,
.ui-tab:focus-visible,
.ui-command-menu-input:focus-visible,
.ui-sidebar-link:focus-visible,
.ui-button:focus-visible {
    outline: 2px solid var(--ui-focus);
    outline-offset: 2px;
}

.ui-field--invalid .ui-field-control {
    border-color: var(--ui-danger);
}

.ui-field-help,
.ui-field-error,
.ui-checkbox-description,
.ui-switch-description,
.ui-empty-state-description,
.ui-metric-card-delta,
.ui-toast-description,
.ui-command-menu-item span,
.ui-dialog-description {
    color: var(--ui-muted-fg);
}

.ui-field-error {
    color: var(--ui-danger);
}

.ui-checkbox,
.ui-switch {
    display: flex;
    gap: var(--ui-space-3);
    align-items: flex-start;
}

.ui-checkbox-input {
    width: 18px;
    height: 18px;
    accent-color: var(--ui-primary);
}

.ui-checkbox--mixed .ui-checkbox-input {
    box-shadow: inset 0 0 0 2px var(--ui-primary);
}

.ui-switch-control {
    position: relative;
    width: 42px;
    height: 24px;
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-full);
    background: var(--ui-surface-muted);
    transition: background var(--ui-motion-fast);
}

.ui-switch-control[aria-checked="true"] {
    background: var(--ui-primary);
}

.ui-switch-thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 18px;
    height: 18px;
    border-radius: var(--ui-radius-full);
    background: #ffffff;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.20);
    transition: transform var(--ui-motion-normal) var(--ui-ease-spring), width var(--ui-motion-fast) var(--ui-ease-spring);
}

.ui-switch-control[aria-checked="true"] .ui-switch-thumb {
    transform: translateX(18px);
}

/* Press feedback: the thumb stretches slightly while the control is held,
   echoing the iOS toggle. */
.ui-switch-control:active .ui-switch-thumb {
    width: 22px;
}

.ui-tabs-list,
.ui-toolbar,
.ui-command-menu-list {
    display: flex;
    gap: var(--ui-space-2);
}

.ui-tabs {
    display: grid;
    gap: var(--ui-space-3);
}

.ui-tab {
    border: 0;
    border-radius: var(--ui-radius-md);
    background: transparent;
    color: var(--ui-muted-fg);
    padding: 8px 10px;
}

.ui-tab[aria-selected="true"] {
    background: var(--ui-surface);
    color: var(--ui-fg);
    box-shadow: inset 0 0 0 1px var(--ui-border);
}

.ui-tab-panel {
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface-muted);
    padding: var(--ui-space-4);
}

.ui-dialog {
    position: fixed;
    inset: 0;
    display: grid;
    place-items: center;
    padding: var(--ui-space-5);
}

.ui-dialog-backdrop {
    position: absolute;
    inset: 0;
    background: rgba(10, 15, 24, 0.38);
}

.ui-dialog-panel,
.ui-command-menu-panel {
    position: relative;
    width: min(560px, 100%);
    padding: var(--ui-space-5);
}

.ui-dialog-actions,
.ui-toast-actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--ui-space-2);
}

.ui-toast {
    display: flex;
    justify-content: space-between;
    gap: var(--ui-space-3);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-surface);
    padding: var(--ui-space-4);
    box-shadow: var(--ui-elevation-2);
}

.ui-toast--success { border-color: color-mix(in srgb, var(--ui-success), transparent 62%); }
.ui-toast--warning { border-color: color-mix(in srgb, var(--ui-warning), transparent 62%); }
.ui-toast--danger { border-color: color-mix(in srgb, var(--ui-danger), transparent 62%); }
.ui-toast--info { border-color: color-mix(in srgb, var(--ui-info), transparent 62%); }

.ui-alert {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--ui-space-3);
    border: 1px solid var(--ui-border);
    border-left-width: 4px;
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    padding: var(--ui-space-3) var(--ui-space-4);
}

.ui-alert--neutral { border-left-color: var(--ui-border); }
.ui-alert--success { border-left-color: var(--ui-success); background: color-mix(in srgb, var(--ui-success), transparent 90%); }
.ui-alert--warning { border-left-color: var(--ui-warning); background: color-mix(in srgb, var(--ui-warning), transparent 90%); }
.ui-alert--danger  { border-left-color: var(--ui-danger);  background: color-mix(in srgb, var(--ui-danger),  transparent 90%); }
.ui-alert--info    { border-left-color: var(--ui-info);    background: color-mix(in srgb, var(--ui-info),    transparent 92%); }

.ui-alert-content {
    display: grid;
    gap: var(--ui-space-1);
    flex: 1;
}

.ui-alert-title {
    color: var(--ui-fg);
    font-weight: 700;
}

.ui-alert-description {
    margin: 0;
    color: var(--ui-muted-fg);
    line-height: 1.45;
}

.ui-alert-dismiss {
    align-self: flex-start;
}

.ui-progress {
    display: grid;
    gap: var(--ui-space-1);
    width: 100%;
}

.ui-progress-label {
    color: var(--ui-fg);
    font-weight: var(--ui-weight-semibold);
    font-size: var(--ui-text-footnote);
    line-height: var(--ui-leading-footnote);
    font-variant-numeric: tabular-nums;
    font-feature-settings: 'tnum' 1;
}

.ui-progress-track {
    position: relative;
    height: 8px;
    border-radius: var(--ui-radius-full);
    background: var(--ui-surface-muted);
    overflow: hidden;
}

.ui-progress-fill {
    height: 100%;
    border-radius: inherit;
    background: var(--ui-accent);
    transition: width 200ms ease-out;
}

.ui-progress--indeterminate .ui-progress-fill {
    width: 33%;
    animation: ui-progress-slide 1200ms ease-in-out infinite;
}

.ui-progress-description {
    margin: 0;
    color: var(--ui-muted-fg);
    font-size: var(--ui-text-caption);
    line-height: var(--ui-leading-caption);
}

@keyframes ui-progress-slide {
    0%   { transform: translateX(-100%); }
    100% { transform: translateX(300%); }
}

@media (prefers-reduced-motion: reduce) {
    .ui-progress--indeterminate .ui-progress-fill {
        animation: none;
        width: 100%;
        opacity: 0.4;
    }
}

[data-ui-motion="reduced"] .ui-progress--indeterminate .ui-progress-fill {
    animation: none;
    width: 100%;
    opacity: 0.4;
}

.ui-skeleton {
    background: var(--ui-surface-muted);
    background-image: linear-gradient(
        90deg,
        var(--ui-surface-muted) 0%,
        color-mix(in srgb, var(--ui-surface-muted), transparent 30%) 50%,
        var(--ui-surface-muted) 100%
    );
    background-size: 200% 100%;
    animation: ui-skeleton-shimmer 1400ms ease-in-out infinite;
}

@keyframes ui-skeleton-shimmer {
    0%   { background-position: 200% 0; }
    100% { background-position: -200% 0; }
}

@media (prefers-reduced-motion: reduce) {
    .ui-skeleton { animation: none; }
}

[data-ui-motion="reduced"] .ui-skeleton { animation: none; }

.ui-breadcrumb-list,
.ui-pagination-list {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--ui-space-2);
    margin: 0;
    padding: 0;
    list-style: none;
}

.ui-breadcrumb-link {
    color: var(--ui-muted-fg);
    text-decoration: none;
}

.ui-breadcrumb-link:hover { color: var(--ui-fg); text-decoration: underline; }

.ui-breadcrumb-sep {
    color: var(--ui-muted-fg);
    margin: 0 var(--ui-space-1);
}

.ui-breadcrumb-current {
    color: var(--ui-fg);
    font-weight: 600;
}

.ui-pagination-button {
    min-width: 32px;
    padding: 4px 10px;
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    color: var(--ui-fg);
    cursor: pointer;
    font: inherit;
    font-variant-numeric: tabular-nums;
    font-feature-settings: 'tnum' 1;
    transition: transform var(--ui-motion-fast), border-color var(--ui-motion-fast), background var(--ui-motion-fast);
}

.ui-pagination-button[disabled] { opacity: 0.45; cursor: not-allowed; }

.ui-pagination-button--current {
    background: var(--ui-accent);
    color: var(--ui-on-accent, #fff);
    border-color: var(--ui-accent);
}

.ui-pagination-item--ellipsis {
    color: var(--ui-muted-fg);
    padding: 0 var(--ui-space-1);
}

.ui-stepper-list {
    display: flex;
    flex-direction: row;
    gap: var(--ui-space-3);
    margin: 0;
    padding: 0;
    list-style: none;
}

.ui-stepper--vertical .ui-stepper-list { flex-direction: column; }

.ui-stepper-trigger {
    display: inline-flex;
    align-items: center;
    gap: var(--ui-space-2);
    background: none;
    border: none;
    padding: 4px 8px;
    cursor: pointer;
    color: var(--ui-muted-fg);
    font: inherit;
    text-align: left;
}

.ui-stepper-marker {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background: var(--ui-surface-muted);
    color: var(--ui-fg);
    font-weight: 700;
}

.ui-stepper-step--active .ui-stepper-trigger { color: var(--ui-fg); }
.ui-stepper-step--active .ui-stepper-marker {
    background: var(--ui-accent);
    color: var(--ui-on-accent, #fff);
}
.ui-stepper-step--complete .ui-stepper-marker {
    background: color-mix(in srgb, var(--ui-success), transparent 70%);
    color: var(--ui-success);
}

.ui-stepper-body { display: grid; gap: var(--ui-space-0); }
.ui-stepper-label { font-weight: var(--ui-weight-semibold); }
.ui-stepper-description { color: var(--ui-muted-fg); font-size: var(--ui-text-caption); line-height: var(--ui-leading-caption); }

.ui-segmented {
    display: inline-flex;
    padding: 2px;
    background: var(--ui-surface-muted);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
}

.ui-segmented-option {
    background: transparent;
    border: none;
    color: var(--ui-muted-fg);
    padding: 4px 12px;
    border-radius: calc(var(--ui-radius-md) - 2px);
    cursor: pointer;
    font: inherit;
    transition: transform var(--ui-motion-fast), background var(--ui-motion-fast), color var(--ui-motion-fast);
}

.ui-segmented-option--selected {
    background: var(--ui-surface);
    color: var(--ui-fg);
    box-shadow: var(--ui-elevation-1);
    font-weight: var(--ui-weight-semibold);
}

/* Opt-in tabular-figures utility for any numeric surface. */
.ui-tabular {
    font-variant-numeric: tabular-nums;
    font-feature-settings: 'tnum' 1;
}

.ui-accordion {
    display: grid;
    gap: var(--ui-space-2);
}

.ui-accordion-section {
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    overflow: hidden;
}

.ui-accordion-heading { margin: 0; }

.ui-accordion-trigger {
    width: 100%;
    display: flex;
    align-items: center;
    gap: var(--ui-space-2);
    padding: var(--ui-space-3) var(--ui-space-4);
    background: none;
    border: none;
    color: var(--ui-fg);
    font: inherit;
    font-weight: 600;
    text-align: left;
    cursor: pointer;
}

.ui-accordion-trigger[disabled] { opacity: 0.45; cursor: not-allowed; }

.ui-accordion-marker {
    width: 18px;
    height: 18px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--ui-muted-fg);
    font-weight: 800;
}

.ui-accordion-region {
    padding: 0 var(--ui-space-4) var(--ui-space-3);
    color: var(--ui-muted-fg);
    line-height: 1.45;
}

.ui-slider {
    display: grid;
    gap: var(--ui-space-1);
}

.ui-slider-label {
    color: var(--ui-fg);
    font-weight: var(--ui-weight-semibold);
    font-size: var(--ui-text-footnote);
    line-height: var(--ui-leading-footnote);
}

.ui-slider-input {
    width: 100%;
    accent-color: var(--ui-accent);
    cursor: pointer;
}

.ui-slider-input[disabled] {
    cursor: not-allowed;
    opacity: 0.45;
}

.ui-slider-description {
    margin: 0;
    color: var(--ui-muted-fg);
    font-size: var(--ui-text-caption);
    line-height: var(--ui-leading-caption);
}

.ui-popover-root {
    position: relative;
    display: inline-block;
}

.ui-popover {
    position: absolute;
    z-index: 50;
    min-width: 220px;
    padding: var(--ui-space-3);
    background: var(--ui-glass);
    backdrop-filter: blur(12px) saturate(160%);
    -webkit-backdrop-filter: blur(12px) saturate(160%);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-floating);
    box-shadow: var(--ui-elevation-3), inset 0 1px 0.5px var(--ui-glass-highlight), inset 0 -1px 0.5px var(--ui-glass-highlight-bottom);
}

.ui-popover--bottom { top: calc(100% + 6px); left: 0; }
.ui-popover--top    { bottom: calc(100% + 6px); left: 0; }
.ui-popover--end    { left: calc(100% + 6px); top: 0; }
.ui-popover--start  { right: calc(100% + 6px); top: 0; }

.ui-popover-trigger {
    display: inline-flex;
}

.ui-select {
    display: grid;
    gap: var(--ui-space-1);
}

.ui-select-label {
    color: var(--ui-fg);
    font-weight: var(--ui-weight-semibold);
    font-size: var(--ui-text-footnote);
    line-height: var(--ui-leading-footnote);
}

.ui-select-trigger {
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--ui-space-2);
    min-width: 220px;
    padding: 6px 10px;
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    color: var(--ui-fg);
    cursor: pointer;
    font: inherit;
    transition: transform var(--ui-motion-fast), border-color var(--ui-motion-fast), background var(--ui-motion-fast);
}

.ui-select-trigger[disabled] { opacity: 0.45; cursor: not-allowed; }
.ui-select-trigger--placeholder { color: var(--ui-muted-fg); }
.ui-select-chevron { color: var(--ui-muted-fg); }

.ui-select-listbox {
    margin: 0;
    padding: 4px;
    list-style: none;
    min-width: 220px;
    max-height: 280px;
    overflow-y: auto;
}

.ui-select-option {
    padding: 6px 10px;
    border-radius: calc(var(--ui-radius-md) - 2px);
    cursor: pointer;
    color: var(--ui-fg);
}

.ui-select-option:hover { background: var(--ui-surface-muted); }
.ui-select-option--selected { background: var(--ui-surface-muted); font-weight: var(--ui-weight-semibold); }
.ui-select-option--disabled { color: var(--ui-muted-fg); cursor: not-allowed; }

/* Keyboard "active descendant" highlight. `data-active="true"` is emitted by
   the Wave-2 keyboard navigation engines; the inset bar + tinted wash gives
   the focused row a clear, low-contrast emphasis distinct from hover/selected. */
.ui-select-option[data-active="true"],
.ui-command-menu-item[data-active="true"] {
    box-shadow: inset 2px 0 0 var(--ui-focus);
    background: color-mix(in srgb, var(--ui-focus), transparent 90%);
}

.ui-datepicker {
    display: grid;
    gap: var(--ui-space-1);
}

.ui-datepicker-label {
    color: var(--ui-fg);
    font-weight: var(--ui-weight-semibold);
    font-size: var(--ui-text-footnote);
    line-height: var(--ui-leading-footnote);
}

.ui-datepicker-trigger {
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--ui-space-2);
    min-width: 200px;
    padding: 6px 10px;
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    color: var(--ui-fg);
    cursor: pointer;
    font: inherit;
    transition: transform var(--ui-motion-fast), border-color var(--ui-motion-fast), background var(--ui-motion-fast);
}

.ui-datepicker-trigger[disabled] { opacity: 0.45; cursor: not-allowed; }
.ui-datepicker-trigger--placeholder { color: var(--ui-muted-fg); }
.ui-datepicker-icon { color: var(--ui-muted-fg); }

.ui-datepicker-calendar {
    display: grid;
    gap: var(--ui-space-3);
    min-width: 280px;
    padding: var(--ui-space-2);
}

.ui-datepicker-nav {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--ui-space-2);
}

.ui-datepicker-nav-button {
    border: 1px solid transparent;
    background: transparent;
    color: var(--ui-fg);
    border-radius: var(--ui-radius-md);
    padding: 4px 10px;
    cursor: pointer;
    font: inherit;
    line-height: 1;
}

.ui-datepicker-nav-button:hover { background: var(--ui-surface-muted); }

.ui-datepicker-title {
    font-size: var(--ui-text-subhead);
    line-height: var(--ui-leading-subhead);
    color: var(--ui-fg);
}

.ui-datepicker-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: var(--ui-space-0);
}

.ui-datepicker-weekday {
    font-size: var(--ui-text-caption2);
    line-height: var(--ui-leading-caption2);
    text-align: center;
    color: var(--ui-muted-fg);
    padding: 4px 0;
    font-weight: var(--ui-weight-bold);
    text-transform: uppercase;
    font-variant-numeric: tabular-nums;
    font-feature-settings: 'tnum' 1;
}

.ui-datepicker-cell {
    aspect-ratio: 1 / 1;
    border: 1px solid transparent;
    background: transparent;
    color: var(--ui-fg);
    border-radius: var(--ui-radius-md);
    cursor: pointer;
    font: inherit;
    display: flex;
    align-items: center;
    justify-content: center;
    font-variant-numeric: tabular-nums;
    font-feature-settings: 'tnum' 1;
}

.ui-datepicker-cell:hover { background: var(--ui-surface-muted); }
.ui-datepicker-cell:focus-visible {
    outline: 2px solid var(--ui-focus);
    outline-offset: 0;
}
.ui-datepicker-cell--selected {
    background: var(--ui-fg);
    color: var(--ui-bg);
    font-weight: var(--ui-weight-bold);
}
.ui-datepicker-cell--empty {
    background: transparent;
    cursor: default;
    pointer-events: none;
}

.ui-data-table {
    width: 100%;
    border-collapse: collapse;
    font: inherit;
    color: var(--ui-fg);
}

.ui-data-table-caption {
    text-align: left;
    color: var(--ui-muted-fg);
    font-size: var(--ui-text-caption);
    line-height: var(--ui-leading-caption);
    padding-bottom: var(--ui-space-2);
}

.ui-data-table-th {
    text-align: left;
    padding: 8px 10px;
    border-bottom: 1px solid var(--ui-border);
    font-weight: var(--ui-weight-bold);
    font-size: var(--ui-text-footnote);
    line-height: var(--ui-leading-footnote);
    color: var(--ui-fg);
}

.ui-data-table-th--sortable {
    padding: 0;
}

.ui-data-table-sort-button {
    width: 100%;
    background: transparent;
    border: none;
    padding: 8px 10px;
    color: inherit;
    font: inherit;
    font-weight: 700;
    text-align: left;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--ui-space-2);
}

.ui-data-table-sort-button:hover { background: var(--ui-surface-muted); }
.ui-data-table-sort-indicator { color: var(--ui-muted-fg); font-size: var(--ui-text-caption); line-height: var(--ui-leading-caption); }

.ui-data-table-cell {
    padding: 8px 10px;
    border-bottom: 1px solid var(--ui-border);
    font-size: var(--ui-text-subhead);
    line-height: var(--ui-leading-subhead);
    vertical-align: top;
    font-variant-numeric: tabular-nums;
    font-feature-settings: 'tnum' 1;
}

.ui-data-table-row:hover { background: var(--ui-surface-muted); }
.ui-data-table-row:last-child .ui-data-table-cell { border-bottom: 0; }

.ui-command-menu {
    display: grid;
    gap: var(--ui-space-3);
}

.ui-command-menu-list {
    flex-direction: column;
}

.ui-command-menu-group {
    display: grid;
    gap: var(--ui-space-1);
}

.ui-command-menu-group-label {
    margin: 0;
    color: var(--ui-muted-fg);
    font-size: var(--ui-text-caption);
    line-height: var(--ui-leading-caption);
    letter-spacing: var(--ui-tracking-caption);
    font-weight: 800;
    text-transform: uppercase;
}

.ui-command-menu-item {
    display: grid;
    gap: var(--ui-space-1);
    border-radius: var(--ui-radius-md);
    padding: 8px 10px;
}

.ui-command-menu-item[aria-selected="true"],
.ui-sidebar-link[aria-current="page"] {
    background: var(--ui-surface-muted);
    color: var(--ui-fg);
}

/* Forward-declared (Wave 2): icon + shortcut affordances inside a command row.
   Lay the row out as [icon] [label/desc] [shortcut] when these are present. */
.ui-command-menu-item:has(.ui-command-menu-item-icon),
.ui-command-menu-item:has(.ui-command-menu-item-shortcut) {
    grid-template-columns: auto 1fr auto;
    align-items: center;
    column-gap: var(--ui-space-3);
}

.ui-command-menu-item-icon {
    display: inline-grid;
    place-items: center;
    width: 18px;
    height: 18px;
    color: var(--ui-muted-fg);
}

.ui-command-menu-item-shortcut {
    display: inline-flex;
    gap: 2px;
    justify-self: end;
    font-size: var(--ui-text-caption);
    color: var(--ui-muted-fg);
    font-variant-numeric: tabular-nums;
}

.ui-command-menu-item-shortcut kbd {
    min-width: 18px;
    padding: 1px 5px;
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-sm);
    background: var(--ui-surface-muted);
    color: var(--ui-fg);
    font: inherit;
    font-size: var(--ui-text-caption2);
    line-height: 1.4;
    text-align: center;
}

.ui-tooltip-content {
    border-radius: var(--ui-radius-md);
    background: var(--ui-glass);
    backdrop-filter: blur(12px) saturate(160%);
    -webkit-backdrop-filter: blur(12px) saturate(160%);
    color: var(--ui-fg);
    border: 1px solid var(--ui-border);
    padding: 6px 8px;
    font-size: var(--ui-text-footnote);
    line-height: var(--ui-leading-footnote);
    box-shadow: var(--ui-elevation-1), inset 0 1px 0.5px var(--ui-glass-highlight), inset 0 -1px 0.5px var(--ui-glass-highlight-bottom);
}

.ui-toolbar {
    align-items: center;
    justify-content: space-between;
}

.ui-toolbar-group {
    display: flex;
    gap: var(--ui-space-2);
    align-items: center;
}

.ui-sidebar {
    display: grid;
    gap: var(--ui-space-4);
    padding: var(--ui-space-4);
}

.ui-sidebar-section {
    display: grid;
    gap: var(--ui-space-2);
}

.ui-sidebar-section-label {
    margin: 0;
    color: var(--ui-muted-fg);
    font-size: var(--ui-text-caption);
    line-height: var(--ui-leading-caption);
    letter-spacing: var(--ui-tracking-caption);
    font-weight: 800;
    text-transform: uppercase;
}

.ui-sidebar-link {
    border-radius: var(--ui-radius-md);
    color: var(--ui-muted-fg);
    padding: 8px 10px;
    text-decoration: none;
}

.ui-metric-card,
.ui-empty-state {
    display: grid;
    gap: var(--ui-space-3);
    padding: var(--ui-space-4);
}

.ui-metric-card {
    box-shadow: var(--ui-elevation-1);
}

.ui-metric-card-value {
    font-size: var(--ui-text-title1);
    line-height: var(--ui-leading-title1);
    letter-spacing: var(--ui-tracking-title1);
    font-weight: var(--ui-weight-bold);
    font-variant-numeric: tabular-nums;
    font-feature-settings: 'tnum' 1;
}

.ui-metric-card--success .ui-metric-card-delta { color: var(--ui-success); }
.ui-metric-card--warning .ui-metric-card-delta { color: var(--ui-warning); }
.ui-metric-card--danger .ui-metric-card-delta { color: var(--ui-danger); }
.ui-metric-card--info .ui-metric-card-delta { color: var(--ui-info); }

.ui-metric-card-sparkline {
    height: 34px;
    border-radius: var(--ui-radius-md);
    background: linear-gradient(135deg, color-mix(in srgb, var(--ui-primary), transparent 70%), transparent);
}

.ui-empty-state {
    justify-items: start;
}

.ui-empty-state-visual {
    width: 42px;
    height: 42px;
    border-radius: var(--ui-radius-lg);
    background: linear-gradient(135deg, var(--ui-primary), var(--ui-info));
}

.ui-glass-layer {
    background: var(--ui-material-bg, var(--ui-glass));
    border: 1px solid var(--ui-material-border, var(--ui-border));
    border-radius: var(--ui-radius-lg);
    box-shadow: var(--ui-material-shadow, var(--ui-elevation-2)), inset 0 1px 0.5px var(--ui-glass-highlight), inset 0 -1px 0.5px var(--ui-glass-highlight-bottom);
    backdrop-filter: blur(var(--ui-material-blur, 18px)) saturate(var(--ui-material-saturate, 160%));
    -webkit-backdrop-filter: blur(var(--ui-material-blur, 18px)) saturate(var(--ui-material-saturate, 160%));
}

.ui-timeline-scope,
.ui-presence-gate {
    display: grid;
    gap: var(--ui-space-3);
}

.ui-kinetic-box,
.ui-kinetic-text,
.ui-frame-layer {
    transition: opacity var(--ui-motion-normal), transform var(--ui-motion-normal);
}

/* Standalone KineticBoxes (no parent Sequence/Timeline driving them via an
   inline `style`) animate themselves on mount using a CSS keyframe matching
   their cue. The `:not([style])` guard means that when a Sequence supplies an
   inline transform/opacity, the JS-driven values win and the keyframe is
   skipped. */
@keyframes kinetic-rise-in {
    from { opacity: 0; transform: translateY(8px); }
    to   { opacity: 1; transform: translateY(0); }
}

@keyframes kinetic-fade-in {
    from { opacity: 0; }
    to   { opacity: 1; }
}

@keyframes kinetic-slide-up {
    from { opacity: 0; transform: translateY(16px); }
    to   { opacity: 1; transform: translateY(0); }
}

.ui-kinetic-box[data-motion-cue="rise-in"]:not([style*="opacity"]):not([style*="transform"]) {
    animation: kinetic-rise-in 320ms ease both;
}

.ui-kinetic-box[data-motion-cue="fade-in"]:not([style*="opacity"]):not([style*="transform"]) {
    animation: kinetic-fade-in 320ms ease both;
}

.ui-kinetic-box[data-motion-cue="slide-up"]:not([style*="opacity"]):not([style*="transform"]) {
    animation: kinetic-slide-up 320ms ease both;
}

/* Inside a TimelineScope, autoplay=false means an external trigger (slider,
   intersection observer, etc.) drives the cues; suppress the on-mount CSS
   animation so the scope stays static until explicitly played. */
.ui-timeline-scope[data-autoplay="false"] .ui-kinetic-box {
    animation: none !important;
}

/* Stagger autoplay'd KineticBoxes by their wrapper's data-stagger-index so a
   row of boxes ripple in one after another instead of firing simultaneously. */
.ui-timeline-scope[data-autoplay="true"] [data-stagger-index="0"] .ui-kinetic-box {
    animation-delay: 0ms;
}
.ui-timeline-scope[data-autoplay="true"] [data-stagger-index="1"] .ui-kinetic-box {
    animation-delay: 80ms;
}
.ui-timeline-scope[data-autoplay="true"] [data-stagger-index="2"] .ui-kinetic-box {
    animation-delay: 160ms;
}
.ui-timeline-scope[data-autoplay="true"] [data-stagger-index="3"] .ui-kinetic-box {
    animation-delay: 240ms;
}

.ui-frame-stage,
.ui-capture-stage {
    position: relative;
    overflow: hidden;
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-surface);
}

.ui-frame-clip {
    display: contents;
}

.ui-icon-button {
    position: relative;
    display: inline-grid;
    place-items: center;
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    color: var(--ui-fg);
    cursor: pointer;
    transition: background var(--ui-motion-fast), border-color var(--ui-motion-fast), transform var(--ui-motion-fast);
}

/* Hit-slop: the painted chrome stays small (compact/default/spacious sizes)
   but the interactive target is expanded to the 44px Apple HIG minimum via a
   transparent centered overlay. It is pointer-events transparent so clicks
   land on the button; the glyph (.ui-icon-button-glyph) sits above it. */
.ui-icon-button::before {
    content: "";
    position: absolute;
    left: 50%;
    top: 50%;
    width: 44px;
    height: 44px;
    transform: translate(-50%, -50%);
}

.ui-icon-button:hover:not(:disabled) {
    background: var(--ui-surface-muted);
    transform: translateY(-1px);
}

.ui-icon-button:active:not(:disabled) {
    transform: translateY(0);
}

.ui-icon-button:focus-visible {
    outline: 2px solid var(--ui-focus);
    outline-offset: 2px;
}

.ui-icon-button:disabled {
    opacity: 0.52;
    cursor: not-allowed;
}

.ui-icon-button--neutral { color: var(--ui-fg); }
.ui-icon-button--primary { color: var(--ui-primary); }
.ui-icon-button--danger { color: var(--ui-danger); }

.ui-icon-button--compact { width: 28px; height: 28px; }
.ui-icon-button--default { width: 32px; height: 32px; }
.ui-icon-button--spacious { width: 40px; height: 40px; }

.ui-icon-button-glyph {
    position: relative;
    z-index: 1;
    display: grid;
    place-items: center;
    pointer-events: none;
}

.ui-presence {
    --ui-presence-t: 1;
    display: contents;
}

/* `.ui-presence` uses `display: contents` so the wrapper does not contribute
   a layout box. Opacity and transform on a `display: contents` element have
   no effect (there is no box to transform), so each presence cue applies its
   visual to direct children instead. */
.ui-presence[data-presence-cue="fade"] > * {
    opacity: var(--ui-presence-t);
    transition: opacity var(--ui-motion-normal);
}

.ui-presence[data-presence-cue="rise"] > * {
    opacity: var(--ui-presence-t);
    transform: translateY(calc((1 - var(--ui-presence-t)) * 8px));
    transition: opacity var(--ui-motion-normal), transform var(--ui-motion-normal);
}

.ui-presence[data-presence-cue="slide"] > * {
    opacity: var(--ui-presence-t);
    transform: translateX(calc((1 - var(--ui-presence-t)) * 16px));
    transition: opacity var(--ui-motion-normal), transform var(--ui-motion-normal);
}

.ui-presence[data-presence-cue="scale"] > * {
    opacity: var(--ui-presence-t);
    transform: scale(calc(0.92 + var(--ui-presence-t) * 0.08));
    transition: opacity var(--ui-motion-normal), transform var(--ui-motion-normal);
}

@media (prefers-reduced-motion: reduce) {
    .ui-presence {
        --ui-presence-t: 1 !important;
    }
    .ui-presence > * {
        transform: none !important;
        opacity: 1 !important;
        transition: none !important;
    }
}

.ui-sequence {
    display: block;
}

.ui-shared-layout {
    display: contents;
}

.ui-shared-element {
    display: block;
    will-change: transform, opacity;
}

/* ============================================================
   DropdownMenu — role="menu" action list (overlays/dropdown_menu.rs).
   Mirrors the .ui-select-option idiom (hover/disabled + focus-visible)
   inside the glass .ui-popover panel it renders into.
   ============================================================ */
.ui-dropdown-menu {
    margin: 0;
    padding: var(--ui-space-1);
    list-style: none;
    display: grid;
    gap: 1px;
    min-width: 200px;
}

.ui-dropdown-menu-item {
    display: block;
}

.ui-dropdown-menu-button {
    width: 100%;
    display: flex;
    align-items: center;
    gap: var(--ui-space-2);
    padding: 6px 10px;
    border: 0;
    border-radius: calc(var(--ui-radius-md) - 2px);
    background: transparent;
    color: var(--ui-fg);
    font: inherit;
    text-align: left;
    cursor: pointer;
    transition: background var(--ui-motion-fast), color var(--ui-motion-fast);
}

.ui-dropdown-menu-button:hover {
    background: var(--ui-surface-muted);
}

.ui-dropdown-menu-button:focus-visible {
    outline: 2px solid var(--ui-focus);
    outline-offset: 0;
}

.ui-dropdown-menu-button[data-active="true"] {
    box-shadow: inset 2px 0 0 var(--ui-focus);
    background: color-mix(in srgb, var(--ui-focus), transparent 90%);
}

.ui-dropdown-menu-item--disabled .ui-dropdown-menu-button,
.ui-dropdown-menu-button:disabled {
    color: var(--ui-muted-fg);
    cursor: not-allowed;
}

.ui-dropdown-menu-item--disabled .ui-dropdown-menu-button:hover,
.ui-dropdown-menu-button:disabled:hover {
    background: transparent;
}

.ui-dropdown-menu-separator {
    height: 1px;
    margin: var(--ui-space-1) calc(-1 * var(--ui-space-1));
    background: var(--ui-border);
    list-style: none;
}

/* ============================================================
   Combobox — typeahead single-select (combobox.rs).
   ============================================================ */
.ui-combobox {
    display: grid;
    gap: var(--ui-space-1);
}

.ui-combobox-label {
    color: var(--ui-fg);
    font-weight: var(--ui-weight-semibold);
    font-size: var(--ui-text-footnote);
    line-height: var(--ui-leading-footnote);
}

.ui-combobox-input {
    width: 100%;
    min-height: var(--ui-control-height);
    padding: 0 12px;
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    color: var(--ui-fg);
    font: inherit;
    transition: border-color var(--ui-motion-fast), box-shadow var(--ui-motion-fast);
}

.ui-combobox-input::placeholder {
    color: var(--ui-muted-fg);
}

.ui-combobox-input:focus-visible {
    outline: 2px solid var(--ui-focus);
    outline-offset: 2px;
}

.ui-combobox-input:disabled {
    opacity: 0.52;
    cursor: not-allowed;
}

.ui-combobox-listbox {
    margin: 0;
    padding: var(--ui-space-1);
    list-style: none;
    min-width: 220px;
    max-height: 280px;
    overflow-y: auto;
    display: grid;
    gap: 1px;
}

.ui-combobox-option {
    padding: 6px 10px;
    border-radius: calc(var(--ui-radius-md) - 2px);
    cursor: pointer;
    color: var(--ui-fg);
    transition: background var(--ui-motion-fast);
}

.ui-combobox-option:hover {
    background: var(--ui-surface-muted);
}

.ui-combobox-option--selected {
    background: var(--ui-surface-muted);
    font-weight: var(--ui-weight-semibold);
}

.ui-combobox-option--disabled {
    color: var(--ui-muted-fg);
    cursor: not-allowed;
}

.ui-combobox-option[data-active="true"] {
    box-shadow: inset 2px 0 0 var(--ui-focus);
    background: color-mix(in srgb, var(--ui-focus), transparent 90%);
}

.ui-combobox-empty {
    margin: 0;
    padding: 8px 10px;
    color: var(--ui-muted-fg);
    font-size: var(--ui-text-footnote);
    line-height: var(--ui-leading-footnote);
}

/* ============================================================
   RadioGroup (forms.rs).
   ============================================================ */
.ui-radio-group {
    margin: 0;
    padding: 0;
    border: 0;
    display: grid;
    gap: var(--ui-space-2);
    color: var(--ui-fg);
}

.ui-radio-group[disabled] {
    opacity: 0.52;
}

.ui-radio-group-legend {
    padding: 0;
    font-weight: var(--ui-weight-semibold);
    font-size: var(--ui-text-subhead);
    line-height: var(--ui-leading-subhead);
}

.ui-radio-group-description {
    margin: 0;
    color: var(--ui-muted-fg);
    font-size: var(--ui-text-footnote);
    line-height: var(--ui-leading-footnote);
}

.ui-radio-group-list {
    display: grid;
    gap: var(--ui-space-2);
}

.ui-radio {
    display: flex;
    align-items: flex-start;
    gap: var(--ui-space-3);
    padding: var(--ui-space-2) var(--ui-space-3);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    transition: border-color var(--ui-motion-fast), background var(--ui-motion-fast);
}

.ui-radio:hover {
    background: var(--ui-surface-muted);
}

.ui-radio--selected {
    border-color: var(--ui-primary);
    background: color-mix(in srgb, var(--ui-primary), transparent 92%);
}

.ui-radio--disabled {
    opacity: 0.52;
    cursor: not-allowed;
}

.ui-radio--disabled:hover {
    background: var(--ui-surface);
}

.ui-radio-input {
    margin: 0;
    width: 18px;
    height: 18px;
    flex: none;
    accent-color: var(--ui-primary);
    cursor: pointer;
}

.ui-radio-input:focus-visible {
    outline: 2px solid var(--ui-focus);
    outline-offset: 2px;
}

.ui-radio-copy {
    display: grid;
    gap: var(--ui-space-0);
}

.ui-radio-label {
    font-weight: var(--ui-weight-medium);
    cursor: pointer;
}

.ui-radio-description {
    margin: 0;
    color: var(--ui-muted-fg);
    font-size: var(--ui-text-footnote);
    line-height: var(--ui-leading-footnote);
}

/* ============================================================
   Type-ramp helpers — apply the contract vars by class.
   ============================================================ */
.ui-text--caption2   { font-size: var(--ui-text-caption2);   line-height: var(--ui-leading-caption2);   letter-spacing: var(--ui-tracking-caption2);   font-weight: var(--ui-weight-medium); }
.ui-text--caption    { font-size: var(--ui-text-caption);    line-height: var(--ui-leading-caption);    letter-spacing: var(--ui-tracking-caption);    font-weight: var(--ui-weight-medium); }
.ui-text--footnote   { font-size: var(--ui-text-footnote);   line-height: var(--ui-leading-footnote);   letter-spacing: var(--ui-tracking-footnote);   font-weight: var(--ui-weight-regular); }
.ui-text--subhead    { font-size: var(--ui-text-subhead);    line-height: var(--ui-leading-subhead);    letter-spacing: var(--ui-tracking-subhead);    font-weight: var(--ui-weight-regular); }
.ui-text--callout    { font-size: var(--ui-text-callout);    line-height: var(--ui-leading-callout);    letter-spacing: var(--ui-tracking-callout);    font-weight: var(--ui-weight-regular); }
.ui-text--body       { font-size: var(--ui-text-body);       line-height: var(--ui-leading-body);       letter-spacing: var(--ui-tracking-body);       font-weight: var(--ui-weight-regular); }
.ui-text--headline   { font-size: var(--ui-text-headline);   line-height: var(--ui-leading-headline);   letter-spacing: var(--ui-tracking-headline);   font-weight: var(--ui-weight-semibold); }
.ui-text--title3     { font-size: var(--ui-text-title3);     line-height: var(--ui-leading-title3);     letter-spacing: var(--ui-tracking-title3);     font-weight: var(--ui-weight-semibold); }
.ui-text--title2     { font-size: var(--ui-text-title2);     line-height: var(--ui-leading-title2);     letter-spacing: var(--ui-tracking-title2);     font-weight: var(--ui-weight-semibold); }
.ui-text--title1     { font-size: var(--ui-text-title1);     line-height: var(--ui-leading-title1);     letter-spacing: var(--ui-tracking-title1);     font-weight: var(--ui-weight-bold); }
.ui-text--largetitle { font-size: var(--ui-text-largetitle); line-height: var(--ui-leading-largetitle); letter-spacing: var(--ui-tracking-largetitle); font-weight: var(--ui-weight-bold); }
.ui-text--display    { font-size: var(--ui-text-display);    line-height: var(--ui-leading-display);    letter-spacing: var(--ui-tracking-display);    font-weight: var(--ui-weight-bold); }

.ui-heading {
    margin: 0;
    color: var(--ui-fg);
    font-weight: var(--ui-weight-semibold);
    letter-spacing: var(--ui-tracking-title2);
    line-height: var(--ui-leading-title2);
}

/* ============================================================
   Forward-declared AI / Comet surfaces (consumed by Wave 2).
   Apple/Comet-grade defaults; every animation is gated under
   reduced-motion at the bottom of this block.
   ============================================================ */
.ui-ai-status {
    display: inline-flex;
    align-items: center;
    gap: var(--ui-space-2);
    padding: 4px 10px 4px 8px;
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-full);
    background: var(--ui-glass);
    backdrop-filter: blur(12px) saturate(160%);
    -webkit-backdrop-filter: blur(12px) saturate(160%);
    color: var(--ui-fg);
    font-size: var(--ui-text-footnote);
    line-height: 1;
}

.ui-ai-status-dot {
    width: 8px;
    height: 8px;
    border-radius: var(--ui-radius-full);
    background: var(--ui-success);
    box-shadow: 0 0 0 0 color-mix(in srgb, var(--ui-success), transparent 40%);
    animation: ui-ai-pulse 1800ms var(--ui-ease-standard) infinite;
}

.ui-ai-status-label {
    color: var(--ui-muted-fg);
    font-weight: var(--ui-weight-medium);
}

@keyframes ui-ai-pulse {
    0%   { box-shadow: 0 0 0 0 color-mix(in srgb, var(--ui-success), transparent 40%); }
    70%  { box-shadow: 0 0 0 6px color-mix(in srgb, var(--ui-success), transparent 100%); }
    100% { box-shadow: 0 0 0 0 color-mix(in srgb, var(--ui-success), transparent 100%); }
}

.ui-citation-chip {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 18px;
    height: 18px;
    padding: 0 5px;
    border-radius: var(--ui-radius-full);
    background: color-mix(in srgb, var(--ui-primary), transparent 84%);
    color: var(--ui-primary);
    font-size: var(--ui-text-caption2);
    font-weight: var(--ui-weight-semibold);
    font-variant-numeric: tabular-nums;
    vertical-align: super;
    cursor: pointer;
    transition: background var(--ui-motion-fast);
}

.ui-citation-chip:hover {
    background: color-mix(in srgb, var(--ui-primary), transparent 72%);
}

.ui-source-rail {
    display: flex;
    gap: var(--ui-space-3);
    overflow-x: auto;
    padding-bottom: var(--ui-space-2);
    scroll-snap-type: x mandatory;
    scrollbar-width: thin;
}

.ui-source-card {
    scroll-snap-align: start;
    flex: 0 0 auto;
    width: 240px;
    display: grid;
    gap: var(--ui-space-1);
    padding: var(--ui-space-3);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-surface);
    color: var(--ui-fg);
    box-shadow: var(--ui-elevation-1);
    text-decoration: none;
    transition: transform var(--ui-motion-fast) var(--ui-ease-standard), box-shadow var(--ui-motion-fast);
}

.ui-source-card:hover {
    transform: translateY(-2px);
    box-shadow: var(--ui-elevation-2);
}

.ui-source-favicon {
    width: 16px;
    height: 16px;
    border-radius: var(--ui-radius-sm);
    object-fit: cover;
}

.ui-source-title {
    font-weight: var(--ui-weight-semibold);
    font-size: var(--ui-text-subhead);
    line-height: var(--ui-leading-subhead);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
}

.ui-source-domain {
    color: var(--ui-muted-fg);
    font-size: var(--ui-text-caption);
    line-height: var(--ui-leading-caption);
}

.ui-source-snippet {
    color: var(--ui-muted-fg);
    font-size: var(--ui-text-footnote);
    line-height: var(--ui-leading-footnote);
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
}

.ui-stream-token {
    opacity: 0;
    animation: ui-stream-token-in 220ms var(--ui-ease-decelerate) forwards;
}

@keyframes ui-stream-token-in {
    from { opacity: 0; }
    to   { opacity: 1; }
}

.ui-stream-caret {
    display: inline-block;
    width: 2px;
    height: 1.1em;
    margin-left: 1px;
    vertical-align: text-bottom;
    background: var(--ui-accent);
    animation: ui-stream-blink 1000ms steps(2, jump-none) infinite;
}

@keyframes ui-stream-blink {
    0%, 50%   { opacity: 1; }
    50.01%, 100% { opacity: 0; }
}

.ui-prompt-input {
    display: flex;
    align-items: flex-end;
    gap: var(--ui-space-2);
    padding: var(--ui-space-2);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-floating);
    background: var(--ui-glass);
    backdrop-filter: blur(18px) saturate(160%);
    -webkit-backdrop-filter: blur(18px) saturate(160%);
    box-shadow: var(--ui-elevation-1), inset 0 1px 0.5px var(--ui-glass-highlight);
}

.ui-prompt-input:focus-within {
    border-color: var(--ui-focus);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--ui-focus), transparent 80%), var(--ui-elevation-1);
}

.ui-prompt-input textarea {
    flex: 1;
    min-height: 24px;
    max-height: 200px;
    resize: none;
    border: 0;
    background: transparent;
    color: var(--ui-fg);
    font: inherit;
    line-height: var(--ui-leading-body);
    outline: none;
}

.ui-prompt-input textarea::placeholder {
    color: var(--ui-muted-fg);
}

.ui-prompt-send,
.ui-prompt-stop {
    flex: none;
    display: inline-grid;
    place-items: center;
    width: 32px;
    height: 32px;
    border: 0;
    border-radius: var(--ui-radius-full);
    cursor: pointer;
    transition: transform var(--ui-motion-fast), background var(--ui-motion-fast);
}

.ui-prompt-send {
    background: var(--ui-accent);
    color: var(--ui-on-accent);
}

.ui-prompt-send:active:not(:disabled) {
    transform: scale(var(--ui-press-scale));
}

.ui-prompt-send:disabled {
    opacity: 0.45;
    cursor: not-allowed;
}

.ui-prompt-stop {
    background: var(--ui-surface-muted);
    color: var(--ui-fg);
}

.ui-assistant-panel {
    display: grid;
    grid-template-rows: auto 1fr auto;
    min-height: 0;
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-glass);
    backdrop-filter: blur(22px) saturate(160%);
    -webkit-backdrop-filter: blur(22px) saturate(160%);
    box-shadow: var(--ui-elevation-3), inset 0 1px 0.5px var(--ui-glass-highlight);
    color: var(--ui-fg);
}

.ui-assistant-panel--docked-end {
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    width: min(420px, 100%);
    border-radius: var(--ui-radius-lg) 0 0 var(--ui-radius-lg);
    z-index: 60;
}

.ui-assistant-panel--docked-start {
    position: fixed;
    top: 0;
    left: 0;
    bottom: 0;
    width: min(420px, 100%);
    border-radius: 0 var(--ui-radius-lg) var(--ui-radius-lg) 0;
    z-index: 60;
}

.ui-assistant-panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--ui-space-2);
    padding: var(--ui-space-3) var(--ui-space-4);
    border-bottom: 1px solid var(--ui-border);
    font-weight: var(--ui-weight-semibold);
}

.ui-assistant-panel-body {
    min-height: 0;
    overflow-y: auto;
    padding: var(--ui-space-4);
    display: grid;
    gap: var(--ui-space-3);
    align-content: start;
}

.ui-assistant-panel-footer {
    padding: var(--ui-space-3) var(--ui-space-4);
    border-top: 1px solid var(--ui-border);
}

.ui-agent-timeline {
    display: grid;
    gap: 0;
}

.ui-agent-timeline-step {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: var(--ui-space-3);
    padding-bottom: var(--ui-space-3);
}

.ui-agent-timeline-node {
    display: inline-grid;
    place-items: center;
    width: 22px;
    height: 22px;
    border-radius: var(--ui-radius-full);
    background: var(--ui-surface-muted);
    border: 1px solid var(--ui-border);
    color: var(--ui-fg);
    font-size: var(--ui-text-caption2);
    z-index: 1;
}

.ui-agent-timeline-step[data-state="active"] .ui-agent-timeline-node {
    background: var(--ui-accent);
    color: var(--ui-on-accent);
    border-color: var(--ui-accent);
}

.ui-agent-timeline-step[data-state="done"] .ui-agent-timeline-node {
    background: color-mix(in srgb, var(--ui-success), transparent 78%);
    color: var(--ui-success);
    border-color: color-mix(in srgb, var(--ui-success), transparent 60%);
}

.ui-agent-timeline-connector {
    grid-column: 1;
    justify-self: center;
    width: 2px;
    min-height: var(--ui-space-3);
    background: var(--ui-border);
    margin: 2px 0;
}

.ui-command-menu-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(10, 15, 24, 0.38);
    z-index: 40;
}

.ui-toast-region {
    position: fixed;
    bottom: var(--ui-space-5);
    right: var(--ui-space-5);
    z-index: 80;
    display: grid;
    gap: var(--ui-space-2);
    width: min(380px, calc(100vw - 2 * var(--ui-space-5)));
    pointer-events: none;
}

.ui-toast-region > * {
    pointer-events: auto;
    animation: ui-toast-in 260ms var(--ui-ease-decelerate) both;
}

@keyframes ui-toast-in {
    from { opacity: 0; transform: translateY(12px) scale(0.98); }
    to   { opacity: 1; transform: translateY(0) scale(1); }
}

.ui-badge {
    display: inline-flex;
    align-items: center;
    gap: var(--ui-space-1);
    padding: 2px 8px;
    border-radius: var(--ui-radius-full);
    font-size: var(--ui-text-caption);
    line-height: var(--ui-leading-caption);
    font-weight: var(--ui-weight-semibold);
    background: var(--ui-surface-muted);
    color: var(--ui-muted-fg);
}

.ui-badge--primary { background: color-mix(in srgb, var(--ui-primary), transparent 86%); color: var(--ui-primary); }
.ui-badge--success { background: color-mix(in srgb, var(--ui-success), transparent 86%); color: var(--ui-success); }
.ui-badge--warning { background: color-mix(in srgb, var(--ui-warning), transparent 86%); color: var(--ui-warning); }
.ui-badge--danger  { background: color-mix(in srgb, var(--ui-danger),  transparent 86%); color: var(--ui-danger); }
.ui-badge--info    { background: color-mix(in srgb, var(--ui-info),    transparent 86%); color: var(--ui-info); }

.ui-avatar {
    display: inline-grid;
    place-items: center;
    width: 36px;
    height: 36px;
    border-radius: var(--ui-radius-full);
    background: var(--ui-surface-strong);
    color: var(--ui-fg);
    font-weight: var(--ui-weight-semibold);
    overflow: hidden;
    flex: none;
}

.ui-avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
}

.ui-avatar--sm { width: 24px; height: 24px; font-size: var(--ui-text-caption); }
.ui-avatar--md { width: 36px; height: 36px; font-size: var(--ui-text-subhead); }
.ui-avatar--lg { width: 48px; height: 48px; font-size: var(--ui-text-callout); }

.ui-spinner {
    display: inline-block;
    width: 18px;
    height: 18px;
    border-radius: var(--ui-radius-full);
    border: 2px solid color-mix(in srgb, var(--ui-accent), transparent 75%);
    border-top-color: var(--ui-accent);
    animation: ui-spin 720ms linear infinite;
}

@keyframes ui-spin {
    to { transform: rotate(360deg); }
}

.ui-sheet {
    position: fixed;
    top: 0;
    bottom: 0;
    width: min(440px, 100%);
    background: var(--ui-glass);
    backdrop-filter: blur(22px) saturate(160%);
    -webkit-backdrop-filter: blur(22px) saturate(160%);
    box-shadow: var(--ui-elevation-3), inset 0 1px 0.5px var(--ui-glass-highlight);
    z-index: 70;
    transition: transform var(--ui-motion-normal) var(--ui-ease-emphasized);
}

.ui-sheet--start {
    left: 0;
    border-radius: 0 var(--ui-radius-lg) var(--ui-radius-lg) 0;
    transform: translateX(-100%);
}

.ui-sheet--end {
    right: 0;
    border-radius: var(--ui-radius-lg) 0 0 var(--ui-radius-lg);
    transform: translateX(100%);
}

.ui-sheet[data-state="open"] {
    transform: translateX(0);
}

/* Overlay enter animation for popovers/dialogs/command menus (Wave 2 drives
   data-state). */
@keyframes ui-overlay-in {
    from { opacity: 0; transform: translateY(6px) scale(0.98); }
    to   { opacity: 1; transform: translateY(0) scale(1); }
}

.ui-popover[data-state="open"],
.ui-dialog-panel[data-state="open"],
.ui-command-menu-panel[data-state="open"] {
    animation: ui-overlay-in 200ms var(--ui-ease-decelerate) both;
}

.ui-popover[data-state="closed"],
.ui-dialog-panel[data-state="closed"],
.ui-command-menu-panel[data-state="closed"] {
    opacity: 0;
    transform: translateY(6px) scale(0.98);
    transition: opacity var(--ui-motion-fast) var(--ui-ease-accelerate), transform var(--ui-motion-fast) var(--ui-ease-accelerate);
}

/* Reduced-motion neutralizers for the forward-declared animated surfaces. */
@media (prefers-reduced-motion: reduce) {
    .ui-ai-status-dot,
    .ui-stream-token,
    .ui-stream-caret,
    .ui-spinner,
    .ui-toast-region > *,
    .ui-source-card,
    .ui-sheet,
    .ui-popover[data-state="open"],
    .ui-dialog-panel[data-state="open"],
    .ui-command-menu-panel[data-state="open"] {
        animation: none !important;
        transition: none !important;
    }
    .ui-stream-token { opacity: 1 !important; }
    .ui-stream-caret { opacity: 1 !important; }
}

[data-ui-motion="reduced"] .ui-ai-status-dot,
[data-ui-motion="reduced"] .ui-stream-token,
[data-ui-motion="reduced"] .ui-stream-caret,
[data-ui-motion="reduced"] .ui-spinner,
[data-ui-motion="reduced"] .ui-toast-region > *,
[data-ui-motion="reduced"] .ui-source-card,
[data-ui-motion="reduced"] .ui-sheet,
[data-ui-motion="reduced"] .ui-popover[data-state="open"],
[data-ui-motion="reduced"] .ui-dialog-panel[data-state="open"],
[data-ui-motion="reduced"] .ui-command-menu-panel[data-state="open"] {
    animation: none !important;
    transition: none !important;
}

[data-ui-motion="reduced"] .ui-stream-token { opacity: 1 !important; }
[data-ui-motion="reduced"] .ui-stream-caret { opacity: 1 !important; }

[data-ui-motion="reduced"] .ui-button,
[data-ui-motion="reduced"] .ui-field-control,
[data-ui-motion="reduced"] .ui-command-menu-input,
[data-ui-motion="reduced"] .ui-icon-button,
[data-ui-motion="reduced"] .ui-switch-thumb,
[data-ui-motion="reduced"] .ui-kinetic-box,
[data-ui-motion="reduced"] .ui-kinetic-text,
[data-ui-motion="reduced"] .ui-frame-layer,
[data-ui-motion="reduced"] .ui-shared-element,
[data-ui-motion="reduced"] .ui-split-text__glyph,
[data-ui-motion="reduced"] .ui-split-text__word,
[data-ui-motion="reduced"] .ui-presence {
    transition: none !important;
    animation: none !important;
    transform: none !important;
}

[data-ui-motion="reduced"] .ui-presence {
    --ui-presence-t: 1 !important;
}

[data-ui-motion="reduced"] .ui-presence > * {
    transform: none !important;
    opacity: 1 !important;
}

[data-ui-glass-policy="solid"] .ui-glass-surface,
[data-ui-glass-policy="solid"] .ui-glass-layer,
[data-ui-glass-policy="solid"] .ui-dialog-panel,
[data-ui-glass-policy="solid"] .ui-command-menu-panel,
[data-ui-glass-policy="solid"] .ui-popover,
[data-ui-glass-policy="solid"] .ui-tooltip-content {
    background: var(--ui-glass-solid) !important;
    backdrop-filter: none !important;
    -webkit-backdrop-filter: none !important;
}
"#;

pub const SCENE_PLAYER_CSS: &str = include_str!("scene_player.css");

pub const GSAP_PRIMITIVES_CSS: &str = include_str!("gsap_primitives.css");

pub const KINETIC_CUES_CSS: &str = include_str!("kinetic_cues.css");

pub const CHARTS_CSS: &str = include_str!("charts.css");

pub const SORTABLE_CSS: &str = include_str!("sortable.css");

pub const TOUR_CSS: &str = include_str!("tour.css");

pub const VOICE_CSS: &str = include_str!("voice.css");

pub const LEARN_CSS: &str = include_str!("learn.css");

pub fn library_css() -> String {
    let base = base_css();
    let mut css = String::with_capacity(
        base.len()
            + COMPONENT_CSS.len()
            + SCENE_PLAYER_CSS.len()
            + GSAP_PRIMITIVES_CSS.len()
            + KINETIC_CUES_CSS.len()
            + CHARTS_CSS.len()
            + SORTABLE_CSS.len()
            + TOUR_CSS.len()
            + VOICE_CSS.len()
            + LEARN_CSS.len()
            + 9,
    );
    css.push_str(&base);
    css.push('\n');
    css.push_str(COMPONENT_CSS);
    css.push('\n');
    css.push_str(SCENE_PLAYER_CSS);
    css.push('\n');
    css.push_str(GSAP_PRIMITIVES_CSS);
    css.push('\n');
    css.push_str(KINETIC_CUES_CSS);
    css.push('\n');
    css.push_str(CHARTS_CSS);
    css.push('\n');
    css.push_str(SORTABLE_CSS);
    css.push('\n');
    css.push_str(TOUR_CSS);
    css.push('\n');
    css.push_str(VOICE_CSS);
    css.push('\n');
    css.push_str(LEARN_CSS);
    css
}
