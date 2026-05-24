import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Combobox", () => {
  test("listbox is open by default and shows the seeded query's filtered options", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "inputs" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Combobox")') })
      .locator(".gallery-preview--ready");

    const input = preview.locator("input#ticket-finder");
    await expect(input).toHaveValue("ord-2024");

    const listbox = preview.locator('[role="listbox"]');
    await expect(listbox).toBeVisible();

    // Seeded query "ord-2024" matches all four options (they all start with
    // ORD-2024-…), so the listbox renders the full set.
    const options = listbox.locator('[role="option"]');
    await expect(options).toHaveCount(4);
  });

  test("typing a non-matching query shows the empty state", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "inputs" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Combobox")') })
      .locator(".gallery-preview--ready");
    const input = preview.locator("input#ticket-finder");

    await input.fill("zzzzz");
    const status = preview.locator('[role="status"]');
    await expect(status).toBeVisible();
    await expect(status).toHaveText("No matches");
  });

  test("clicking an option closes the listbox", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "inputs" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Combobox")') })
      .locator(".gallery-preview--ready");

    const options = preview.locator('[role="option"]');
    await expect(options.first()).toBeVisible();
    await options.nth(2).click();
    // Selection collapses the popover panel — the listbox unmounts.
    await expect(preview.locator('[role="listbox"]')).toHaveCount(0);
  });
});
