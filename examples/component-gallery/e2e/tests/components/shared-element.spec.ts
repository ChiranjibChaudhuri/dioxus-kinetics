import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("SharedElement", () => {
  test("the hero element repositions when layouts swap", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "composition" });
    const entry = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("SharedElement")') });
    const frame = entry.locator(".gallery-demo-frame--flip");
    await expect(frame).toBeVisible();

    const hero = frame.locator('[data-shared-id="demo-hero"]');
    await expect(hero).toBeVisible();
    const boxA = await hero.boundingBox();
    expect(boxA).toBeTruthy();

    await frame.getByRole("button", { name: "Swap layout" }).click();
    await page.waitForTimeout(500);
    const boxB = await hero.boundingBox();
    expect(boxB).toBeTruthy();
    expect(Math.abs((boxA!.x) - (boxB!.x))).toBeGreaterThan(20);
  });
});
