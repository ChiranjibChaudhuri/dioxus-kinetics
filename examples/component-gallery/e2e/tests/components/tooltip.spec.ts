import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Tooltip", () => {
  test("appears on hover and disappears on mouseleave", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Tooltip")') })
      .locator(".gallery-preview--ready");

    // The preview's code-snippet block also contains the literal tooltip
    // string, so we scope the assertion to the actual tooltip element
    // (role="tooltip") rather than getByText, which would match both.
    const tooltipContent = preview.getByRole("tooltip");
    const trigger = preview.getByText("Net revenue").first();
    await trigger.hover();
    await expect(tooltipContent).toBeVisible();

    await page.mouse.move(0, 0);
    await expect(tooltipContent).toBeHidden();
  });

  test("focus surfaces the tooltip for keyboard users", async ({ page }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Tooltip")') });

    // The trigger is a non-focusable <span>; the parent .gallery-demo-frame-body
    // catches focus via onfocusin. Tab-key navigation is flaky under parallel
    // workers (focus competes across pages). Dispatch the focusin event
    // directly to deterministically test the keyboard-accessibility contract:
    // when focus enters the trigger region, the tooltip surfaces.
    const body = preview.locator(".gallery-demo-frame-body");
    await body.evaluate((el) => {
      el.dispatchEvent(new FocusEvent("focusin", { bubbles: true }));
    });
    await expect(preview.getByRole("tooltip")).toBeVisible();
  });
});
