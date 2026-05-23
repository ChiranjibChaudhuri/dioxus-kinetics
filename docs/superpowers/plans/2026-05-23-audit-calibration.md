# Audit Calibration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move the chromium-static audit from 266/268 to 268/268 by reading the correct observable in two motion specs (the WAAPI compositor doesn't update inline `style` mid-flight; values must come from `getComputedStyle` or `Animation` introspection).

**Architecture:** Three small TypeScript-only edits in `examples/component-gallery/e2e/tests/components/`. No production code touched.

**Tech Stack:** Playwright Test (TS).

---

## Task 1: KineticBox motion@default — sample at t=110ms + assert finished state

**Files:**
- Modify: `examples/component-gallery/e2e/tests/components/kinetic-box.spec.ts`

- [ ] **Step 1: Replace the first test body**

Find the existing `test("each of the three cues drives an inline style under a clock", ...)` block. Replace its body so the loop samples at t=110ms (mid-animation) AND asserts the WAAPI animation finishes by t=400ms via `getAnimations()`. The complete replacement test:

```ts
test("each of the three cues drives an inline style under a clock", async ({ page }) => {
  await mountGallery(page, { variant: "default", hash: "motion" });
  const entry = page
    .locator("article.gallery-entry")
    .filter({ has: page.locator('h4:text-is("KineticBox")') });

  for (const cue of ["rise-in", "fade-in", "slide-up"]) {
    const tile = entry
      .locator(".gallery-variant-tile")
      .filter({ has: page.locator(`text="${cue}"`) });

    const box = tile.locator(`[data-motion-cue="${cue}"]`);
    await expect(box).toBeVisible();

    const clock = await installClock(page);
    await clickReplay(tile.locator(".gallery-demo-frame"));

    // Sample mid-animation: cue durations are ~220ms tweens. At t=110ms
    // the compositor is roughly halfway between `from` and `to`.
    await clock.tickMs(110);
    const midComputed = await box.evaluate((el) => {
      const cs = getComputedStyle(el as HTMLElement);
      return { opacity: cs.opacity, transform: cs.transform };
    });
    const midOpacity = Number.parseFloat(midComputed.opacity);
    const midTransform = midComputed.transform;
    expect(
      (midOpacity > 0 && midOpacity < 1) ||
        (midTransform !== "none" && midTransform !== ""),
      `expected in-flight value at t=110ms for cue ${cue}; got opacity=${midComputed.opacity} transform=${midComputed.transform}`,
    ).toBeTruthy();

    // Advance to t=400ms (past the cue's settling time) and assert at
    // least one WAAPI animation on the element has reached the
    // `finished` playState — proves the compositor actually played a
    // keyframe set, not just statically rendered the `to` value.
    await clock.tickMs(290);
    const playStates = await box.evaluate((el) =>
      (el as HTMLElement).getAnimations().map((a) => a.playState),
    );
    expect(
      playStates.length > 0 && playStates.every((s) => s === "finished"),
      `expected at least one finished WAAPI animation on ${cue} at t=400ms; got ${JSON.stringify(playStates)}`,
    ).toBeTruthy();
  }
});
```

- [ ] **Step 2: tsc check**

```bash
cd "/c/Users/Chiranjib Chaudhuri/Documents/Chiranjib/Dioxus_Kinetics/examples/component-gallery/e2e"
npx tsc --noEmit
```

Expected: exit 0.

- [ ] **Step 3: Run the spec**

```bash
cd "/c/Users/Chiranjib Chaudhuri/Documents/Chiranjib/Dioxus_Kinetics/examples/component-gallery/e2e"
npx playwright test tests/components/kinetic-box.spec.ts --project=static --reporter=list 2>&1 | tail -10
```

Expected: 2/2 pass.

- [ ] **Step 4: Commit**

```bash
cd "/c/Users/Chiranjib Chaudhuri/Documents/Chiranjib/Dioxus_Kinetics"
git add examples/component-gallery/e2e/tests/components/kinetic-box.spec.ts
git commit -m "test(e2e): KineticBox samples at t=110ms + asserts finished playState"
```

