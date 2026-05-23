import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("IconButton", () => {
  test("renders the 3x3 tone × size matrix", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "actions" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("IconButton")') })
      .locator(".gallery-preview--ready");

    for (const tone of ["Neutral", "Primary", "Danger"]) {
      for (const size of ["Compact", "Default", "Spacious"]) {
        await expect(preview.getByText(`${tone} · ${size}`)).toBeVisible();
      }
    }
    await expect(preview.locator(".gallery-variant-grid--3x3")).toBeVisible();
  });

  test("danger tone applies the danger modifier class", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "actions" });
    const danger = page
      .locator(".gallery-variant-tile")
      .filter({ hasText: "Danger · Default" })
      .locator(".ui-icon-button");
    await expect(danger).toHaveClass(/ui-icon-button--danger/);
  });
});
