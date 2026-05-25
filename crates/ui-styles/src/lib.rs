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
    --ui-fg: #111827;
    --ui-muted-fg: #5c6778;
    --ui-border: rgba(118, 132, 150, 0.26);
    --ui-focus: #007aff;
    --ui-primary: #0066cc;
    --ui-success: #248a3d;
    --ui-warning: #b66900;
    --ui-danger: #c42b2b;
    --ui-info: #1476bf;
    --ui-shadow-soft: 0 18px 46px rgba(27, 39, 61, 0.10);
    --ui-shadow-lifted: 0 24px 80px rgba(13, 20, 32, 0.24);
    --ui-elevation-0: {l0};
    --ui-elevation-1: {l1};
    --ui-elevation-2: {l2};
    --ui-elevation-3: {l3};
    --ui-radius-sm: 6px;
    --ui-radius-md: 8px;
    --ui-radius-lg: 12px;
    --ui-space-1: 4px;
    --ui-space-2: 8px;
    --ui-space-3: 12px;
    --ui-space-4: 16px;
    --ui-space-5: 24px;
    --ui-control-height: 36px;
    --ui-motion-fast: 120ms;
    --ui-motion-normal: 180ms;
}}

[data-ui-theme="dark"] {{
    color-scheme: dark;
    --ui-bg: #0d1117;
    --ui-surface: #151b23;
    --ui-surface-muted: #1c2430;
    --ui-surface-strong: #263142;
    --ui-glass: rgba(25, 32, 43, 0.72);
    --ui-glass-solid: #151b23;
    --ui-fg: #eef3f8;
    --ui-muted-fg: #aab4c2;
    --ui-border: rgba(205, 215, 228, 0.18);
    --ui-focus: #64b5ff;
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
    border: 1px solid transparent;
    padding: 0 14px;
    font-weight: 700;
    cursor: pointer;
}

.ui-button:hover:not(:disabled) {
    transform: translateY(-1px);
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
    color: #ffffff;
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
    /* Tier 5 (Solid) fallback base. The engine renders the actual glass
       effect when the liquid-glass feature is on; this rule only takes
       effect when neither WebGPU nor WebGL2 is available AND no SVG
       backdrop filter is reachable. */
    background: var(--ui-glass-solid);
    box-shadow: var(--ui-elevation-2);
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

/* Visible tone wash for the CSS-only render path. `color-mix` blends ~22%
   of the tone tint into the solid base so each tone reads distinctly even
   without the wgpu engine. The `[data-ui-glass-policy="solid"]` override
   below still wins (its `background: var(--ui-glass-solid) !important`
   forces uniform solid for the "solid" preference), so this only kicks in
   on the default + dark + reduced-motion policies. */
.ui-glass-surface[data-glass-tone="primary"] {
    background: linear-gradient(180deg,
        color-mix(in srgb, var(--ui-glass-tint) 22%, var(--ui-glass-solid)),
        var(--ui-glass-solid));
}
.ui-glass-surface[data-glass-tone="info"] {
    background: linear-gradient(180deg,
        color-mix(in srgb, var(--ui-glass-tint) 22%, var(--ui-glass-solid)),
        var(--ui-glass-solid));
}
.ui-glass-surface[data-glass-tone="success"] {
    background: linear-gradient(180deg,
        color-mix(in srgb, var(--ui-glass-tint) 22%, var(--ui-glass-solid)),
        var(--ui-glass-solid));
}
.ui-glass-surface[data-glass-tone="warning"] {
    background: linear-gradient(180deg,
        color-mix(in srgb, var(--ui-glass-tint) 22%, var(--ui-glass-solid)),
        var(--ui-glass-solid));
}
.ui-glass-surface[data-glass-tone="danger"] {
    background: linear-gradient(180deg,
        color-mix(in srgb, var(--ui-glass-tint) 22%, var(--ui-glass-solid)),
        var(--ui-glass-solid));
}

/* Level: maps to elevation depth. Subtle sits close to the page;
   Floating + Overlay + Chrome progressively lift off it. The wgpu engine
   computes its own shadow; these rules give the CSS render path a parallel
   visual contract so the 3-level showcase row is distinguishable. */
.ui-glass-surface[data-glass-level="subtle"]   { box-shadow: var(--ui-elevation-1); }
.ui-glass-surface[data-glass-level="floating"] { box-shadow: var(--ui-elevation-2); }
.ui-glass-surface[data-glass-level="overlay"]  { box-shadow: var(--ui-elevation-3); }
.ui-glass-surface[data-glass-level="chrome"]   { box-shadow: var(--ui-elevation-3); }

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
    box-shadow: var(--ui-elevation-3);
}

.ui-command-menu-panel {
    box-shadow: var(--ui-elevation-2);
}

.ui-stack {
    display: flex;
    flex-direction: column;
}

.ui-stack--gap-sm { gap: var(--ui-space-2); }
.ui-stack--gap-md { gap: var(--ui-space-3); }

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
    border-radius: 999px;
    background: var(--ui-surface-muted);
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
    border-radius: 999px;
    background: #ffffff;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.20);
    transition: transform var(--ui-motion-normal);
}

