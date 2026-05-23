import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("LiquidSurface", () => {
  test("mounts a canvas with the configured dimensions", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "foundations" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("LiquidSurface")') })
      .locator(".gallery-preview--ready");
    const canvas = preview.locator("canvas");
    await expect(canvas).toBeVisible();
    await expect(canvas).toHaveAttribute("width", /^\d+$/);
    await expect(canvas).toHaveAttribute("height", /^\d+$/);
    await expect(preview.getByText("Hover me")).toBeVisible();
  });

  test("solid-glass variant still renders a stable fallback", async ({ page }) => {
    await mountGallery(page, { variant: "solid-glass", hash: "foundations" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("LiquidSurface")') })
      .locator(".gallery-preview--ready");
    await expect(preview.getByText("Hover me")).toBeVisible();
  });
});
