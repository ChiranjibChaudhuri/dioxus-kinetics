import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("CommandMenu", () => {
  test("renders the seeded group with two items, second pre-selected", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "actions" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("CommandMenu")') })
      .locator(".gallery-preview--ready");

    const menu = preview.locator(".ui-command-menu");
    await expect(menu).toBeVisible();
    await expect(menu.getByText("Navigation")).toBeVisible();
    await expect(menu.getByText("Open dashboard")).toBeVisible();
    await expect(menu.getByText("Open reports")).toBeVisible();

    const selected = menu.locator('[aria-selected="true"]');
    await expect(selected).toContainText("Open reports");
  });

  test("input is seeded with the active query", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "actions" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("CommandMenu")') });
    const input = preview.locator(".ui-command-menu input");
    await expect(input).toHaveValue("rep");
  });
});