---

## Task 2: TimelineScope motion@default — waitForFunction on computed opacity

**Files:**
- Modify: `examples/component-gallery/e2e/tests/components/timeline-scope.spec.ts`

- [ ] **Step 1: Replace the first test body**

Find the existing `test("stagger variant tiles animate in across the scrub window", ...)`. Replace its body. The autoplay path no longer responds to scrub events at the runtime level (the WAAPI animation runs on `document.timeline`, not on the ScrubFrame's elapsed-ms signal). Drop the scrub assertions; replace with a poll on rendered opacity.

```ts
test("stagger variant tiles animate in across the scrub window", async ({ page }) => {
  await mountGallery(page, { variant: "default", hash: "motion" });

  const frame = page
    .locator(".gallery-demo-frame--scrub")
    .filter({ hasText: "Stagger" });

  const tile0 = frame.locator('[data-stagger-index="0"] [data-kinetic-id="stagger-0"]');
  const tile3 = frame.locator('[data-stagger-index="3"] [data-kinetic-id="stagger-3"]');

  await expect(tile0).toBeVisible();
  await expect(tile3).toBeVisible();

  // Stagger tiles autoplay via WAAPI on the compositor. Poll the
  // rendered opacity until both first and last tiles settle. Generous
  // 3s budget for the longest stagger delay + cue duration.
  await page.waitForFunction(
    () => {
      const t0 = document.querySelector(
        '[data-stagger-index="0"] [data-kinetic-id="stagger-0"]',
      );
      const t3 = document.querySelector(
        '[data-stagger-index="3"] [data-kinetic-id="stagger-3"]',
      );
      if (!t0 || !t3) return false;
      const o0 = Number.parseFloat(getComputedStyle(t0).opacity);
      const o3 = Number.parseFloat(getComputedStyle(t3).opacity);
      return o0 > 0.95 && o3 > 0.95;
    },
    null,
    { timeout: 3000 },
  );
});
```

The reduced-motion variant test below this one is unchanged.

- [ ] **Step 2: tsc + run**

```bash
cd "/c/Users/Chiranjib Chaudhuri/Documents/Chiranjib/Dioxus_Kinetics/examples/component-gallery/e2e"
npx tsc --noEmit
npx playwright test tests/components/timeline-scope.spec.ts --project=static --reporter=list 2>&1 | tail -10
```

Expected: tsc exit 0; 2/2 pass.

- [ ] **Step 3: Commit**

```bash
cd "/c/Users/Chiranjib Chaudhuri/Documents/Chiranjib/Dioxus_Kinetics"
git add examples/component-gallery/e2e/tests/components/timeline-scope.spec.ts
git commit -m "test(e2e): TimelineScope polls computed opacity instead of scrubbing"
```

---

## Task 3: Re-run audit + regenerate `audit-report.md`

**Files:**
- Modify: `examples/component-gallery/e2e/audit-report.md` (regenerated)

- [ ] **Step 1: Full Chromium-static audit**

```bash
cd "/c/Users/Chiranjib Chaudhuri/Documents/Chiranjib/Dioxus_Kinetics/examples/component-gallery/e2e"
npx playwright test --project=static --reporter=list,./reporters/audit-report.ts 2>&1 | tail -10
```

Use timeout 900000 (15 min).

- [ ] **Step 2: Inspect the report**

```bash
cat examples/component-gallery/e2e/audit-report.md
```

Expected: 268/268 pass. Every component shows `ready` across all four variants.

- [ ] **Step 3: Commit**

```bash
cd "/c/Users/Chiranjib Chaudhuri/Documents/Chiranjib/Dioxus_Kinetics"
git add examples/component-gallery/e2e/audit-report.md
git commit -m "chore(e2e): regenerate audit-report.md — 268/268 after Spec 4 calibration"
```

---

## Execution Handoff

Plan saved. Two execution options:

1. **Subagent-Driven** — fresh subagent per task.
2. **Inline Execution** — execute in this session.

Given the spec is 3 trivial TS edits with no Rust changes and no design judgment, inline execution is recommended.
