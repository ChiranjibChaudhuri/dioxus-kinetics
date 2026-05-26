# Flagship

Single-page, self-referential marketing site for `dioxus-kinetics`.
Composed entirely from existing scenes and components — no new
primitives. The page exists to make the library look like it
deserves the showcase, and to expose any primitive-level gaps the
gallery hides behind 148 px preview tiles.

## Run

```powershell
dx serve --package flagship --port 9174
```

## Sections (top to bottom)

1. **Hero** — `ProductIntroScene` at full viewport, autoplay-once.
2. **Story** — `ScrollPinnedStoryScene` pinned full-bleed, scroll-driven.
3. **Features** — three `GlassSurface` cards (Info / Primary / Success).
4. **Metrics** — four `MetricCounter`s in a row.
5. **CTA + footer** — two `Button`s and a minimal one-row footer.

## E2E

Playwright spec at `e2e/tests/flagship.spec.ts` asserts zero
gallery-shell markers, hero scene mount, and five-section presence.

```powershell
cd e2e
npm install
npx playwright install chromium
npm test
```

## Binding visual check

See `docs/superpowers/specs/2026-05-25-flagship-marketing-page-design.md`
section "Pass / fail check". The reference screenshot lives at
`docs/hero-screenshot.png`.
