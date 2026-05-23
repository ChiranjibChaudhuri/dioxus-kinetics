import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("EmptyState", () => {
  test("renders the title, description, and action button", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("EmptyState")') })
      .locator(".gallery-preview--ready");
    await expect(preview.getByRole("heading", { name: "No reports yet" })).toBeVisible();
    await expect(preview.getByText("Create a report to share")).toBeVisible();
    await expect(preview.getByRole("button", { name: "Create report" })).toBeVisible();
  });
});
