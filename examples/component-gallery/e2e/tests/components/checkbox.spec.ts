import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Checkbox", () => {
  test("toggles its checked state on click", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "inputs" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Checkbox")') })
      .locator(".gallery-preview--ready");
    const checkbox = preview.locator("input#weekly-summary");
    await expect(checkbox).toBeChecked();
    await checkbox.click();
    await expect(checkbox).not.toBeChecked();
    await checkbox.click();
    await expect(checkbox).toBeChecked();
  });
});
