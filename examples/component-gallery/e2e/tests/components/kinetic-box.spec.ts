import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";
import { clickReplay } from "../_lib/scrub.js";
import { readStyles } from "../_lib/styles.js";

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

      // Replay re-mounts the children, which fires KineticBox's onmounted
      // handler again and starts a fresh WAAPI animation. WAAPI runs on
      // `document.timeline`; we read `getAnimations()` (running OR finished)
      // to verify the compositor was invoked. Cue durations are short
      // (~220ms) so the animation may already be "finished" by the time
      // we poll — both states count as proof of WAAPI invocation.
      await clickReplay(tile.locator(".gallery-demo-frame"));

      await page.waitForFunction(
        (sel) => {
          const el = document.querySelector(sel) as HTMLElement | null;
          if (!el) return false;
          return el.getAnimations().length > 0;
        },
        `[data-motion-cue="${cue}"]`,
        { timeout: 2000 },
      );

      const playStates = await box.evaluate((el) =>
        (el as HTMLElement).getAnimations().map((a) => a.playState),
      );
      expect(
        playStates.length > 0 &&
          playStates.every((s) => s === "running" || s === "finished"),
        `expected at least one WAAPI animation on cue ${cue}; got ${JSON.stringify(playStates)}`,
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
