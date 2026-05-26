pub const FLAGSHIP_CSS: &str = r#"
.flagship-shell {
    --ui-primary: #0a7aff;
    --flagship-display-1: clamp(56px, 8vw, 96px);
    --flagship-display-2: clamp(40px, 5vw, 64px);
    --flagship-eyebrow: 13px;
    --flagship-section-pad-y: clamp(72px, 10vh, 144px);
    --flagship-section-pad-x: clamp(20px, 5vw, 88px);
    --flagship-content-max: 1180px;

    display: block;
    position: relative;
    isolation: isolate;
}

.flagship-shell::before {
    content: "";
    position: fixed;
    inset: -10vmax;
    z-index: -1;
    background:
        radial-gradient(closest-side at 18% 28%, color-mix(in srgb, var(--ui-primary), transparent 60%), transparent 70%),
        radial-gradient(closest-side at 78% 22%, color-mix(in srgb, var(--ui-info), transparent 62%), transparent 70%),
        radial-gradient(closest-side at 50% 82%, color-mix(in srgb, var(--ui-success), transparent 70%), transparent 70%),
        var(--ui-bg);
    filter: saturate(112%);
    animation: flagship-mesh-drift 48s linear infinite;
}

@keyframes flagship-mesh-drift {
    0%   { transform: translate3d(0, 0, 0); }
    50%  { transform: translate3d(-4%, -3%, 0); }
    100% { transform: translate3d(0, 0, 0); }
}

[data-ui-motion="reduced"] .flagship-shell::before,
[data-ui-theme="dark"] .flagship-shell::before {
    /* Dark theme uses its own backdrop drift via the existing GALLERY token
       set; the flagship inherits ui-bg and re-tints. Animation suppression
       under reduced motion mirrors the gallery contract. */
}

@media (prefers-reduced-motion: reduce) {
    .flagship-shell::before { animation: none !important; }
}

[data-ui-motion="reduced"] .flagship-shell::before {
    animation: none !important;
}

.flagship-display-2 {
    margin: 0;
    font-size: var(--flagship-display-2);
    font-weight: 800;
    line-height: 1.08;
    letter-spacing: -0.01em;
}

.flagship-eyebrow {
    margin: 0;
    color: var(--ui-primary);
    font-size: var(--flagship-eyebrow);
    font-weight: 800;
    letter-spacing: 0.08em;
    text-transform: uppercase;
}

/* Hero: full viewport, no chrome, the scene fills it edge-to-edge. */
.flagship-hero {
    position: relative;
    width: 100vw;
    height: 100vh;
    min-height: 640px;
    display: grid;
    place-items: center;
    overflow: hidden;
    padding: 0;
}

.flagship-hero-stage {
    width: min(100vw, 1920px);
    height: min(100vh, 1080px);
    display: grid;
    place-items: center;
    transform: translateZ(0);
}

.flagship-hero-stage > * {
    width: 100%;
    height: 100%;
}

/* Story: the embedded scene already provides a 200vh trigger and a sticky
   inner shell, so we only widen the outer slot. */
.flagship-story {
    width: 100vw;
}

.flagship-story .scene-scroll-trigger {
    width: 100vw;
}

.flagship-story .scene-scroll-sticky > * {
    width: min(100vw, 1280px);
    margin-inline: auto;
}

/* Glass feature triplet. */
.flagship-features {
    width: 100vw;
    padding: var(--flagship-section-pad-y) var(--flagship-section-pad-x);
}

.flagship-features-inner {
    width: min(100%, var(--flagship-content-max));
    margin-inline: auto;
    display: grid;
    gap: var(--ui-space-5);
}

.flagship-features-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: var(--ui-space-4);
}

.flagship-features-grid .ui-glass-surface {
    min-height: 220px;
    display: grid;
    gap: var(--ui-space-2);
    transition: transform var(--ui-motion-fast), box-shadow var(--ui-motion-fast);
}

.flagship-features-grid .ui-glass-surface:hover {
    transform: translateY(-2px);
    box-shadow: var(--ui-elevation-3);
}

.flagship-card-title {
    margin: 0;
    font-size: 22px;
    font-weight: 700;
    line-height: 1.18;
}

.flagship-card-body {
    margin: 0;
    color: var(--ui-muted-fg);
    line-height: 1.5;
}

/* Live metric strip. */
.flagship-metrics {
    width: 100vw;
    padding: var(--flagship-section-pad-y) var(--flagship-section-pad-x);
    background: color-mix(in srgb, var(--ui-surface), transparent 18%);
    backdrop-filter: blur(16px) saturate(150%);
    -webkit-backdrop-filter: blur(16px) saturate(150%);
}

.flagship-metrics-inner {
    width: min(100%, var(--flagship-content-max));
    margin-inline: auto;
    display: grid;
    gap: var(--ui-space-5);
}

.flagship-metrics-grid {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: var(--ui-space-4);
}

.flagship-metrics-grid .ui-block-metric-counter {
    padding: var(--ui-space-4);
    border-radius: var(--ui-radius-lg);
    border: 1px solid var(--ui-border);
    background: var(--ui-surface);
    box-shadow: var(--ui-elevation-1);
}

/* CTA band + footer. */
.flagship-cta {
    width: 100vw;
    padding: var(--flagship-section-pad-y) var(--flagship-section-pad-x) 0;
}

.flagship-cta-inner {
    width: min(100%, var(--flagship-content-max));
    margin-inline: auto;
    text-align: center;
    display: grid;
    justify-items: center;
    gap: var(--ui-space-4);
}

.flagship-cta-inner .flagship-display-2 {
    max-width: 18ch;
}

.flagship-cta-caption {
    margin: 0;
    color: var(--ui-muted-fg);
    font-size: 16px;
    max-width: 56ch;
}

.flagship-cta-actions {
    display: flex;
    gap: var(--ui-space-3);
    flex-wrap: wrap;
    justify-content: center;
    padding-top: var(--ui-space-2);
}

.flagship-footer {
    margin-top: var(--flagship-section-pad-y);
    padding: var(--ui-space-4) var(--flagship-section-pad-x);
    border-top: 1px solid var(--ui-border);
    display: flex;
    flex-wrap: wrap;
    gap: var(--ui-space-3);
    align-items: center;
    justify-content: space-between;
    color: var(--ui-muted-fg);
    font-size: 13px;
}

.flagship-footer p {
    margin: 0;
}

.flagship-footer-brand {
    font-weight: 800;
    color: var(--ui-fg);
    letter-spacing: -0.01em;
}

/* Mobile: collapse multi-column grids. */
@media (max-width: 820px) {
    .flagship-features-grid {
        grid-template-columns: 1fr;
    }
    .flagship-metrics-grid {
        grid-template-columns: repeat(2, minmax(0, 1fr));
    }
}

@media (max-width: 540px) {
    .flagship-metrics-grid {
        grid-template-columns: 1fr;
    }
    .flagship-cta-actions {
        flex-direction: column;
        width: 100%;
        align-items: stretch;
    }
}
"#;
