import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";
import { scrubTo } from "../_lib/scrub.js";
import { readStyles } from "../_lib/styles.js";

test.describe("Sequence", () => {
  test("scrubbing 0 → 560 ms animates the three children", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "motion" });
    const frame = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Sequence")') })
      .locator(".gallery-demo-frame--scrub");

    const title = frame.locator('[data-kinetic-id="title"]');
    const body = frame.locator('[data-kinetic-id="body"]');
    const cta = frame.locator('[data-kinetic-id="cta"]');

    await scrubTo(page, frame, 0);
    const titleStart = await readStyles(title, ["opacity"]);
    expect(titleStart.opacity ?? 0).toBeLessThanOrEqual(0.1);

    await scrubTo(page, frame, 220);
    const titleMid = await readStyles(title, ["opacity"]);
    expect(titleMid.opacity ?? 0).toBeGreaterThan(0.5);

    await scrubTo(page, frame, 560);
    const titleEnd = await readStyles(title, ["opacity"]);
    expect(titleEnd.opacity ?? 0).toBeGreaterThan(0.95);

    const bodyEnd = await readStyles(body, ["transform"]);
    expect(bodyEnd.transform ?? "").toMatch(
      /translateY\(0(?:\.0+)?px\)|translate\(0(?:\.0+)?px,\s*0(?:\.0+)?px\)|^$|none/
    );

    const ctaEnd = await readStyles(cta, ["transform"]);
    expect(ctaEnd.transform ?? "").toMatch(/scale\(1(?:\.0+)?\)|none|^$/);
  });

  test("reduced motion keeps the sequence at its settled state at t=0", async ({ page }) => {
    await mountGallery(page, { variant: "reduced-motion", hash: "motion" });
    const frame = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Sequence")') })
      .locator(".gallery-demo-frame--scrub");
    const title = frame.locator('[data-kinetic-id="title"]');

    await scrubTo(page, frame, 0);
    const opacity = (await readStyles(title, ["opacity"])).opacity ?? 1;
    expect(opacity).toBeGreaterThan(0.95);
  });
});
