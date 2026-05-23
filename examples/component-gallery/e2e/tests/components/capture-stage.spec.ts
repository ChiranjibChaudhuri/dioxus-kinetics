import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("CaptureStage", () => {
  test("renders three viewport profiles", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "capture" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("CaptureStage")') })
      .locator(".gallery-preview--ready");
    for (const caption of [
      "Mobile · 360 × 640",
      "Tablet · 768 × 1024",
      "Desktop · 1440 × 900",
    ]) {
      await expect(preview.getByText(caption)).toBeVisible();
    }
  });
});
