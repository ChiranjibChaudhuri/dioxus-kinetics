import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Dialog", () => {
  test("opens by default with reopen control; cancel dismisses, reopen re-opens", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Dialog")') })
      .locator(".gallery-preview--ready");

    // Showcase starts open by default.
    await expect(preview.locator("#ui-dialog-title")).toBeVisible();
    await expect(preview.getByText("Archive workspace")).toBeVisible();
    await expect(preview.getByText("Move this workspace out of active navigation.")).toBeVisible();
    await expect(preview.getByText("Team members can still request access later.")).toBeVisible();

    // Cancel dismisses. Webkit sometimes intercepts the click through the
    // modal backdrop; force-click bypasses the actionability check that the
    // overlay would otherwise stall on.
    await preview.getByRole("button", { name: "Cancel" }).click({ force: true });
    await expect(preview.locator("#ui-dialog-title")).toHaveCount(0);

    // Reopen brings it back.
    await preview.getByRole("button", { name: "Reopen" }).click();
    await expect(preview.locator("#ui-dialog-title")).toBeVisible();
  });

  test("reduced motion variant still renders the dialog open", async ({ page }) => {
    await mountGallery(page, { variant: "reduced-motion", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Dialog")') });
    await expect(preview.locator("#ui-dialog-title")).toBeVisible();
  });
});
