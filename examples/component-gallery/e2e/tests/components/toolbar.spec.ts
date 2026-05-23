import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Toolbar", () => {
  test("renders the primary actions + secondary label", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "actions" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Toolbar")') })
      .locator(".gallery-preview--ready");
    for (const label of ["New", "Filter", "Export"]) {
      await expect(preview.getByRole("button", { name: label })).toBeVisible();
    }
    await expect(preview.getByText("Updated now")).toBeVisible();
  });
});
