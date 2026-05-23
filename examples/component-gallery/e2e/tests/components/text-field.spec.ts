import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("TextField", () => {
  test("renders with the seeded value and accepts typed input", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "inputs" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("TextField")') })
      .locator(".gallery-preview--ready");
    const input = preview.locator("input#workspace-name");
    await expect(input).toHaveValue("Acme Ops");
    await input.fill("Acme Ops Renamed");
    await expect(input).toHaveValue("Acme Ops Renamed");
    await expect(preview.getByText("Visible to teammates")).toBeVisible();
  });
});
