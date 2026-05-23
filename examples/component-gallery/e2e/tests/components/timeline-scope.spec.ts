import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";
import { scrubTo } from "../_lib/scrub.js";
import { readStyles } from "../_lib/styles.js";

test.describe("TimelineScope", () => {
  test("stagger variant tiles animate in across the scrub window", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "motion" });

    const frame = page
      .locator(".gallery-demo-frame--scrub")
      .filter({ hasText: "Stagger" });

    const tile0 = frame.locator('[data-stagger-index="0"] [data-kinetic-id="stagger-0"]');
    const tile3 = frame.locator('[data-stagger-index="3"] [data-kinetic-id="stagger-3"]');

    await scrubTo(page, frame, 1200);
    const end0 = (await readStyles(tile0, ["opacity"])).opacity ?? 0;
    const end3 = (await readStyles(tile3, ["opacity"])).opacity ?? 0;
    expect(end0).toBeGreaterThan(0.95);
    expect(end3).toBeGreaterThan(0.95);
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
