import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("SharedLayout", () => {
  test("swapping layouts repositions the two shared cards", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "composition" });
    const entry = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("SharedLayout")') });
    const frame = entry.locator(".gallery-demo-frame--flip");
    await expect(frame).toBeVisible();

    const left = frame.locator('[data-shared-id="card-left"]');
    const right = frame.locator('[data-shared-id="card-right"]');
    const leftBoxA = await left.boundingBox();
    const rightBoxA = await right.boundingBox();
    expect(leftBoxA).toBeTruthy();
    expect(rightBoxA).toBeTruthy();

    await frame.getByRole("button", { name: "Swap layout" }).click();
    await page.waitForTimeout(500);
    const leftBoxB = await left.boundingBox();
    const rightBoxB = await right.boundingBox();
    expect(leftBoxB).toBeTruthy();
    expect(rightBoxB).toBeTruthy();

    expect(Math.abs(leftBoxB!.x - rightBoxA!.x)).toBeLessThan(40);
  });
});
