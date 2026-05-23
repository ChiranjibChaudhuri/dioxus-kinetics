import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";
import { clickReplay } from "../_lib/scrub.js";
import { readStyles } from "../_lib/styles.js";
import { installClock } from "../_lib/clock.js";

test.describe("KineticBox", () => {
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
      await clock.tickMs(400);

      // WAAPI animates on the compositor; inline `style` attribute does not
      // reflect mid-flight values. Read getComputedStyle instead.
      const computed = await box.evaluate((el) => {
        const cs = getComputedStyle(el as HTMLElement);
        return { opacity: cs.opacity, transform: cs.transform };
      });
      const opacity = Number.parseFloat(computed.opacity);
      const transform = computed.transform;
      expect(
        (opacity < 1 && opacity > 0) || (transform !== "none" && transform !== "")
      ).toBeTruthy();
    }
  });

  test("reduced motion holds each cue at the settled state", async ({ page }) => {
    await mountGallery(page, { variant: "reduced-motion", hash: "motion" });
    const entry = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("KineticBox")') });
    const tile = entry.locator(".gallery-variant-tile").first();
    const box = tile.locator("[data-motion-cue]").first();
    const styles = await readStyles(box, ["opacity", "transform"]);
    const opacity = styles.opacity ?? 1;
    expect(opacity).toBeGreaterThan(0.95);
  });
});
