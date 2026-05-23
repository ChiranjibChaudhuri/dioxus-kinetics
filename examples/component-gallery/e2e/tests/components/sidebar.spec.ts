import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Sidebar", () => {
  test("renders the three navigation items with settings selected", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "layout" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Sidebar")') })
      .locator(".gallery-preview--ready");
    const sidebar = preview.locator(".ui-sidebar");
    await expect(sidebar).toBeVisible();
    for (const label of ["Home", "Settings", "Billing"]) {
      await expect(sidebar.getByRole("link", { name: label })).toBeVisible();
    }
    await expect(sidebar.locator('[aria-current="page"]')).toContainText("Settings");
  });

  test("clicking a different item updates the selection", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "layout" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Sidebar")') });
    await preview.getByRole("link", { name: "Billing" }).click();
    await expect(preview.locator(".ui-sidebar [aria-current=\"page\"]")).toContainText("Billing");
  });
});
