# Gallery Showcase + Playwright Verification — Implementation Plan

> **For agentic workers:** Execute task-by-task. Each task is a checkbox; mark `- [x]` when complete. Stop on first failure; surface the trace + screenshot before continuing.

**Spec:** `docs/superpowers/specs/2026-05-24-gallery-showcase-and-playwright-verification-design.md`

**Goal:** Make every gallery preview visibly demonstrate the capabilities its `docs.rs` summary advertises; land behavioral specs + visual baselines for the three new components (`Combobox`, `RadioGroup`, `DropdownMenu`); leave the workspace green on lint, test, and e2e.

**Architecture:** No new modules. Edits limited to `examples/component-gallery/src/previews/*.rs` (showcase remediations) and `examples/component-gallery/e2e/tests/components/*.spec.ts` (new behavioral specs). Visual snapshot baselines regenerated only for the affected subset. The existing `audit-report.ts` reporter regenerates `audit-report.md`.

**Tech stack:** No changes — Rust 1.94, Dioxus 0.7, Playwright 1.49 (already installed), `dx` CLI for static gallery build.

---

## Phase 0 — Pre-flight (sanity checks before any code change)

- [ ] **0.1** Run `dx --version` from repo root; confirm Dioxus CLI is installed. If missing, stop and report so the user can install it.
- [ ] **0.2** Run `npm --prefix examples/component-gallery/e2e ls --depth=0 @playwright/test` to confirm node_modules exist. If missing, `npm --prefix examples/component-gallery/e2e ci`.
- [ ] **0.3** Run `npx --prefix examples/component-gallery/e2e playwright install --with-deps chromium webkit` to confirm browsers are installed. (Idempotent — skips if already there.)
- [ ] **0.4** Verify clean working tree (`git status --porcelain` empty) so the diff after Phase 1 is purely showcase edits.

---

## Phase 1 — Showcase audit + remediation

Goal: open each preview file, decide showcased vs remediate against the audit-table criteria from the spec, apply remediation if needed, run `cargo build -p component-gallery` after each touched file group to catch typos early. Keep edits surgical.

### 1.A — Component-by-component audit (45 components, grouped by preview file)

For each task: read the preview function, cross-reference `docs.rs` summary + the component source in `crates/ui-dioxus/src/`. Apply remediation pattern from spec or mark showcased.

- [ ] **1.A.1** `previews/foundations.rs` — `GlassLayer` (check tones rendered; remediate to show neutral/primary/etc. variants if not shown).
- [ ] **1.A.2** `previews/surfaces.rs` — `Surface`, `GlassSurface`, `MetricCard`. MetricCard: confirm all `MetricTone` variants visible; remediate to a small grid if not.
- [ ] **1.A.3** `previews/liquid_glass.rs` — `LiquidSurface`. Confirm a textured background sits behind it so the glass effect is visible (not just a solid color).
- [ ] **1.A.4** `previews/actions.rs` — `Button`, `IconButton`, `CommandMenu`, `Toolbar`, `DropdownMenu`. DropdownMenu preview must open the menu by default (`open: true` controlled signal or hardcoded). Remediation lives in this file; touch DropdownMenu only.
- [ ] **1.A.5** `previews/inputs.rs` — `TextField`, `Checkbox`, `Switch`, `Slider`, `SegmentedControl`, `Select`, `DatePicker`, `DataTable`, `Combobox`, `RadioGroup`. Apply "open overlay" pattern to `Select`, `DatePicker`, `Combobox` (each builds on Popover — open the popover so the listbox/calendar/filtered list shows). `TextField`: add a second tile that exhibits the `invalid` + `error_text` state.
- [ ] **1.A.6** `previews/layout.rs` — `Stack`, `Tabs`, `Sidebar`, `Accordion`. Accordion: ensure at least one section is expanded so the disclosure pattern is visible.
- [ ] **1.A.7** `previews/navigation.rs` — `Breadcrumb`, `Pagination`, `Stepper`, `SegmentedControl`. Stepper: confirm a step is in `active` AND a prior step is `complete` so the three states render together.
- [ ] **1.A.8** `previews/feedback.rs` — `Alert`, `Progress`, `Skeleton`, `Dialog`, `Toast`, `Tooltip`, `Popover`, `EmptyState`. Apply "open overlay" pattern to `Dialog`, `Tooltip`, `Popover`; apply "persistent overlay" to `Toast`. `Alert`: render Neutral + Success + Warning + Danger + Info tiles. `Progress`: render determinate + indeterminate.
- [ ] **1.A.9** `previews/motion.rs` — `Presence`, `PresenceGate`, `KineticBox`, `KineticText`, `Sequence`, `TimelineScope`. Apply "pinned motion frame" — each preview must render in the visible state (`present: true`, sequence already mid-progress, etc.) so the static snapshot conveys the effect.
- [ ] **1.A.10** `previews/composition.rs` — `FrameStage` (with `FrameClip`/`FrameLayer`). Confirm multi-layer rendering visible; remediate to show at least 2 layers if only one.
- [ ] **1.A.11** `previews/shared.rs` — `SharedElement`, `SharedLayout`. Confirm registry + element visible.
- [ ] **1.A.12** `previews/capture.rs` — `CaptureStage`. Confirm capture viewport visible with content inside.

### 1.B — Apply remediations & checkpoint

