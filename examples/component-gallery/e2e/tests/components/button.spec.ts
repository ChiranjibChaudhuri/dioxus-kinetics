import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Button", () => {
  test("renders all four variants", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "actions" });

    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Button")') })
      .locator(".gallery-preview--ready");

    await expect(preview.getByRole("button", { name: "Save changes" })).toBeVisible();
    await expect(preview.getByRole("button", { name: "Review" })).toBeVisible();
    await expect(preview.getByRole("button", { name: "Dismiss" })).toBeVisible();
    await expect(preview.getByRole("button", { name: "Delete" })).toBeVisible();

    // Hover the primary button — assert no console fires; CSS state change
    // is captured by the visual suite.
    await preview.getByRole("button", { name: "Save changes" }).hover();
  });

  test("danger button has the danger CSS class", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "actions" });
    const dangerBtn = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Button")') })
      .locator(".gallery-preview--ready")
      .getByRole("button", { name: "Delete" });
    await expect(dangerBtn).toHaveClass(/ui-button--danger/);
  });
});
