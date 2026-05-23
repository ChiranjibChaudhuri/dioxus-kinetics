import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Tooltip", () => {
  test("appears on hover and disappears on mouseleave", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Tooltip")') })
      .locator(".gallery-preview--ready");

    const trigger = preview.getByText("Net revenue");
    await trigger.hover();
    await expect(preview.getByText("Revenue after refunds and credits.")).toBeVisible();

    await page.mouse.move(0, 0);
    await expect(preview.getByText("Revenue after refunds and credits.")).toBeHidden();
  });

  test("focus surfaces the tooltip for keyboard users", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Tooltip")') });
    await preview.locator(".gallery-demo-frame-body").click({ position: { x: 0, y: 0 } });
    await page.keyboard.press("Tab");
    await expect(preview.getByText("Revenue after refunds and credits.")).toBeVisible();
  });
});
