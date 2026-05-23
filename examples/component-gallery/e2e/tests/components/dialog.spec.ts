import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Dialog", () => {
  test("opens on trigger click and closes on action", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Dialog")') })
      .locator(".gallery-preview--ready");

    // Starts closed: the dialog title element only exists when open.
    await expect(preview.locator("#ui-dialog-title")).toHaveCount(0);

    await preview.getByRole("button", { name: "Show dialog" }).click();
    await expect(preview.locator("#ui-dialog-title")).toBeVisible();
    await expect(preview.getByText("Archive workspace")).toBeVisible();
    await expect(preview.getByText("Move this workspace out of active navigation.")).toBeVisible();
    await expect(preview.getByText("Team members can still request access later.")).toBeVisible();

    // Cancel dismisses.
    await preview.getByRole("button", { name: "Cancel" }).click();
    await expect(preview.locator("#ui-dialog-title")).toHaveCount(0);
  });

  test("reduced motion variant still opens the dialog", async ({ page }) => {
    await mountGallery(page, { variant: "reduced-motion", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Dialog")') });
    await preview.getByRole("button", { name: "Show dialog" }).click();
    await expect(preview.locator("#ui-dialog-title")).toBeVisible();
  });
});
