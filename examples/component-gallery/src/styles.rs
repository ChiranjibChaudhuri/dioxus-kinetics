pub const GALLERY_CSS: &str = r#"
body {
    min-width: 320px;
}

.gallery-shell {
    display: grid;
    grid-template-columns: 280px minmax(0, 1fr);
    min-height: 100vh;
}

.gallery-rail {
    position: sticky;
    top: 0;
    align-self: start;
    height: 100vh;
    padding: 24px 18px;
    border-right: 1px solid var(--ui-border);
    background: var(--ui-glass);
    backdrop-filter: blur(22px) saturate(160%);
}

.gallery-brand {
    display: flex;
    gap: var(--ui-space-3);
    align-items: center;
    padding-bottom: var(--ui-space-5);
}

.gallery-mark {
    display: grid;
    width: 42px;
    height: 42px;
    place-items: center;
    border-radius: var(--ui-radius-lg);
    background: var(--ui-fg);
    color: var(--ui-bg);
    font-weight: 800;
}

.gallery-brand h1,
.gallery-brand p,
.gallery-header h2,
.gallery-header p,
.gallery-section-heading h3,
.gallery-section-heading p,
.gallery-entry h4,
.gallery-entry p {
    margin: 0;
}

.gallery-nav,
.gallery-mobile-tabs,
.gallery-controls,
.gallery-control-group {
    display: flex;
    gap: var(--ui-space-2);
}

.gallery-nav {
    flex-direction: column;
}

.gallery-nav a,
.gallery-mobile-tabs a {
    color: var(--ui-muted-fg);
    text-decoration: none;
    border-radius: var(--ui-radius-md);
    padding: 9px 10px;
}

.gallery-nav a:hover,
.gallery-mobile-tabs a:hover {
    background: var(--ui-surface-muted);
    color: var(--ui-fg);
}

.gallery-main {
    width: min(1220px, 100%);
    padding: 36px;
}

.gallery-header,
.gallery-section,
.gallery-entry-copy {
    display: grid;
    gap: var(--ui-space-3);
}

.gallery-eyebrow {
    color: var(--ui-primary);
    font-size: 13px;
    font-weight: 800;
    text-transform: uppercase;
}

.gallery-header h2 {
    font-size: 34px;
    line-height: 1.12;
}

.gallery-header p,
.gallery-section-heading p,
.gallery-entry p {
    color: var(--ui-muted-fg);
    line-height: 1.6;
}

.gallery-controls {
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    margin: 20px 0;
    padding: var(--ui-space-4);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-glass);
    backdrop-filter: blur(18px) saturate(160%);
}

.gallery-control-group {
    align-items: center;
    flex-wrap: wrap;
}

.gallery-control-label {
    font-weight: 800;
}

.gallery-mobile-tabs {
    display: none;
    overflow-x: auto;
    padding-bottom: var(--ui-space-4);
}

.gallery-section {
    padding: 24px 0 12px;
}

.gallery-grid {
    display: grid;
    gap: var(--ui-space-4);
}

.gallery-entry {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(300px, 0.9fr);
    gap: var(--ui-space-4);
    padding: var(--ui-space-4);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: color-mix(in srgb, var(--ui-surface), transparent 10%);
    box-shadow: 0 18px 46px rgba(27, 39, 61, 0.08);
}

.gallery-entry-title {
    display: flex;
    gap: var(--ui-space-3);
    align-items: center;
    justify-content: space-between;
}

.gallery-status {
    border-radius: 999px;
    padding: 4px 8px;
    font-size: 12px;
    font-weight: 800;
}

.gallery-status--ready {
    background: color-mix(in srgb, var(--ui-success), transparent 86%);
    color: var(--ui-success);
}

.gallery-status--soon {
    background: var(--ui-surface-muted);
    color: var(--ui-muted-fg);
}

.gallery-preview {
    min-height: 148px;
    display: grid;
    align-content: center;
    gap: var(--ui-space-3);
    padding: var(--ui-space-4);
    border-radius: var(--ui-radius-lg);
    border: 1px solid var(--ui-border);
    background: linear-gradient(135deg, var(--ui-surface), var(--ui-surface-muted));
    overflow: auto;
}

.gallery-preview--soon {
    color: var(--ui-muted-fg);
}

.gallery-preview .ui-dialog {
    position: relative;
    inset: auto;
    min-height: 240px;
    display: grid;
    place-items: center;
    padding: var(--ui-space-4);
}

.gallery-preview .ui-dialog-backdrop {
    position: absolute;
    inset: 0;
    border-radius: var(--ui-radius-lg);
    background: rgba(10, 15, 24, 0.24);
}

.gallery-preview .ui-dialog-panel {
    width: min(420px, 100%);
    padding: var(--ui-space-4);
}

.gallery-inline {
    display: flex;
    flex-wrap: wrap;
    gap: var(--ui-space-2);
}

.gallery-accessibility {
    display: grid;
    gap: var(--ui-space-1);
    padding: var(--ui-space-3);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface-muted);
}

.gallery-code {
    grid-column: 1 / -1;
    overflow-x: auto;
    margin: 0;
    padding: 14px;
    border-radius: var(--ui-radius-md);
    background: #101722;
    color: #e8eef7;
    font-size: 13px;
    line-height: 1.55;
}

.gallery-preview .ui-frame-stage,
.gallery-preview .ui-capture-stage {
    min-height: 180px;
    display: grid;
    place-items: center;
    padding: var(--ui-space-4);
}

.gallery-logo {
    display: block;
    width: 100%;
    max-width: 260px;
    margin-bottom: var(--ui-space-4);
}

