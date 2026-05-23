import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Surface", () => {
  test("renders the pipeline health surface", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "surfaces" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Surface")') })
      .locator(".gallery-preview--ready");
    await expect(preview.getByRole("heading", { name: "Pipeline health" })).toBeVisible();
    await expect(preview.getByText("12 workflows running")).toBeVisible();
  });
});
