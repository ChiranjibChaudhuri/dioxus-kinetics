import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";
import { scrubTo } from "../_lib/scrub.js";

test.describe("FrameStage", () => {
  test("scrubbing advances the frame counter", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "composition" });
    const frame = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("FrameStage")') })
      .locator(".gallery-demo-frame--scrub");

    await scrubTo(page, frame, 0);
    await expect(frame.getByText(/Frame 0 \/ 180/)).toBeVisible();

    await scrubTo(page, frame, 1000);
    await expect(frame.getByText(/Frame 30 \/ 180/)).toBeVisible();

    await scrubTo(page, frame, 6000);
    await expect(frame.getByText(/Frame 180 \/ 180/)).toBeVisible();
  });
});