.gallery-logo svg {
    width: 100%;
    height: auto;
    border-radius: var(--ui-radius-lg);
    display: block;
}

.visually-hidden {
    position: absolute;
    width: 1px;
    height: 1px;
    margin: -1px;
    padding: 0;
    overflow: hidden;
    clip: rect(0 0 0 0);
    border: 0;
}

.gallery-variant-grid {
    display: grid;
    gap: var(--ui-space-2);
    width: 100%;
}

.gallery-variant-grid--3x3 {
    grid-template-columns: repeat(3, minmax(0, 1fr));
}

.gallery-variant-grid--3col {
    grid-template-columns: repeat(3, minmax(0, 1fr));
}

.gallery-variant-grid--2col {
    grid-template-columns: repeat(2, minmax(0, 1fr));
}

.gallery-variant-grid--stack {
    grid-template-columns: 1fr;
}

.gallery-variant-tile {
    display: grid;
    gap: var(--ui-space-1);
    padding: var(--ui-space-3);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
    min-height: 96px;
}

.gallery-variant-label {
    font-size: 11px;
    color: var(--ui-muted-fg);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
}

@media (max-width: 820px) {
    .gallery-shell {
        display: block;
    }

    .gallery-rail {
        position: static;
        height: auto;
        border-right: 0;
        border-bottom: 1px solid var(--ui-border);
    }

    .gallery-nav {
        display: none;
    }

    .gallery-mobile-tabs {
        display: flex;
    }

    .gallery-main {
        padding: 24px 16px;
    }

    .gallery-entry {
        grid-template-columns: 1fr;
    }

    .gallery-variant-grid--3x3,
    .gallery-variant-grid--3col,
    .gallery-variant-grid--2col {
        grid-template-columns: 1fr;
    }
}

body {
    position: relative;
}

body::before {
    content: "";
    position: fixed;
    inset: -10vmax;
    z-index: -1;
    background:
        radial-gradient(closest-side at 18% 28%, color-mix(in srgb, var(--ui-primary), transparent 64%), transparent 70%),
        radial-gradient(closest-side at 78% 22%, color-mix(in srgb, var(--ui-info), transparent 64%), transparent 70%),
        radial-gradient(closest-side at 50% 82%, color-mix(in srgb, var(--ui-success), transparent 70%), transparent 70%),
        var(--ui-bg);
    filter: saturate(110%);
    animation: gallery-mesh-drift 40s linear infinite;
}

.gallery-ambient-mesh {
    /* Marker class used by tests; the real backdrop is body::before above. */
    display: none;
}

[data-ui-theme="dark"] body::before {
    background:
        radial-gradient(closest-side at 18% 28%, rgba(40, 90, 140, 0.50), transparent 70%),
        radial-gradient(closest-side at 78% 22%, rgba(110, 60, 150, 0.40), transparent 70%),
        radial-gradient(closest-side at 50% 82%, rgba(30, 110, 100, 0.40), transparent 70%),
        var(--ui-bg);
}

@keyframes gallery-mesh-drift {
    0%   { transform: translate3d(0, 0, 0); }
    50%  { transform: translate3d(-4%, -3%, 0); }
    100% { transform: translate3d(0, 0, 0); }
}

[data-ui-motion="reduced"] body::before {
    animation: none !important;
}

@media (prefers-reduced-motion: reduce) {
    body::before { animation: none !important; }
}

.gallery-section--glass-stage {
    position: relative;
    isolation: isolate;
}

.gallery-section--glass-stage::before {
    content: "";
    position: absolute;
    inset: var(--ui-space-4);
    z-index: -1;
    border-radius: var(--ui-radius-lg);
    background:
        radial-gradient(circle at 25% 30%, color-mix(in srgb, var(--ui-primary), transparent 30%), transparent 55%),
        radial-gradient(circle at 80% 28%, color-mix(in srgb, var(--ui-success), transparent 30%), transparent 55%),
        radial-gradient(circle at 50% 80%, color-mix(in srgb, var(--ui-warning), transparent 30%), transparent 60%),
        linear-gradient(135deg, color-mix(in srgb, var(--ui-info), transparent 50%), transparent);
    opacity: 0.6;
}

[data-ui-theme="dark"] .gallery-section--glass-stage::before {
    opacity: 0.42;
}

.gallery-controls {
    position: sticky;
    top: 0;
    z-index: 4;
    flex-wrap: wrap;
    align-items: center;
    justify-content: flex-start;
    gap: var(--ui-space-3);
    margin: 0 0 var(--ui-space-4);
    padding: var(--ui-space-3) var(--ui-space-4);
    border: 1px solid var(--ui-border);
    border-radius: var(--ui-radius-lg);
    background: var(--ui-glass);
    backdrop-filter: blur(22px) saturate(160%);
    box-shadow: var(--ui-elevation-1);
}

.gallery-toggle-group {
    display: inline-flex;
    align-items: center;
    gap: var(--ui-space-2);
    padding: 0 var(--ui-space-2);
}

.gallery-toggle-group .ui-button {
    padding: 4px 10px;
    min-height: 28px;
    font-size: 13px;
}

.gallery-demo-frame {
    display: grid;
    gap: var(--ui-space-2);
    padding: var(--ui-space-3);
    border: 1px dashed var(--ui-border);
    border-radius: var(--ui-radius-md);
    background: var(--ui-surface);
}

.gallery-demo-frame-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--ui-space-2);
}

.gallery-demo-frame-replay {
    min-height: 26px;
    padding: 2px 10px;
    font-size: 12px;
}

[data-ui-motion="reduced"] .gallery-demo-frame-replay {
    display: none;
}
"#;