- [ ] **1.B.1** After each preview file changes, run `cargo build -p component-gallery` to catch syntax/type errors before moving on.
- [ ] **1.B.2** Update the audit table in the spec doc with final verdicts (`needs review` → `showcased` or `remediate`) so the spec reflects what actually changed.
- [ ] **1.B.3** Run `cargo test -p component-gallery` to confirm the manifest test still passes (no docs.rs name drift).
- [ ] **1.B.4** Run `cargo fmt --all` + `cargo clippy -p component-gallery --all-targets -- -D warnings`. Both clean.

---

## Phase 2 — Behavioral spec files for the three new components

Goal: behavioral parity with the existing 28 per-component spec files. Pattern-match `select.spec.ts` (for combobox) and `command-menu.spec.ts` (for dropdown-menu).

- [ ] **2.1** Read `e2e/tests/components/select.spec.ts` and `e2e/tests/components/command-menu.spec.ts` as templates.
- [ ] **2.2** Create `e2e/tests/components/combobox.spec.ts`:
  - mount → scope to entry by `h4` text
  - assert listbox visible (preview opens by default after Phase 1)
  - type "ord" → assert ≥1 option containing "ord"
  - clear → assert listbox shows all options
  - type "xyzqwerty" → assert empty state (`role="status"`)
  - click an option → assert input updated, option highlighted
  - `expectNoConsoleErrors` clean
- [ ] **2.3** Create `e2e/tests/components/radio-group.spec.ts`:
  - scope to entry
  - assert legend, three radio inputs, `name` attr matches
  - click second option → assert it is `:checked`, others are not
  - tab to next option, press space → assert selection moves
  - `expectNoConsoleErrors` clean
- [ ] **2.4** Create `e2e/tests/components/dropdown-menu.spec.ts`:
  - scope to entry
  - assert menu visible with `role="menu"` (open by default after Phase 1)
  - assert items have `role="menuitem"`, separator has `role="separator"`
  - click a menuitem → assert menu closes
  - disabled item: assert clicking it does not close menu (or assert it's `aria-disabled="true"` and `disabled`)
  - `expectNoConsoleErrors` clean
- [ ] **2.5** Run `npm --prefix examples/component-gallery/e2e run lint:tsc` to confirm new specs typecheck.

---

## Phase 3 — Build static gallery + Playwright baselines

- [ ] **3.1** From repo root, run `dx build -p component-gallery --release --platform web`. Confirm output exists at `target/dx/component-gallery/release/web/public/`. This is the artifact the Playwright `static` project serves.
- [ ] **3.2** Identify components whose preview changed in Phase 1 by `git diff --name-only examples/component-gallery/src/previews/` → derive the set of slugs (e.g. `dialog|tooltip|popover|datepicker|select|combobox|radio-group|dropdown-menu|…`).
- [ ] **3.3** Run `npm --prefix examples/component-gallery/e2e run e2e -- --update-snapshots --grep "<slug-union>"` to regenerate baselines for the affected set on both `static` and `static-webkit`. Do **not** run plain `--update-snapshots` (would silently re-baseline all 45).
- [ ] **3.4** Inspect the newly-written PNGs by eye (open a few in an image viewer). Confirm each visibly demonstrates the documented capability — the calendar grid, listbox, menu, modal, etc. is visible. If any preview's snapshot doesn't show what it should, return to Phase 1 for that component.

---

## Phase 4 — Final verification

- [ ] **4.1** Run full e2e: `npm --prefix examples/component-gallery/e2e run e2e`. All tests green on both projects.
- [ ] **4.2** Open the regenerated `examples/component-gallery/e2e/audit-report.md`. Confirm 45 component rows, all "ready", and the timestamp is fresh.
- [ ] **4.3** Run workspace verification:
  - `cargo fmt --all -- --check` (exit 0)
  - `cargo clippy --workspace --all-targets --no-deps -- -D warnings` (exit 0)
  - `cargo clippy --workspace --all-targets --no-deps --target wasm32-unknown-unknown -- -D warnings` (exit 0)
  - `cargo test --workspace --tests --exclude ui-glass-engine` (all pass)
- [ ] **4.4** Stage everything: preview edits, new spec files, regenerated baselines, refreshed audit-report.md, updated spec doc with finalized audit table.

---

## Commit policy

Single commit at the end with subject:

> `feat(gallery): showcase audit + Combobox/RadioGroup/DropdownMenu e2e specs`

The commit body lists the components remediated, the spec files added, and the count of regenerated PNGs. Do **not** force-push and do **not** amend if a pre-commit hook fails — fix the issue and create a new commit.

---

## Stop conditions (fail-fast)

Stop the plan and surface the error if:
- `dx build` fails (any rust/wasm error) — fix upstream before continuing.
- `cargo clippy` regresses — fix in the touched preview file before continuing.
- Any Phase 3 baseline shows a closed/empty preview after remediation — return to Phase 1, the remediation was wrong.
- Phase 4 `npm run e2e` reports any "failed" or "missing" component — diagnose, do not bypass with `--update-snapshots`.

---

## Rollback

The only persistent state changes are:
- Files under `examples/component-gallery/src/previews/`
- Files under `examples/component-gallery/e2e/tests/components/`
- PNGs under `examples/component-gallery/e2e/tests/visual.spec.ts-snapshots/`
- `examples/component-gallery/e2e/audit-report.md`
- `docs/superpowers/specs/2026-05-24-gallery-showcase-and-playwright-verification-design.md` (audit table)

To roll back: `git checkout HEAD -- examples/component-gallery docs/superpowers/specs/2026-05-24-*.md` (only if no commit was made yet) or `git revert <commit>` after the fact.