.ui-switch-control[aria-checked="true"] .ui-switch-thumb {
    transform: translateX(18px);
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
    font-weight: 600;
    font-size: 13px;
}

.ui-progress-track {
    position: relative;
    height: 8px;
    border-radius: 999px;
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
    font-size: 12px;
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

.ui-stepper-body { display: grid; gap: 2px; }
.ui-stepper-label { font-weight: 600; }
.ui-stepper-description { color: var(--ui-muted-fg); font-size: 12px; }

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
}

.ui-segmented-option--selected {
    background: var(--ui-surface);
    color: var(--ui-fg);
    box-shadow: var(--ui-elevation-1);
    font-weight: 600;
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
    font-weight: 600;
    font-size: 13px;
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
    font-size: 12px;
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
    background: var(--ui-surface);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    box-shadow: var(--ui-elevation-3);
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
    font-weight: 600;
    font-size: 13px;
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
.ui-select-option--selected { background: var(--ui-surface-muted); font-weight: 600; }
.ui-select-option--disabled { color: var(--ui-muted-fg); cursor: not-allowed; }

.ui-datepicker {
    display: grid;
    gap: var(--ui-space-1);
}

.ui-datepicker-label {
    color: var(--ui-fg);
    font-weight: 600;
    font-size: 13px;
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
    font-size: 14px;
    color: var(--ui-fg);
}

.ui-datepicker-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 2px;
}

.ui-datepicker-weekday {
    font-size: 11px;
    text-align: center;
    color: var(--ui-muted-fg);
    padding: 4px 0;
    font-weight: 700;
    text-transform: uppercase;
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
}

.ui-datepicker-cell:hover { background: var(--ui-surface-muted); }
.ui-datepicker-cell--selected {
    background: var(--ui-fg);
    color: var(--ui-bg);
    font-weight: 700;
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
    font-size: 12px;
    padding-bottom: var(--ui-space-2);
}

.ui-data-table-th {
    text-align: left;
    padding: 8px 10px;
    border-bottom: 1px solid var(--ui-border);
    font-weight: 700;
    font-size: 13px;
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
.ui-data-table-sort-indicator { color: var(--ui-muted-fg); font-size: 12px; }

.ui-data-table-cell {
    padding: 8px 10px;
    border-bottom: 1px solid var(--ui-border);
    font-size: 14px;
    vertical-align: top;
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
    font-size: 12px;
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

.ui-tooltip-content {
    border-radius: var(--ui-radius-md);
    background: var(--ui-fg);
    color: var(--ui-bg);
    padding: 6px 8px;
    box-shadow: var(--ui-elevation-1);
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
    font-size: 12px;
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
    font-size: 28px;
    font-weight: 800;
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
    box-shadow: var(--ui-material-shadow, var(--ui-elevation-2));
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
    display: inline-grid;
    place-items: center;
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    color: var(--ui-fg);
    cursor: pointer;
    transition: background var(--ui-motion-fast), border-color var(--ui-motion-fast), transform var(--ui-motion-fast);
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

[data-ui-motion="reduced"] .ui-button,
[data-ui-motion="reduced"] .ui-field-control,
[data-ui-motion="reduced"] .ui-command-menu-input,
[data-ui-motion="reduced"] .ui-icon-button,
[data-ui-motion="reduced"] .ui-switch-thumb,
[data-ui-motion="reduced"] .ui-kinetic-box,
[data-ui-motion="reduced"] .ui-kinetic-text,
[data-ui-motion="reduced"] .ui-frame-layer,
[data-ui-motion="reduced"] .ui-shared-element,
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
[data-ui-glass-policy="solid"] .ui-command-menu-panel {
    background: var(--ui-glass-solid) !important;
    backdrop-filter: none !important;
    -webkit-backdrop-filter: none !important;
}
"#;

pub const SCENE_PLAYER_CSS: &str = include_str!("scene_player.css");

pub const GSAP_PRIMITIVES_CSS: &str = include_str!("gsap_primitives.css");

pub fn library_css() -> String {
    let base = base_css();
    let mut css = String::with_capacity(
        base.len() + COMPONENT_CSS.len() + SCENE_PLAYER_CSS.len() + GSAP_PRIMITIVES_CSS.len() + 3,
    );
    css.push_str(&base);
    css.push('\n');
    css.push_str(COMPONENT_CSS);
    css.push('\n');
    css.push_str(SCENE_PLAYER_CSS);
    css.push('\n');
    css.push_str(GSAP_PRIMITIVES_CSS);
    css
}
