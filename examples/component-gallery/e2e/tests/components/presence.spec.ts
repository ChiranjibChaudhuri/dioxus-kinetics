import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";
import { clickReplay } from "../_lib/scrub.js";
import { readStyles } from "../_lib/styles.js";
import { installClock } from "../_lib/clock.js";

test.describe("Presence", () => {
  test("tween enter tile transitions from t=0 → t=1 under a clock", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "motion" });
    const tile = page
      .locator(".gallery-variant-tile")
      .filter({ hasText: "Tween enter" });

    const clock = await installClock(page);
    await clickReplay(tile.locator(".gallery-demo-frame"));

    await clock.tickMs(0);
    const t0 = await readStyles(tile.locator('[data-presence-state]'), [
      "presenceT",
      "opacity",
    ]);
    await clock.tickMs(120);
    const t120 = await readStyles(tile.locator('[data-presence-state]'), [
      "presenceT",
      "opacity",
    ]);
    await clock.tickMs(400);
    const tEnd = await readStyles(tile.locator('[data-presence-state]'), [
      "presenceT",
      "opacity",
    ]);

    const start = t0.presenceT ?? t0.opacity ?? 0;
    const mid = t120.presenceT ?? t120.opacity ?? 0;
    const end = tEnd.presenceT ?? tEnd.opacity ?? 0;

    expect(start).toBeLessThanOrEqual(mid);
    expect(mid).toBeLessThanOrEqual(end);
    expect(end).toBeGreaterThan(0.95);
  });

  test("reduced-motion variant lands at the settled state immediately", async ({ page }) => {
    await mountGallery(page, { variant: "reduced-motion", hash: "motion" });
    const tile = page
      .locator(".gallery-variant-tile")
      .filter({ hasText: "Tween enter" });
    const node = tile.locator('[data-presence-state]');
    await expect(node).toBeVisible();

    const styles = await readStyles(node, ["presenceT", "opacity"]);
    const settled = styles.presenceT ?? styles.opacity ?? 1;
    expect(settled).toBeGreaterThan(0.95);
  });

  test("exit tile reaches removed state after exit cue settles", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "motion" });
    const tile = page
      .locator(".gallery-variant-tile")
      .filter({ hasText: "Tween exit" });

    const clock = await installClock(page);
    await clickReplay(tile.locator(".gallery-demo-frame"));
    await clock.tickMs(800);

    const node = tile.locator('[data-presence-state]');
    const count = await node.count();
    if (count === 0) {
      expect(count).toBe(0);
    } else {
      const styles = await readStyles(node, ["presenceT", "opacity"]);
      const t = styles.presenceT ?? styles.opacity ?? 0;
      expect(t).toBeLessThanOrEqual(0.1);
    }
  });
});
