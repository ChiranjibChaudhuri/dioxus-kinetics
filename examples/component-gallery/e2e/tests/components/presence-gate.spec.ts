import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("PresenceGate", () => {
  test("present tile renders children; hidden tile gates them out", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "motion" });
    const entry = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("PresenceGate")') });

    const presentTile = entry
      .locator(".gallery-variant-tile")
      .filter({ has: page.locator('text="Present"') });
    const hiddenTile = entry
      .locator(".gallery-variant-tile")
      .filter({ has: page.locator('text="Hidden"') });

    await expect(presentTile.getByText("Visible state")).toBeVisible();
    await expect(hiddenTile.getByText("(gate suppresses children)")).toBeVisible();
  });
});
