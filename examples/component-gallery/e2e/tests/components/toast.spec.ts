import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Toast", () => {
  test("pre-seeded showcase renders one toast per tone", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Toast")') })
      .locator(".gallery-preview--ready");

    const stage = preview.locator(".gallery-toast-stage");
    await expect(stage.locator(".ui-toast")).toHaveCount(4);
    await expect(stage.getByText("Report exported")).toBeVisible();
    await expect(stage.getByText("Sync started")).toBeVisible();
    await expect(stage.getByText("Quota close")).toBeVisible();
    await expect(stage.getByText("Export failed")).toBeVisible();
  });

  test("clicking a trigger appends a new toast that auto-dismisses", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Toast")') });
    const stage = preview.locator(".gallery-toast-stage");

    const baseline = await stage.locator(".ui-toast").count();
    await preview.getByRole("button", { name: "Trigger success" }).click();
    await expect(stage.locator(".ui-toast")).toHaveCount(baseline + 1);

    // Newly triggered toasts auto-dismiss after ~3s; pre-seeded ones persist.
    await expect(stage.locator(".ui-toast")).toHaveCount(baseline, {
      timeout: 4_000,
    });
  });
});
