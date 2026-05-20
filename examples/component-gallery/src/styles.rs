pub const GALLERY_CSS: &str = r#"
:root {
    color-scheme: light;
    font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    background: #f5f7fa;
    color: #151922;
}

* {
    box-sizing: border-box;
}

body {
    margin: 0;
    min-width: 320px;
    background:
        linear-gradient(135deg, rgba(205, 231, 255, 0.72), rgba(255, 255, 255, 0.0) 34%),
        linear-gradient(180deg, #f7f9fc 0%, #eef2f7 100%);
}

button,
input,
textarea,
select {
    font: inherit;
}

.gallery-shell {
    display: grid;
    grid-template-columns: 260px minmax(0, 1fr);
    min-height: 100vh;
}

.gallery-rail {
    position: sticky;
    top: 0;
    align-self: start;
    height: 100vh;
    padding: 24px 18px;
    border-right: 1px solid rgba(118, 132, 150, 0.24);
    background: rgba(255, 255, 255, 0.64);
    backdrop-filter: blur(22px) saturate(160%);
}

.gallery-brand {
    display: flex;
    gap: 12px;
    align-items: center;
    padding-bottom: 24px;
}

.gallery-mark {
    display: grid;
    width: 42px;
    height: 42px;
    place-items: center;
    border-radius: 12px;
    background: #111827;
    color: #ffffff;
    font-weight: 700;
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

.gallery-brand h1 {
    font-size: 15px;
}

.gallery-brand p,
.gallery-section-heading p,
.gallery-entry p {
    color: #5d6676;
}

.gallery-nav,
.gallery-mobile-tabs {
    display: flex;
    gap: 8px;
}

.gallery-nav {
    flex-direction: column;
}

.gallery-nav a,
.gallery-mobile-tabs a {
    color: #303846;
    text-decoration: none;
    border-radius: 8px;
    padding: 9px 10px;
    font-size: 14px;
}

.gallery-nav a:hover,
.gallery-mobile-tabs a:hover {
    background: rgba(17, 24, 39, 0.06);
}

.gallery-main {
    width: min(1180px, 100%);
    padding: 36px;
}

.gallery-header {
    display: grid;
    gap: 8px;
    padding-bottom: 28px;
}

.gallery-eyebrow {
    color: #0066cc;
    font-size: 13px;
    font-weight: 700;
    text-transform: uppercase;
}

.gallery-header h2 {
    font-size: 34px;
    line-height: 1.12;
}

.gallery-header p {
    max-width: 760px;
    color: #566174;
    line-height: 1.6;
}

.gallery-mobile-tabs {
    display: none;
    overflow-x: auto;
    padding-bottom: 18px;
}

.gallery-section {
    display: grid;
    gap: 16px;
    padding: 24px 0 12px;
}

.gallery-section-heading {
    display: grid;
    gap: 6px;
}

.gallery-section-heading h3 {
    font-size: 22px;
}

.gallery-grid {
    display: grid;
    gap: 16px;
}

.gallery-entry {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(260px, 0.9fr);
    gap: 16px;
    padding: 16px;
    border: 1px solid rgba(118, 132, 150, 0.20);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.78);
    box-shadow: 0 18px 46px rgba(27, 39, 61, 0.08);
}

.gallery-entry-copy {
    display: grid;
    gap: 10px;
}

.gallery-entry-title {
    display: flex;
    gap: 10px;
    align-items: center;
    justify-content: space-between;
}

.gallery-entry-title h4 {
    font-size: 18px;
}

.gallery-status {
    border-radius: 999px;
    padding: 4px 8px;
    font-size: 12px;
    font-weight: 700;
}

.gallery-status--ready {
    background: rgba(36, 138, 61, 0.12);
    color: #1f7a3a;
}

.gallery-status--soon {
    background: rgba(86, 94, 108, 0.12);
    color: #566174;
}

.gallery-example {
    min-width: 0;
}

.gallery-preview {
    min-height: 132px;
    display: grid;
    align-content: center;
    gap: 10px;
    padding: 16px;
    border-radius: 8px;
    border: 1px solid rgba(118, 132, 150, 0.22);
    background:
        linear-gradient(135deg, rgba(255, 255, 255, 0.86), rgba(242, 247, 255, 0.66)),
        #ffffff;
}

.gallery-preview--soon {
    color: #647084;
    background: repeating-linear-gradient(
        135deg,
        rgba(100, 112, 132, 0.08),
        rgba(100, 112, 132, 0.08) 8px,
        rgba(255, 255, 255, 0.72) 8px,
        rgba(255, 255, 255, 0.72) 16px
    );
}

.gallery-preview--soon span {
    font-weight: 700;
}

.gallery-inline {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
}

.gallery-code {
    grid-column: 1 / -1;
    overflow-x: auto;
    margin: 0;
    padding: 14px;
    border-radius: 8px;
    background: #101722;
    color: #e8eef7;
    font-size: 13px;
    line-height: 1.55;
}

.ui-button {
    min-height: 36px;
    border: 1px solid transparent;
    border-radius: 8px;
    padding: 0 14px;
    font-weight: 700;
    cursor: pointer;
}

.ui-button--primary {
    background: #0066cc;
    color: #ffffff;
    box-shadow: 0 10px 22px rgba(0, 102, 204, 0.20);
}

.ui-button--secondary {
    background: #ffffff;
    color: #182230;
    border-color: rgba(118, 132, 150, 0.28);
}

.ui-button--ghost {
    background: transparent;
    color: #2f3a4b;
}

.ui-button--danger {
    background: #c42b2b;
    color: #ffffff;
}

.ui-surface,
.ui-glass-surface {
    display: grid;
    gap: 6px;
    border-radius: 8px;
    padding: 16px;
}

.ui-surface {
    border: 1px solid rgba(118, 132, 150, 0.22);
    background: #ffffff;
    color: #151922;
}

.ui-glass-surface {
    border: 1px solid rgba(255, 255, 255, 0.58);
    background: rgba(255, 255, 255, 0.66);
    backdrop-filter: blur(18px) saturate(160%);
    box-shadow: 0 18px 42px rgba(27, 39, 61, 0.16);
}

.ui-stack {
    display: flex;
    flex-direction: column;
}

.ui-stack--gap-sm {
    gap: 8px;
}

.ui-stack--gap-md {
    gap: 12px;
}

@media (max-width: 820px) {
    .gallery-shell {
        display: block;
    }

    .gallery-rail {
        position: static;
        height: auto;
        border-right: 0;
        border-bottom: 1px solid rgba(118, 132, 150, 0.24);
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
}
"#;
