import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Stack", () => {
  test("renders the two buttons in a stack container", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "layout" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Stack")') })
      .locator(".gallery-preview--ready");
    await expect(preview.getByRole("button", { name: "Create workspace" })).toBeVisible();
    await expect(preview.getByRole("button", { name: "Import data" })).toBeVisible();
  });
});
