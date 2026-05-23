import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("GlassLayer", () => {
  test("renders the 3x3 level × tone matrix", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "foundations" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("GlassLayer")') })
      .locator(".gallery-preview--ready");

    for (const level of ["Subtle", "Floating", "Overlay"]) {
      for (const tone of ["Neutral", "Info", "Warning"]) {
        await expect(preview.getByText(`${level} · ${tone}`)).toBeVisible();
      }
    }
  });
});
