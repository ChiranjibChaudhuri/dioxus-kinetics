import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("MetricCard", () => {
  test("renders the net-revenue card", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "surfaces" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("MetricCard")') })
      .locator(".gallery-preview--ready");
    await expect(preview.getByText("Net revenue")).toBeVisible();
    await expect(preview.getByText("$128.4k")).toBeVisible();
    await expect(preview.getByText("+12.5%")).toBeVisible();
  });
});
