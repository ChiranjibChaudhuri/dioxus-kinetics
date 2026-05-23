import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";
import { readStyles } from "../_lib/styles.js";

test.describe("TimelineScope", () => {
  test("stagger variant tiles animate in across the scrub window", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "motion" });

    const frame = page
      .locator(".gallery-demo-frame--scrub")
      .filter({ hasText: "Stagger" });

    const tile0 = frame.locator('[data-stagger-index="0"] [data-kinetic-id="stagger-0"]');
    const tile3 = frame.locator('[data-stagger-index="3"] [data-kinetic-id="stagger-3"]');

    await expect(tile0).toBeVisible();
    await expect(tile3).toBeVisible();

    // Stagger tiles autoplay via WAAPI on the compositor (`document.timeline`).
    // Playwright's installed clock doesn't advance the compositor clock, so we
    // poll the rendered opacity until both first and last tiles settle.
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

  test("reduced-motion variant renders the autoplay tiles in their settled state", async ({ page }) => {
    await mountGallery(page, { variant: "reduced-motion", hash: "motion" });
    const reduced = page.locator('[data-ui-transparency="reduced"]');
    await expect(reduced).toBeVisible();
    const tile = reduced.locator('[data-kinetic-id="reduced-0"]');
    const opacity = (await readStyles(tile, ["opacity"])).opacity ?? 1;
    expect(opacity).toBeGreaterThan(0.95);
  });
});
