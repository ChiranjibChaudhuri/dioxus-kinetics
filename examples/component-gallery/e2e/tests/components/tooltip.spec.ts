import { test, expect } from "@playwright/test";
import { mountGallery } from "../_lib/mount.js";

test.describe("Tooltip", () => {
  test("always-visible showcase tile renders a permanent tooltip", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Tooltip")') })
      .locator(".gallery-preview--ready");

    // The first tile demonstrates the open state directly via `visible: true`.
    const showcase = preview.getByRole("tooltip").first();
    await expect(showcase).toBeVisible();
    await expect(showcase).toContainText("Compared to the 30-day rolling average.");
  });

  test("hover-driven tile surfaces the tooltip on hover and hides on leave", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Tooltip")') })
      .locator(".gallery-preview--ready");

    // Scope to the hover tile so the always-visible showcase doesn't interfere.
    const hoverTile = preview
      .locator(".gallery-variant-tile")
      .filter({ has: page.locator("text=Hover / focus") });
    const trigger = hoverTile.getByText("Net revenue").first();
    const hoverTooltip = hoverTile.getByRole("tooltip");

    await expect(hoverTooltip).toHaveCount(0);
    await trigger.hover();
    await expect(hoverTooltip).toBeVisible();

    await page.mouse.move(0, 0);
    await expect(hoverTooltip).toHaveCount(0);
  });

  test("focus surfaces the hover-driven tooltip for keyboard users", async ({
    page,
  }) => {
    await mountGallery(page, { variant: "default", hash: "feedback" });
    const preview = page
      .locator("article.gallery-entry")
      .filter({ has: page.locator('h4:text-is("Tooltip")') });

    // Dispatch focusin on the hover tile to deterministically exercise the
    // keyboard-accessibility contract: focus surfaces the tooltip.
    const hoverTile = preview
      .locator(".gallery-variant-tile")
      .filter({ has: page.locator("text=Hover / focus") });
    await hoverTile.evaluate((el) => {
      el.dispatchEvent(new FocusEvent("focusin", { bubbles: true }));
    });
    await expect(hoverTile.getByRole("tooltip")).toBeVisible();
  });
});
