#![forbid(unsafe_code)]

pub const BASE_CSS: &str = r#"
:root,
[data-ui-theme="light"] {
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
}

[data-ui-theme="dark"] {
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
}

[data-ui-density="compact"] {
    --ui-control-height: 32px;
    --ui-space-3: 10px;
    --ui-space-4: 12px;
}

[data-ui-density="comfortable"] {
    --ui-control-height: 36px;
}

[data-ui-density="spacious"] {
    --ui-control-height: 42px;
    --ui-space-3: 14px;
    --ui-space-4: 20px;
}

[data-ui-transparency="reduced"] {
    --ui-glass: var(--ui-glass-solid);
}

* {
    box-sizing: border-box;
}

body {
    margin: 0;
    font-family: var(--ui-font-sans);
    background: var(--ui-bg);
    color: var(--ui-fg);
}

button,
input,
textarea,
select {
    font: inherit;
}

@media (prefers-reduced-motion: reduce) {
    *,
    *::before,
    *::after {
        transition-duration: 0.01ms !important;
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        scroll-behavior: auto !important;
    }
}
"#;

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
}

.ui-surface,
.ui-glass-surface {
    display: grid;
    gap: var(--ui-space-2);
    padding: var(--ui-space-4);
}

.ui-glass-surface,
.ui-dialog-panel,
.ui-command-menu-panel {
    background: var(--ui-glass);
    backdrop-filter: blur(18px) saturate(160%);
    box-shadow: var(--ui-shadow-lifted);
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
    box-shadow: var(--ui-shadow-soft);
}

.ui-toast--success { border-color: color-mix(in srgb, var(--ui-success), transparent 62%); }
.ui-toast--warning { border-color: color-mix(in srgb, var(--ui-warning), transparent 62%); }
.ui-toast--danger { border-color: color-mix(in srgb, var(--ui-danger), transparent 62%); }
.ui-toast--info { border-color: color-mix(in srgb, var(--ui-info), transparent 62%); }

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
    box-shadow: var(--ui-material-shadow, var(--ui-shadow-soft));
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

.ui-presence[data-presence-cue="fade"] {
    opacity: var(--ui-presence-t);
}

.ui-presence[data-presence-cue="rise"] {
    opacity: var(--ui-presence-t);
    transform: translateY(calc((1 - var(--ui-presence-t)) * 8px));
}

.ui-presence[data-presence-cue="slide"] {
    opacity: var(--ui-presence-t);
    transform: translateX(calc((1 - var(--ui-presence-t)) * 16px));
}

.ui-presence[data-presence-cue="scale"] {
    opacity: var(--ui-presence-t);
    transform: scale(calc(0.92 + var(--ui-presence-t) * 0.08));
}

@media (prefers-reduced-motion: reduce) {
    .ui-presence {
        --ui-presence-t: 1 !important;
        transform: none !important;
        opacity: 1 !important;
    }
}
"#;

pub fn library_css() -> String {
    let mut css = String::with_capacity(BASE_CSS.len() + COMPONENT_CSS.len() + 1);
    css.push_str(BASE_CSS);
    css.push('\n');
    css.push_str(COMPONENT_CSS);
    css
}
